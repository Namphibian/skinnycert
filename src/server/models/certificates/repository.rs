use crate::server::models::certificates::db::CertificateDetails;
use crate::server::models::responses::{RepositoryError, map_sqlx_error};
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

pub struct CertificateRepository {
    pool: PgPool,
}

impl CertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CertificateDetails>, RepositoryError> {
        let result = sqlx::query_as::<_, CertificateDetails>(
            r#"
            SELECT * FROM certificate_details
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(result)
    }
    pub async fn find_all(&self) -> Result<Vec<CertificateDetails>, Box<dyn Error>> {
        let results = sqlx::query_as::<_, CertificateDetails>(
            r#"
            SELECT * FROM certificate_details
            ORDER BY created_on DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}
