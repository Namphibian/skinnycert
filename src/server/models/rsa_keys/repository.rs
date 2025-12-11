use crate::server::models::repository_errors::{RepositoryError, map_sqlx_error};
use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

pub struct RsaKeyRepository {
    pool: PgPool,
}

impl RsaKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, key_size: i32) -> Result<Option<RSAKeyAlgorithm>, RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;

        let rsa_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO rsa_key_algorithm (algorithm, key_size)
            VALUES ('RSA', $1)
            RETURNING id;
        "#,
        )
        .bind(key_size)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;

        self.find_by_id(rsa_id).await // already returns Option
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<RSAKeyAlgorithm>, RepositoryError> {
        let result = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
            SELECT * FROM rsa_key_algorithm WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(result)
    }
    pub async fn find_all(&self) -> Result<Vec<RSAKeyAlgorithm>, Box<dyn Error>> {
        let results = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
            SELECT * FROM rsa_key_algorithm ORDER BY key_size ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}
