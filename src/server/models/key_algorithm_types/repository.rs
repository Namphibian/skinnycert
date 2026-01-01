use crate::server::models::key_algorithm_types::db::KeyAlgorithmTypeInfo;
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
use sqlx::PgPool;

#[derive(Debug)]
pub struct KeyAlgorithmTypeRepository {
    pool: PgPool,
}

impl KeyAlgorithmTypeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn find_all(&self) -> Result<Vec<KeyAlgorithmTypeInfo>, RepositoryError> {
        let results =
            sqlx::query_as::<_, KeyAlgorithmTypeInfo>("SELECT * FROM key_algorithm_type_info")
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx_error)?;
        Ok(results)
    }
}
