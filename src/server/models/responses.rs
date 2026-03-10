use sqlx::postgres::PgDatabaseError;
use thiserror::Error;

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

/// An enumeration representing potential errors that can occur in a repository layer.
///
/// This enum encapsulates various types of database and SQL-related errors, providing
/// detailed information about specific issues encountered during data operations.
///
/// # Variants
///
/// - `UniqueViolation`
///   - Errors related to unique constraint violations on a specified constraint.
///   - Fields:
///     - `constraint` - The name of the unique constraint that was violated.
///
/// - `ForeignKeyViolation`
///   - Errors caused by foreign key constraint violations on a specified constraint.
///   - Fields:
///     - `constraint` - The name of the foreign key constraint that was violated.
///
/// - `NotNullViolation`
///   - Errors related to a violation of a NOT NULL constraint on a specific column.
///   - Fields:
///     - `column` - The name of the column for which the NOT NULL constraint was violated.
///
/// - `CheckViolation`
///   - Errors linked to check constraint violations on a specified constraint.
///   - Fields:
///     - `constraint` - The name of the check constraint that was violated.
///
/// - `StringTooLong`
///   - Errors occurring when the string length exceeds the maximum length for a column.
///   - Fields:
///     - `column` - The name of the column where the string was too long.
///
/// - `NumericOutOfRange`
///   - Errors involving numeric values that are out of range for the specified type.
///
/// - `InvalidDatetime`
///   - Errors arising from invalid datetime formats during parsing or validation.
///
/// - `SyntaxError`
///   - SQL syntax errors encountered in a query.
///
/// - `UndefinedColumn`
///   - Errors caused by the use of an undefined column in a query.
///   - Fields:
///     - `column` - The name of the undefined column.
///
/// - `UndefinedTable`
///   - Errors triggered by reference to a non-existent table in a query.
///   - Fields:
///     - `table` - The name of the undefined table.
///
/// - `SerializationFailure`
///   - Errors related to concurrent transaction conflicts, typically caused by serialization failures.
///
/// - `QueryCanceled`
///   - Errors occurring when a query is canceled due to a timeout or a manual cancellation.
///
/// - `DeadlockDetected`
///   - Errors resulting from the detection of a deadlock situation during transactional operations.
///
/// - `InsufficientPrivilege`
///   - Errors triggered by insufficient privileges to perform an operation on a database resource.
///
/// - `Database`
///   - A generic error representing other database-related issues.
///   - Fields:
///     - `message` - Additional details about the database error.
///
/// - `Transaction`
///   - Errors specifically related to transactional failures.
///   - Fields:
///     - `message` - Additional details about the transaction error.
///
/// # Examples
///
/// ```rust
/// use thiserror::Error;
/// use crate::RepositoryError;
///
/// fn simulate_error() -> Result<(), RepositoryError> {
///     Err(RepositoryError::UniqueViolation {
///         constraint: "users_email_key".to_string(),
///     })
/// }
///
/// match simulate_error() {
///     Ok(_) => println!("Operation succeeded"),
///     Err(e) => println!("Error occurred: {}", e),
/// }
/// ```
///
/// This struct leverages the `thiserror` crate for deriving the `Error` implementation, making
/// it compatible with standard Rust error handling schemes.
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unique constraint violation on {constraint}")]
    UniqueViolation { constraint: String },

    #[error("Foreign key violation on {constraint}")]
    ForeignKeyViolation { constraint: String },

    #[error("Not null violation on column {column}")]
    NotNullViolation { column: String },

    #[error("Check constraint violation on {constraint}")]
    CheckViolation { constraint: String },

    #[error("String too long for column {column}")]
    StringTooLong { column: String },

    #[error("Numeric value out of range")]
    NumericOutOfRange,

    #[error("Invalid datetime format")]
    InvalidDatetime,

    #[error("Syntax error in SQL")]
    SyntaxError,

    #[error("Undefined column {column}")]
    UndefinedColumn { column: String },

    #[error("Undefined table {table}")]
    UndefinedTable { table: String },

    #[error("Serialization failure (concurrent transaction conflict)")]
    SerializationFailure,

    #[error("Query canceled (timeout or manual cancel)")]
    QueryCanceled,

    #[error("Deadlock detected")]
    DeadlockDetected,

    #[error("Insufficient privilege")]
    InsufficientPrivilege,

    #[error("Generic database error: {message}")]
    Database { message: String },

    #[error("Transaction error: {message}")]
    Transaction { message: String },

    #[error("Invalid page token")]
    InvalidToken,

    #[error("Invalid timestamp in page token")]
    InvalidTimestamp,

    #[error("Invalid UUID in page token")]
    InvalidUuid,
}



