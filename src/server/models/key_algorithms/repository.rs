use crate::server::models::key_algorithms::db::KeyAlgorithmInfo;
use crate::server::models::responses::{RepositoryError, map_sqlx_error};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct KeyAlgorithmRepository {
    pool: PgPool,
}

impl KeyAlgorithmRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    #[tracing::instrument(name = "DB Read All Key Algorithms",level = tracing::Level::DEBUG)]
    pub async fn find_all(&self) -> Result<Vec<KeyAlgorithmInfo>, RepositoryError> {
        let results = sqlx::query_as::<_, KeyAlgorithmInfo>(
            r#"
            SELECT *
            FROM key_algorithm_info
            ORDER BY
                key_algorithm_strength,
                algorithm_type_name,
                tls_status_name,
                status_name
                ASC;
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(results)
    }
    #[tracing::instrument(name = "DB Read Key Algorithms By ID",level = tracing::Level::DEBUG)]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<KeyAlgorithmInfo>, RepositoryError> {
        let result = sqlx::query_as::<_, KeyAlgorithmInfo>(
            r#"
            SELECT *
            FROM key_algorithm_info
            WHERE key_algorithm_id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;
        Ok(result)
    }
}
