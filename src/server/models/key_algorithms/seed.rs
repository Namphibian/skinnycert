use openssl::nid::Nid;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::BTreeMap;
use std::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Debug)]
pub struct EcBuiltinCurve {
    nid: c_int,
    comment: *const c_char,
}

unsafe extern "C" {
    fn EC_get_builtin_curves(r: *mut EcBuiltinCurve, n: c_int) -> c_int;
}

/// Extract builtin ECDSA curves from OpenSSL
#[tracing::instrument(name = "START UP - Get OpenSSL Builtin Curves",level = tracing::Level::DEBUG)]
pub fn builtin_curves() -> Vec<(Nid, String)> {
    unsafe {
        let count = EC_get_builtin_curves(std::ptr::null_mut(), 0);
        let mut curves: Vec<EcBuiltinCurve> = Vec::with_capacity(count as usize);

        let got = EC_get_builtin_curves(curves.as_mut_ptr(), count);
        assert_eq!(got, count);

        curves.set_len(count as usize);

        curves
            .into_iter()
            .map(|c| {
                let nid = Nid::from_raw(c.nid);
                let comment = if c.comment.is_null() {
                    "".to_string()
                } else {
                    std::ffi::CStr::from_ptr(c.comment)
                        .to_string_lossy()
                        .into_owned()
                };
                (nid, comment)
            })
            .collect()
    }
}

#[tracing::instrument(name = "START UP - Extract Curve Size To Key Strength",level = tracing::Level::DEBUG)]
fn extract_curve_size(comment: &str) -> Option<i32> {
    let re = regex::Regex::new(r"(\d+)\s*[- ]*bit").unwrap();
    re.captures(comment)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<i32>().ok())
}

/// Load all key_algorithm_statuses into a BTreeMap<&'static str, Uuid>
#[tracing::instrument(name = "START UP - Load Key Algorithm Statuses",level = tracing::Level::DEBUG)]
async fn load_key_algorithm_statuses(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<BTreeMap<&'static str, uuid::Uuid>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT name, id
        FROM key_algorithm_statuses
        "#
    )
    .fetch_all(&mut **tx)
    .await?;

    let mut map = BTreeMap::new();

    for row in rows {
        let key = match row.name.as_str() {
            "tls_secure" => "TLS_SECURE",
            "tls_insecure" => "TLS_INSECURE",
            "internal_only" => "INTERNAL_ONLY",
            "deprecated" => "DEPRECATED",
            "forbidden" => "FORBIDDEN",
            "experimental" => "EXPERIMENTAL",
            other => panic!("Unknown key_algorithm_status: {}", other),
        };

        map.insert(key, row.id);
    }

    Ok(map)
}

