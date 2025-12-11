
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;
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
}

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
