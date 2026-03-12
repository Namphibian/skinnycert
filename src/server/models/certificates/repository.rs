use crate::server::models::certificates::db::CertificateInfo;
use crate::server::models::certificates::filters::CertificateFilterParams;
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
use crate::server::models::shared::{decode_cursor, encode_cursor, PageDirection, PagedResult};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use urlencoding::decode as url_decode;
use uuid::Uuid;

#[derive(Debug)]
pub struct CertificateRepository {
    pool: PgPool,
}

impl CertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "DB Read Certificate By ID", level = tracing::Level::DEBUG)]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<CertificateInfo>, RepositoryError> {
        let result = sqlx::query_as::<_, CertificateInfo>(
            r#"
            SELECT * FROM certificate_info
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result)
    }
    pub async fn create(
        &self,
        csr_pem: &str,
        key_pem: &str,
        public_key_pem: &str,
        key_algorithm_id: Uuid,
        organization: Option<&str>,
        organizational_unit: Option<&str>,
        country: Option<&str>,
        state_or_province: Option<&str>,
        locality: Option<&str>,
        email: Option<&str>,
        sans: &[String],
    ) -> Result<Uuid, RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;

        let cert_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO certificates (
                csr_pem, key_pem, public_key_pem, key_algorithm_id,
                organization, organizational_unit, country, state_or_province, locality, email
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#,
        )
        .bind(csr_pem)
        .bind(key_pem)
        .bind(public_key_pem)
        .bind(key_algorithm_id)
        .bind(organization)
        .bind(organizational_unit)
        .bind(country)
        .bind(state_or_province)
        .bind(locality)
        .bind(email)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

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
            .await
            .map_err(map_sqlx_error)?;
        }

        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(cert_id)
    }
    #[tracing::instrument(name = "DB Delete Certificate By ID", level = tracing::Level::DEBUG)]
    pub async fn delete_by_id(&self, id: Uuid) -> Result<Option<Uuid>, RepositoryError> {
        // If your DB supports RETURNING, fetch_optional returns Some(row) when deleted
        let row = sqlx::query!("DELETE FROM certificates WHERE id = $1 RETURNING id", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(row.map(|r| r.id))
    }

    #[tracing::instrument(name = "DB Read All Certificates (Paged)", level = tracing::Level::INFO)]
    pub async fn find_all_paged(
        &self,
        params: &CertificateFilterParams,
    ) -> Result<PagedResult<CertificateInfo>, RepositoryError> {
        // requested limit (default 100)
        let limit = params.limit.unwrap_or(100);

        // Decode cursor if present; on failure return an explicit repository error
        let (cursor_created_on, cursor_id, has_cursor): (DateTime<Utc>, Uuid, bool) =
            if let Some(token) = &params.page_token {
                // URL-decode first (clients may URL-encode tokens)
                match url_decode(token) {
                    Ok(decoded_token_cow) => {
                        let decoded_token = decoded_token_cow.into_owned();
                        match decode_cursor(&decoded_token) {
                            Ok((ts, id)) => (ts, id, true),
                            Err(e) => {
                                tracing::warn!(%token, "Invalid page token: {:?}", e);
                                // Map any cursor decode failure to a RepositoryError variant
                                // The responses layer maps RepositoryError::InvalidToken / InvalidTimestamp / InvalidUuid
                                // to a 400 Bad Request. Here we return InvalidToken for all decode failures.
                                return Err(RepositoryError::InvalidToken);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!(%token, "Failed to URL-decode page token: {:?}", e);
                        return Err(RepositoryError::InvalidToken);
                    }
                }
            } else {
                (DateTime::<Utc>::MAX_UTC, Uuid::nil(), false)
            };

        // Fetch limit + 1 rows to robustly detect "has more" (next page)
        let fetch_limit = limit.saturating_add(1);

        let direction = params.direction.unwrap_or(PageDirection::Next);

        let results = match direction {
            PageDirection::Next => {
                // Fetch newest-first (DESC). Request limit+1 to detect if there are more items.
                let mut rows = sqlx::query_as::<_, CertificateInfo>(
                    r#"
                    SELECT *
                    FROM certificate_info
                    WHERE
                        ($1 IS NULL OR common_name ILIKE '%' || $1 || '%')
                        AND ($2 IS NULL OR EXISTS (
                            SELECT 1 FROM unnest(sans) AS s(san)
                            WHERE s.san ILIKE '%' || $2 || '%'
                        ))
                        AND ($3 IS NULL OR organization ILIKE '%' || $3 || '%')
                        AND ($4 IS NULL OR organizational_unit ILIKE '%' || $4 || '%')
                        AND ($5 IS NULL OR country = $5)
                        AND ($6 IS NULL OR state_or_province ILIKE '%' || $6 || '%')
                        AND ($7 IS NULL OR locality ILIKE '%' || $7 || '%')
                        AND ($8 IS NULL OR email ILIKE '%' || $8 || '%')
                        AND ($9 IS NULL OR algorithm_type_name = $9)
                        AND ($10 IS NULL OR key_algorithm_display_name = $10)
                        AND ($11 IS NULL OR key_algorithm_key_strength = $11)
                        AND ($12 IS NULL OR key_algorithm_nid_value = $12)
                        AND ($13 IS NULL OR tls_status_name = $13)
                        AND ($14 IS NULL OR status_name = $14)
                        AND ($15 IS NULL OR is_signed = $15)
                        AND ($16 IS NULL OR is_expired = $16)
                        AND ($17 IS NULL OR created_on >= $17)
                        AND ($18 IS NULL OR created_on <= $18)
                        AND ($19 IS NULL OR valid_to >= $19)
                        AND ($20 IS NULL OR valid_to <= $20)
                        AND (
                            $21 = false
                            OR created_on < $22
                            OR (created_on = $22 AND id < $23)
                        )
                    ORDER BY created_on DESC, id DESC
                    LIMIT COALESCE($24, 100)
                "#,
                )
                .bind(&params.common_name)
                .bind(&params.san)
                .bind(&params.organization)
                .bind(&params.organizational_unit)
                .bind(&params.country)
                .bind(&params.state_or_province)
                .bind(&params.locality)
                .bind(&params.email)
                .bind(&params.algorithm_type_name)
                .bind(&params.key_algorithm_display_name)
                .bind(params.key_algorithm_key_strength)
                .bind(params.key_algorithm_nid_value)
                .bind(&params.tls_status_name)
                .bind(&params.status_name)
                .bind(params.is_signed)
                .bind(params.is_expired)
                .bind(params.created_after)
                .bind(params.created_before)
                .bind(params.valid_to_after)
                .bind(params.valid_to_before)
                .bind(has_cursor)
                .bind(cursor_created_on)
                .bind(cursor_id)
                .bind(fetch_limit)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx_error)?;

                // If we fetched more than `limit`, drop the extra row (it only indicates "has more")
                if rows.len() as i64 > limit {
                    rows.pop(); // extra row is the last element (oldest) for DESC ordering
                }
                rows
            }

            PageDirection::Prev => {
                // For prev, invert the comparison and sort ASC, then reverse in memory.
                // Fetch limit+1 to detect if there are more items in the "prev" direction.
                let mut rows = sqlx::query_as::<_, CertificateInfo>(
                    r#"
                    SELECT *
                    FROM certificate_info
                    WHERE
                        ($1 IS NULL OR common_name ILIKE '%' || $1 || '%')
                        AND ($2 IS NULL OR EXISTS (
                            SELECT 1 FROM unnest(sans) AS s(san)
                            WHERE s.san ILIKE '%' || $2 || '%'
                        ))
                        AND ($3 IS NULL OR organization ILIKE '%' || $3 || '%')
                        AND ($4 IS NULL OR organizational_unit ILIKE '%' || $4 || '%')
                        AND ($5 IS NULL OR country = $5)
                        AND ($6 IS NULL OR state_or_province ILIKE '%' || $6 || '%')
                        AND ($7 IS NULL OR locality ILIKE '%' || $7 || '%')
                        AND ($8 IS NULL OR email ILIKE '%' || $8 || '%')
                        AND ($9 IS NULL OR algorithm_type_name = $9)
                        AND ($10 IS NULL OR key_algorithm_display_name = $10)
                        AND ($11 IS NULL OR key_algorithm_key_strength = $11)
                        AND ($12 IS NULL OR key_algorithm_nid_value = $12)
                        AND ($13 IS NULL OR tls_status_name = $13)
                        AND ($14 IS NULL OR status_name = $14)
                        AND ($15 IS NULL OR is_signed = $15)
                        AND ($16 IS NULL OR is_expired = $16)
                        AND ($17 IS NULL OR created_on >= $17)
                        AND ($18 IS NULL OR created_on <= $18)
                        AND ($19 IS NULL OR valid_to >= $19)
                        AND ($20 IS NULL OR valid_to <= $20)
                        AND (
                            $21 = false
                            OR created_on > $22
                            OR (created_on = $22 AND id > $23)
                        )
                    ORDER BY created_on ASC, id ASC
                    LIMIT COALESCE($24, 100)
                "#,
                )
                .bind(&params.common_name)
                .bind(&params.san)
                .bind(&params.organization)
                .bind(&params.organizational_unit)
                .bind(&params.country)
                .bind(&params.state_or_province)
                .bind(&params.locality)
                .bind(&params.email)
                .bind(&params.algorithm_type_name)
                .bind(&params.key_algorithm_display_name)
                .bind(params.key_algorithm_key_strength)
                .bind(params.key_algorithm_nid_value)
                .bind(&params.tls_status_name)
                .bind(&params.status_name)
                .bind(params.is_signed)
                .bind(params.is_expired)
                .bind(params.created_after)
                .bind(params.created_before)
                .bind(params.valid_to_after)
                .bind(params.valid_to_before)
                .bind(has_cursor)
                .bind(cursor_created_on)
                .bind(cursor_id)
                .bind(fetch_limit)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx_error)?;

                // Reverse to restore global newest‑first order
                rows.reverse();

                // If we fetched more than `limit`, drop the extra row (it will be the last element after reverse)
                if rows.len() as i64 > limit {
                    rows.pop();
                }

                rows
            }
        };

        // After trimming the extra row above, results.len() == limit indicates "may have more"
        let has_more = results.len() as i64 == limit;

        // next_page_token: only present if there may be more items after this page
        let next_page_token = if has_more {
            results
                .last()
                .map(|cert| encode_cursor(cert.created_on, cert.id))
        } else {
            None
        };

        // prev_page_token: only present if the request included a cursor (i.e., this page was produced from a cursor)
        let prev_page_token = if has_cursor {
            results
                .first()
                .map(|cert| encode_cursor(cert.created_on, cert.id))
        } else {
            None
        };

        Ok(PagedResult {
            items: results,
            next_page_token,
            prev_page_token,
            limit,
        })
    }
}
