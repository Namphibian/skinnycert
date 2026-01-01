use chrono::Utc;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::server::models::legacy_certificates::certificates_model::{
    EcdsaCurve, KeyAlgorithm, KeyStrength, RsaKeySize,
};
use crate::server::models::legacy_certificates::db::DbCertificateWithSans;

pub struct CertificateRepository {
    pool: PgPool,
}

impl CertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert a new certificate and its SANs
    pub async fn create(
        &self,
        csr_pem: &str,
        key_pem: &str,
        public_key_pem: &str,
        key_algorithm: KeyAlgorithm,
        key_strength: KeyStrength,
        organization: Option<&str>,
        organizational_unit: Option<&str>,
        country: Option<&str>,
        state_or_province: Option<&str>,
        locality: Option<&str>,
        email: Option<&str>,
        sans: &[String],
    ) -> Result<Uuid, Box<dyn Error>> {
        let mut tx = self.pool.begin().await?;

        let key_algorithm_str = match key_algorithm {
            KeyAlgorithm::RSA => "RSA",
            KeyAlgorithm::ECDSA => "ECDSA",
        };

        let (rsa_key_size, ecdsa_curve) = match key_strength {
            KeyStrength::Rsa(size) => {
                let size_str = match size {
                    RsaKeySize::Bits2048 => "2048",
                    RsaKeySize::Bits3072 => "3072",
                    RsaKeySize::Bits4096 => "4096",
                };
                (Some(size_str), None)
            }
            KeyStrength::Ecdsa(curve) => {
                let curve_str = match curve {
                    EcdsaCurve::P256 => "P256",
                    EcdsaCurve::P384 => "P384",
                    EcdsaCurve::P521 => "P521",
                };
                (None, Some(curve_str))
            }
        };

        // Insert certificate
        let cert_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO legacy_certificates (
                csr_pem, key_pem, public_key_pem, key_algorithm, rsa_key_size, ecdsa_curve,
                organization, organizational_unit, country, state_or_province, locality, email
            )
            VALUES ($1, $2, $3, $4::key_algorithm, $5::rsa_key_size, $6::ecdsa_curve, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#
        )
            .bind(csr_pem)
            .bind(key_pem)
            .bind(public_key_pem)
            .bind(key_algorithm_str)
            .bind(rsa_key_size)
            .bind(ecdsa_curve)
            .bind(organization)
            .bind(organizational_unit)
            .bind(country)
            .bind(state_or_province)
            .bind(locality)
            .bind(email)
            .fetch_one(&mut *tx)
            .await?;

        // Insert SANs
        for (index, san) in sans.iter().enumerate() {
            sqlx::query(
                r#"
                INSERT INTO certificate_sans (certificate_id, san_value, san_order)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(cert_id)
            .bind(san)
            .bind(index as i32)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(cert_id)
    }

    /// Get certificate by ID with SANs
    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<DbCertificateWithSans>, Box<dyn Error>> {
        let result = sqlx::query_as::<_, DbCertificateWithSans>(
            r#"
            SELECT * FROM certificates_with_sans
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    /// Get all active legacy_certificates
    pub async fn find_all_active(&self) -> Result<Vec<DbCertificateWithSans>, Box<dyn Error>> {
        let results = sqlx::query_as::<_, DbCertificateWithSans>(
            r#"
            SELECT * FROM certificates_with_sans
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    /// Patch certificate with signed cert from CA (ONE-TIME operation)
    pub async fn patch_certificate(
        &self,
        id: Uuid,
        cert_pem: &str,
        chain_pem: Option<&str>,
        fingerprint: &str,
        valid_from: chrono::DateTime<Utc>,
        expires_at: chrono::DateTime<Utc>,
    ) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            UPDATE legacy_certificates
            SET cert_pem = $2,
                chain_pem = $3,
                fingerprint = $4,
                valid_from = $5,
                expires_at = $6,
                cert_uploaded_at = NOW()
            WHERE id = $1
              AND cert_pem IS NULL
              AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(cert_pem)
        .bind(chain_pem)
        .bind(fingerprint)
        .bind(valid_from)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Certificate not found, already patched, or deleted".into());
        }

        Ok(())
    }

    /// Soft delete a certificate
    pub async fn soft_delete(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query(
            r#"
            UPDATE legacy_certificates
            SET deleted_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Certificate not found or already deleted".into());
        }

        Ok(())
    }
}
