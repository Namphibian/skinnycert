use crate::server::models::key_algorithm_statuses::db::KeyAlgorithmStatus;
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
use sqlx::PgPool;

pub struct KeyAlgorithmStatusRepository {
    pool: PgPool,
}

impl KeyAlgorithmStatusRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_all(&self) -> Result<Vec<KeyAlgorithmStatus>, RepositoryError> {
        let results =
            sqlx::query_as::<_, KeyAlgorithmStatus>("SELECT * FROM key_algorithm_statuses")
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx_error)?;
        Ok(results)
    }
}
