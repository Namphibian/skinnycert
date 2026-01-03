use crate::server::models::certificates::db::{CertificateFilterParams, CertificateInfo};
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct CertificateRepository {
    pool: PgPool,
}

impl CertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    #[tracing::instrument(name = "DB Read Certificate By ID",level = tracing::Level::DEBUG)]
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
    #[tracing::instrument(name = "DB Read All Certificates", level = tracing::Level::INFO)]
    pub async fn find_all(
        &self,
        params: &CertificateFilterParams,
    ) -> Result<Vec<CertificateInfo>, RepositoryError> {
        let results = sqlx::query_as::<_, CertificateInfo>(
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
        ORDER BY created_on DESC
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
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(results)
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

        // Insert certificate
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
            .await
            .map_err(map_sqlx_error)?;
        }

        tx.commit().await.map_err(map_sqlx_error)?;

        Ok(cert_id)
    }
}
