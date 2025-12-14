use crate::server::models::repository_errors::{RepositoryError, map_sqlx_error};
use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum PatchResult<T> {
    Updated(T),
    NotFound,
    NotModified,
}
pub struct RsaKeyRepository {
    pool: PgPool,
}

impl RsaKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, key_size: i32) -> Result<RSAKeyAlgorithm, RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;

        let rsa_key: RSAKeyAlgorithm = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
            INSERT INTO rsa_key_algorithm (algorithm, key_size)
            VALUES ('RSA', $1)
            RETURNING *;
        "#,
        )
        .bind(key_size)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;

        Ok(rsa_key)
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
    pub async fn find_all(&self) -> Result<Vec<RSAKeyAlgorithm>, RepositoryError> {
        let results = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
            SELECT * FROM rsa_key_algorithm ORDER BY key_size ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(results)
    }
    pub async fn patch(
        &self,
        id: Uuid,
        deprecated: bool,
    ) -> Result<PatchResult<RSAKeyAlgorithm>, RepositoryError> {
        let updated = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
                UPDATE rsa_key_algorithm
                SET deprecated = $1
                WHERE id = $2 AND deprecated <> $1
                RETURNING *
            "#,
        )
        .bind(deprecated)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        match updated {
            Some(model) => Ok(PatchResult::Updated(model)),
            None => {
                // Either not found or not modified
                match self.find_by_id(id).await? {
                    Some(_) => Ok(PatchResult::NotModified),
                    None => Ok(PatchResult::NotFound),
                }
            }
        }
    }
    pub async fn delete(&self, id: Uuid) -> Result<Option<RSAKeyAlgorithm>, RepositoryError> {
        let deleted = sqlx::query_as::<_, RSAKeyAlgorithm>(
            r#"
                DELETE FROM rsa_key_algorithm
                WHERE id = $1 
                RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(deleted)
    }
}