/// Seed TLS statuses (idempotent)
#[tracing::instrument(name = "START UP - Seed TLS Statuses",level = tracing::Level::DEBUG)]
async fn seed_tls_statuses(tx: &mut Transaction<'_, Postgres>) -> Result<(), sqlx::Error> {
    let statuses = [
        ("supported", "Usable for browser-trusted TLS certificates"),
        ("not_supported", "Not usable for TLS certificates"),
        ("experimental", "Future or internal-only algorithms"),
    ];

    for (name, desc) in statuses {
        sqlx::query!(
            r#"
            INSERT INTO key_algorithm_type_tls_statuses (name, description)
            VALUES ($1, $2)
            ON CONFLICT (name) DO NOTHING
            "#,
            name,
            desc
        )
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

#[tracing::instrument(name = "START UP - Get tls status id for mapping ",level = tracing::Level::DEBUG)]
async fn get_tls_status_id(
    tx: &mut Transaction<'_, Postgres>,
    name: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    sqlx::query_scalar!(
        r#"SELECT id FROM key_algorithm_type_tls_statuses WHERE name = $1"#,
        name
    )
    .fetch_one(&mut **tx)
    .await
}

struct AlgorithmTypeIds {
    rsa: uuid::Uuid,
    ecdsa: uuid::Uuid,
    ed25519: uuid::Uuid,
    x25519: uuid::Uuid,
}

/// Seed algorithm_types (idempotent)
#[tracing::instrument(name = "START UP - Seed algorithm types",level = tracing::Level::DEBUG)]
async fn seed_algorithm_types(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<AlgorithmTypeIds, sqlx::Error> {
    seed_tls_statuses(tx).await?;

    let supported = get_tls_status_id(tx, "supported").await?;
    let not_supported = get_tls_status_id(tx, "not_supported").await?;

    let rows = [
        ("RSA", "Rivest–Shamir–Adleman", false, true, supported),
        (
            "ECDSA",
            "Elliptic Curve Digital Signature Algorithm",
            true,
            true,
            supported,
        ),
        (
            "Ed25519",
            "Edwards-curve Digital Signature Algorithm",
            false,
            false,
            not_supported,
        ),
        (
            "X25519",
            "Montgomery curve Diffie–Hellman key exchange",
            false,
            false,
            not_supported,
        ),
    ];

    for (name, desc, requires_nid, requires_strength, tls_status_id) in rows {
        sqlx::query!(
            r#"
            INSERT INTO key_algorithm_types (name, description, requires_nid, requires_strength, tls_status_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (name) DO NOTHING
            "#,
            name,
            desc,
            requires_nid,
            requires_strength,
            tls_status_id
        )
            .execute(&mut  **tx)
            .await?;
    }

    Ok(AlgorithmTypeIds {
        rsa: sqlx::query_scalar!(r#"SELECT id FROM key_algorithm_types WHERE name = 'RSA'"#)
            .fetch_one(&mut **tx)
            .await?,
        ecdsa: sqlx::query_scalar!(r#"SELECT id FROM key_algorithm_types WHERE name = 'ECDSA'"#)
            .fetch_one(&mut **tx)
            .await?,
        ed25519: sqlx::query_scalar!(
            r#"SELECT id FROM key_algorithm_types WHERE name = 'Ed25519'"#
        )
        .fetch_one(&mut **tx)
        .await?,
        x25519: sqlx::query_scalar!(r#"SELECT id FROM key_algorithm_types WHERE name = 'X25519'"#)
            .fetch_one(&mut **tx)
            .await?,
    })
}

/// Seed RSA key algorithms
#[tracing::instrument(name = "START UP - Seed concrete RSA keys",level = tracing::Level::DEBUG)]
pub async fn seed_rsa_key_algorithms(
    tx: &mut Transaction<'_, Postgres>,
    rsa_type_id: uuid::Uuid,
    statuses: &BTreeMap<&'static str, uuid::Uuid>,
    rsa_min_supported_size: u32,
    rsa_max_supported_size: u32,
) -> Result<(), sqlx::Error> {
    let tls_secure = statuses["TLS_SECURE"];

    //
    // 1. VALIDATION
    //
    if rsa_min_supported_size < 2048 {
        return Err(sqlx::Error::Protocol(
            "rsa_min_supported_size must be >= 2048".into(),
        ));
    }
    if rsa_max_supported_size < rsa_min_supported_size {
        return Err(sqlx::Error::Protocol(
            "rsa_max_supported_size must be >= rsa_min_supported_size".into(),
        ));
    }
    if rsa_min_supported_size % 1024 != 0 || rsa_max_supported_size % 1024 != 0 {
        return Err(sqlx::Error::Protocol(
            "RSA sizes must be multiples of 1024".into(),
        ));
    }

    //
    // 2. CLEANUP OF OVERSIZED ENTRIES
    //
    // Fetch all RSA key sizes above env max
    let oversized = sqlx::query!(
        r#"
        SELECT id, key_strength
        FROM key_algorithms
        WHERE algorithm_type_id = $1
          AND key_strength > $2
        "#,
        rsa_type_id,
        rsa_max_supported_size as i32
    )
        .fetch_all(&mut **tx)
        .await?;

    if !oversized.is_empty() {
        let oversized_ids: Vec<uuid::Uuid> = oversized.iter().map(|r| r.id).collect();

        // Check which oversized entries are referenced by certificates
        let referenced = sqlx::query!(
            r#"
            SELECT DISTINCT key_algorithm_id
            FROM certificates
            WHERE key_algorithm_id = ANY($1)
            "#,
            &oversized_ids
        )
            .fetch_all(&mut **tx)
            .await?;

        let referenced_ids: std::collections::HashSet<uuid::Uuid> =
            referenced.iter().map(|r| r.key_algorithm_id).collect();

        // Log warning if any oversized entries are referenced
        if !referenced_ids.is_empty() {
            let max_db = oversized
                .iter()
                .map(|r| r.key_strength.unwrap_or(0))
                .max()
                .unwrap_or(0);

            tracing::warn!(
                "RSA max size reduced to {}, but DB contains larger sizes (up to {}). \
                 Some of these are referenced by certificates and will be kept. \
                 Ignoring env max for those sizes.",
                rsa_max_supported_size,
                max_db
            );
        }

        // Delete only unreferenced oversized entries
        let deletable_ids: Vec<uuid::Uuid> = oversized_ids
            .into_iter()
            .filter(|id| !referenced_ids.contains(id))
            .collect();

        if !deletable_ids.is_empty() {
            sqlx::query!(
                r#"
                DELETE FROM key_algorithms
                WHERE id = ANY($1)
                "#,
                &deletable_ids
            )
                .execute(&mut **tx)
                .await?;

            tracing::info!(
                "Deleted {} unused RSA key sizes above configured max {}",
                deletable_ids.len(),
                rsa_max_supported_size
            );
        }
    }

    //
    // 3. INSERT NEW RSA SIZES UP TO ENV MAX
    //
    let mut sizes = Vec::new();
    let mut current = rsa_min_supported_size;
    while current <= rsa_max_supported_size {
        sizes.push(current);
        current += 1024;
    }

    for size in sizes {
        let display_name = format!("RSA {}-bit", size);

        sqlx::query!(
            r#"
            INSERT INTO key_algorithms (algorithm_type_id, status_id, key_strength, nid_value, display_name)
            VALUES ($1, $2, $3, NULL, $4)
            ON CONFLICT DO NOTHING
            "#,
            rsa_type_id,
            tls_secure,
            size as i32,
            display_name
        )
            .execute(&mut **tx)
            .await?;
    }

    Ok(())
}


/// Seed ECDSA curves
#[tracing::instrument(name = "START UP - Seed concrete ECDSA keys",level = tracing::Level::DEBUG)]
async fn seed_ecdsa_key_algorithms(
    tx: &mut Transaction<'_, Postgres>,
    ecdsa_type_id: uuid::Uuid,
    statuses: &BTreeMap<&'static str, uuid::Uuid>,
) -> Result<(), sqlx::Error> {
    let tls_secure = statuses["TLS_SECURE"];
    let tls_insecure = statuses["TLS_INSECURE"];
    let deprecated = statuses["DEPRECATED"];
    let forbidden = statuses["FORBIDDEN"];

    for (nid, comment) in builtin_curves() {
        let nid_value = nid.as_raw();
        let key_strength = extract_curve_size(&comment).unwrap_or(0);

        let status_id = match key_strength {
            256 | 384 | 521 => tls_secure,
            224 => tls_insecure,
            192 => deprecated,
            _ => forbidden,
        };

        let display_name = if comment.is_empty() {
            format!("{:?}", nid)
        } else {
            comment.clone()
        };

        sqlx::query!(
            r#"
            INSERT INTO key_algorithms (algorithm_type_id, status_id, key_strength, nid_value, display_name)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
            "#,
            ecdsa_type_id,
            status_id,
            key_strength,
            nid_value,
            display_name
        )
            .execute(&mut  **tx)
            .await?;
    }

    Ok(())
}

/// Seed Ed25519
async fn seed_ed25519_key_algorithm(
    tx: &mut Transaction<'_, Postgres>,
    ed25519_type_id: uuid::Uuid,
    statuses: &BTreeMap<&'static str, uuid::Uuid>,
) -> Result<(), sqlx::Error> {
    let tls_secure = statuses["EXPERIMENTAL"];

    sqlx::query!(
        r#"
        INSERT INTO key_algorithms (algorithm_type_id, status_id, key_strength, nid_value, display_name)
        VALUES ($1, $2, NULL, NULL, 'Ed25519')
        ON CONFLICT DO NOTHING
        "#,
        ed25519_type_id,
        tls_secure
    )
        .execute(&mut  **tx)
        .await?;

    Ok(())
}

/// Seed X25519
async fn seed_x25519_key_algorithm(
    tx: &mut Transaction<'_, Postgres>,
    x25519_type_id: uuid::Uuid,
    statuses: &BTreeMap<&'static str, uuid::Uuid>,
) -> Result<(), sqlx::Error> {
    let tls_secure = statuses["FORBIDDEN"];

    sqlx::query!(
        r#"
        INSERT INTO key_algorithms (algorithm_type_id, status_id, key_strength, nid_value, display_name)
        VALUES ($1, $2, NULL, NULL, 'X25519')
        ON CONFLICT DO NOTHING
        "#,
        x25519_type_id,
        tls_secure
    )
        .execute(&mut  **tx)
        .await?;

    Ok(())
}

/// Public entry point
#[tracing::instrument(name = "START UP - Seed all key data based on configured and OpenSSL supported keys",level = tracing::Level::DEBUG)]
pub async fn seed_all_algorithms(
    pool: &PgPool,
    rsa_key_min_supported_size: u32,
    rsa_key_max_supported_size: u32,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let ids = seed_algorithm_types(&mut tx).await?;

    // Load all statuses into a BTreeMap<&'static str, Uuid>
    let statuses = load_key_algorithm_statuses(&mut tx).await?;

    seed_rsa_key_algorithms(
        &mut tx,
        ids.rsa,
        &statuses,
        rsa_key_min_supported_size,
        rsa_key_max_supported_size,
    )
    .await?;
    seed_ecdsa_key_algorithms(&mut tx, ids.ecdsa, &statuses).await?;
    seed_ed25519_key_algorithm(&mut tx, ids.ed25519, &statuses).await?;
    seed_x25519_key_algorithm(&mut tx, ids.x25519, &statuses).await?;
    tx.commit().await?;
    Ok(())
}
