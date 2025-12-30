use crate::server::models::key_algorithms::db::KeyAlgorithmInfo;
use crate::server::models::responses::{RepositoryError, map_sqlx_error};
use crate::server::models::rsa_key::db::RSAKeyAlgorithm;
use sqlx::PgPool;

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
                algorithm_type_name,
                tls_status_name,
                status_name,
                key_algorithm_strength
                ASC;
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(results)
    }
}
