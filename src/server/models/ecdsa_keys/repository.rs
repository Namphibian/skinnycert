use crate::server::models::ecdsa_keys::db::EcdsaKeyAlgorithm;
use crate::server::models::repository_errors::{RepositoryError, map_sqlx_error};
use sqlx::PgPool;
use uuid::Uuid;

use crate::server::models::rsa_keys::repository::{PatchResult, RsaKeyRepository};

pub struct EcdsaKeyRepository {
    pool: PgPool,
}

impl EcdsaKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create(
        &self,
        display_name: &str,
        curve: i32,
        nid_name: &str,
        nid_value: i32,
        standard: &str,
    ) -> Result<EcdsaKeyAlgorithm, RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;

        let ecdsa_key: EcdsaKeyAlgorithm = sqlx::query_as::<_, EcdsaKeyAlgorithm>(
            r#"
            INSERT INTO ecdsa_key_algorithm
            (
                algorithm,
                display_name,
                curve,
                nid_name,
                nid_value,
                standard
            )
            VALUES
            (
                'ECDSA',
                $1,
                $2,
                $3,
                $4,
                $5
            );
        "#,
        )
        .bind(display_name)
        .bind(curve)
        .bind(nid_name)
        .bind(nid_value)
        .bind(standard)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;

        Ok(ecdsa_key)
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
            ORDER BY key_size ASC
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
