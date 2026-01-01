use crate::server::models::ecdsa_key::db::EcdsaKeyAlgorithm;
use crate::server::models::responses::{map_sqlx_error, PatchResult, RepositoryError};
use sqlx::PgPool;
use uuid::Uuid;

pub struct EcdsaKeyRepository {
    pool: PgPool,
}

impl EcdsaKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<EcdsaKeyAlgorithm>, RepositoryError> {
        let result = sqlx::query_as::<_, EcdsaKeyAlgorithm>(
            r#"
            SELECT *
            FROM ecdsa_key_algorithm
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;
        Ok(result)
    }
    pub async fn find_all(&self) -> Result<Vec<EcdsaKeyAlgorithm>, RepositoryError> {
        let results = sqlx::query_as::<_, EcdsaKeyAlgorithm>(
            r#"
            SELECT *
            FROM ecdsa_key_algorithm
            ORDER BY curve_size ASC
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
    ) -> Result<PatchResult<EcdsaKeyAlgorithm>, RepositoryError> {
        let updated = sqlx::query_as::<_, EcdsaKeyAlgorithm>(
            r#"
                UPDATE rsa_key_algorithm
                SET deprecated = $1
                WHERE id = $2
                  AND deprecated <> $1
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
}
