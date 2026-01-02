use crate::server::models::certificates::db::CertificateInfo;
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
use sqlx::PgPool;
use uuid::Uuid;

pub struct CertificateRepository {
    pool: PgPool,
}

impl CertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
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
    pub async fn find_all(&self) -> Result<Vec<CertificateInfo>, RepositoryError> {
        let results = sqlx::query_as::<_, CertificateInfo>(
            r#"
            SELECT * FROM certificate_info
            ORDER BY created_on DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(results)
    }
}