/// Maps an `sqlx::Error` to a custom `RepositoryError`.
///
/// This function is used to translate a generic database or SQL error encountered during
/// database operations into a more detailed and structured `RepositoryError` representation.
///
/// The function inspects the type and code of the provided `sqlx::Error` and, when applicable,
/// parses PostgreSQL-specific error codes and details (such as constraint names, column names,
/// or table names). The resulting `RepositoryError` encapsulates this information for better
/// error handling and debugging.
///
/// # Parameters
///
/// - `e`: A value of type `sqlx::Error` representing the original error encountered during database
///   operation.
///
/// # Returns
///
/// - A `RepositoryError` representing the mapped error with specifics of the cause, if available.
///
/// # Behavior
///
/// The function handles errors based on the following rules:
///
/// - If the error is a database-related error (`sqlx::Error::Database`), it attempts to downcast it
///   to a `PgDatabaseError` to extract PostgreSQL-specific error codes and additional details.
/// - Based on the error code from PostgreSQL, a corresponding `RepositoryError` variant is created.
/// - If the error code is not recognized, or in cases where downcasting fails, a generic `Database`
///   error is returned with the underlying error message.
/// - For non-database errors, the function wraps the error message in a `RepositoryError::Database`
///   variant.
///
/// # PostgreSQL Error Codes Mapped:
///
/// - `23505` → `RepositoryError::UniqueViolation`
/// - `23503` → `RepositoryError::ForeignKeyViolation`
/// - `23502` → `RepositoryError::NotNullViolation`
/// - `23514` → `RepositoryError::CheckViolation`
/// - `22001` → `RepositoryError::StringTooLong`
/// - `22003` → `RepositoryError::NumericOutOfRange`
/// - `22007` → `RepositoryError::InvalidDatetime`
/// - `42601` → `RepositoryError::SyntaxError`
/// - `42703` → `RepositoryError::UndefinedColumn`
/// - `42P01` → `RepositoryError::UndefinedTable`
/// - `40001` → `RepositoryError::SerializationFailure`
/// - `57014` → `RepositoryError::QueryCanceled`
/// - `40P01` → `RepositoryError::DeadlockDetected`
/// - `42501` → `RepositoryError::InsufficientPrivilege`
///
/// For unknown PostgreSQL-specific errors, a generic `RepositoryError::Database` is returned with
/// the error message.
///
/// # Examples
///
/// ```rust
/// use sqlx::Error;
/// use crate::{map_sqlx_error, RepositoryError};
///
/// fn process_error(e: sqlx::Error) -> RepositoryError {
///     map_sqlx_error(e)
/// }
///
/// let error = sqlx::Error::RowNotFound;
/// let mapped_error = process_error(error);
///
/// match mapped_error {
///     RepositoryError::Database { message } => {
///         println!("Database error: {}", message);
///     },
///     _ => println!("Other error"),
/// }
/// ```
///
/// # Notes
///
/// - The function is intended to simplify error handling and provide domain-specific error types
///   in a PostgreSQL-backed application.
/// - This implementation depends on the `sqlx` and `thiserror` crates.
#[tracing::instrument(name = "Map PostrgeSQL error to SQL Errors",level = tracing::Level::DEBUG)]
pub fn map_sqlx_error(e: sqlx::Error) -> RepositoryError {
    if let sqlx::Error::Database(db_err) = &e {
        if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
            match pg_err.code() {
                "23505" => RepositoryError::UniqueViolation {
                    constraint: pg_err.constraint().unwrap_or_default().to_string(),
                },
                "23503" => RepositoryError::ForeignKeyViolation {
                    constraint: pg_err.constraint().unwrap_or_default().to_string(),
                },
                "23502" => RepositoryError::NotNullViolation {
                    column: pg_err.column().unwrap_or_default().to_string(),
                },
                "23514" => RepositoryError::CheckViolation {
                    constraint: pg_err.constraint().unwrap_or_default().to_string(),
                },
                "22001" => RepositoryError::StringTooLong {
                    column: pg_err.column().unwrap_or_default().to_string(),
                },
                "22003" => RepositoryError::NumericOutOfRange,
                "22007" => RepositoryError::InvalidDatetime,
                "42601" => RepositoryError::SyntaxError,
                "42703" => RepositoryError::UndefinedColumn {
                    column: pg_err.column().unwrap_or_default().to_string(),
                },
                "42P01" => RepositoryError::UndefinedTable {
                    table: pg_err.table().unwrap_or_default().to_string(),
                },
                "40001" => RepositoryError::SerializationFailure,
                "57014" => RepositoryError::QueryCanceled,
                "40P01" => RepositoryError::DeadlockDetected,
                "42501" => RepositoryError::InsufficientPrivilege,
                _ => RepositoryError::Database {
                    message: pg_err.message().to_string(),
                },
            }
        } else {
            RepositoryError::Database {
                message: db_err.message().to_string(),
            }
        }
    } else {
        RepositoryError::Database {
            message: e.to_string(),
        }
    }
}
