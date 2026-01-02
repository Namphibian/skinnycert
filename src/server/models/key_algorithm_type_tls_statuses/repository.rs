use crate::server::models::key_algorithm_type_tls_statuses::db::KeyAlgorithmTypeTlsStatus;
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
use sqlx::PgPool;

#[derive(Debug)]
pub struct KeyAlgorithmTypeTlsStatusRepository {
    pool: PgPool,
}

impl KeyAlgorithmTypeTlsStatusRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    #[tracing::instrument(name = "DB Read Key Algorithm Type TLS Statuses",level = tracing::Level::DEBUG)]
    pub async fn find_all(&self) -> Result<Vec<KeyAlgorithmTypeTlsStatus>, RepositoryError> {
        let results = sqlx::query_as::<_, KeyAlgorithmTypeTlsStatus>(
            "SELECT * FROM key_algorithm_type_tls_statuses",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(results)
    }
}
