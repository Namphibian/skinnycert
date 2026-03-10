use crate::server::models::key_algorithms::db::{KeyAlgorithmFilterParams, KeyAlgorithmInfo};
use crate::server::models::responses::{map_sqlx_error, RepositoryError};
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

    #[tracing::instrument(name = "DB Read All Key Algorithms", level = tracing::Level::DEBUG)]
    pub async fn find_all(
        &self,
        params: &KeyAlgorithmFilterParams,
    ) -> Result<Vec<KeyAlgorithmInfo>, RepositoryError> {
        let results = sqlx::query_as::<_, KeyAlgorithmInfo>(
            r#"
        SELECT *
        FROM key_algorithm_info
        WHERE
            ($1 IS NULL OR algorithm_type_name = $1)
            AND ($2 IS NULL OR tls_status_name = $2)
            AND ($3 IS NULL OR status_name = $3)
            AND ($4 IS NULL OR key_algorithm_strength = $4)
        ORDER BY
            key_algorithm_strength,
            algorithm_type_name,
            tls_status_name,
            status_name 
        "#,
        )
        .bind(&params.algorithm_type)
        .bind(&params.tls_status)
        .bind(&params.algorithm_status)
        .bind(params.strength)
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
