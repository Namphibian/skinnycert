//! This module provides the `PatchResult` enum and the `RsaKeyRepository` struct for interacting
//! with a PostgreSQL database to manage RSA key algorithms. It includes functionality such as
//! creating, finding, updating, and deleting RSA key entries. Additionally, it provides an enum
//! to indicate the outcome of patch operations.
use crate::server::models::repository_errors::{RepositoryError, map_sqlx_error};
use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;
use sqlx::PgPool;
use uuid::Uuid;

/// Represents the result of a patch operation, indicating whether an item was updated,
/// not found, or not modified.
///
/// # Variants
///
/// - `Updated(T)`:
///   The patch operation was successful and resulted in an updated value of type `T`.
///
/// - `NotFound`:
///   The patch operation could not proceed because the target item was not found.
///
/// - `NotModified`:
///   The patch operation was unnecessary as the target item was already up-to-date.
///
/// # Type Parameters
///
/// - `T`: The type of the updated item contained in the `Updated` variant.
///
/// # Examples
///
/// ```
/// use your_crate::PatchResult;
///
/// let result: PatchResult<String> = PatchResult::Updated(String::from("UpdatedValue"));
///
/// match result {
///     PatchResult::Updated(value) => println!("Successfully updated: {}", value),
///     PatchResult::NotFound => println!("Item not found."),
///     PatchResult::NotModified => println!("No modification needed."),
/// }
/// ```
#[derive(Debug)]
pub enum PatchResult<T> {
    Updated(T),
    NotFound,
    NotModified,
}
/// A repository for handling RSA keys, backed by a PostgreSQL database.
///
/// The `RsaKeyRepository` is responsible for managing storage and retrieval
/// of RSA keys using a PostgreSQL connection pool.
///
/// # Fields
/// - `pool`: `PgPool`
///   - A connection pool to the PostgreSQL database that enables efficient
///     execution of queries for RSA key operations.
///
/// # Example
/// ```rust
/// use sqlx::PgPool;
/// use your_crate::RsaKeyRepository;
///
/// #[tokio::main]
/// async fn main() -> Result<(), sqlx::Error> {
///     let database_url = "postgres://user:password@localhost/dbname";
///     let pool = PgPool::connect(database_url).await?;
///
///     let rsa_key_repo = RsaKeyRepository { pool };
///
///     // Use `rsa_key_repo` for operations with RSA keys...
///
///     Ok(())
/// }
/// ```
pub struct RsaKeyRepository {
    pool: PgPool,
}

impl RsaKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new RSA key algorithm entry in the database with the specified key size.
    ///
    /// # Arguments
    ///
    /// * `key_size` - An integer specifying the size of the RSA key to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(RSAKeyAlgorithm)` - If the operation is successful, returns the created `RSAKeyAlgorithm` struct,
    ///   which contains the information about the RSA key algorithm entry.
    /// * `Err(RepositoryError)` - If there is an error during the database operation, returns a `RepositoryError`.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * If there is an issue starting or committing the database transaction.
    /// * If there is a failure executing the SQL query.
    /// * If the `key_size` does not meet the database constraints.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = rsa_key_repository.create(2048).await;
    /// match result {
    ///     Ok(rsa_key) => println!("Created RSA key with ID: {}", rsa_key.id),
    ///     Err(e) => eprintln!("Failed to create RSA key: {:?}", e),
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// This function uses a database connection pool to perform the operation asynchronously.
    /// The transaction is committed only after successfully creating the RSA key entry. If any error occurs,
    /// the transaction is rolled back automatically.
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
