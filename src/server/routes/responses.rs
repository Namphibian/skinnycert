use crate::server::models::responses::RepositoryError;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyPairResponse {
    pub public_key: String,
    pub private_key: String,
}
impl ResponseError for RepositoryError {
    #[tracing::instrument(name = "Response status_code",level = tracing::Level::DEBUG)]
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            RepositoryError::UniqueViolation { .. } => actix_web::http::StatusCode::CONFLICT,
            RepositoryError::ForeignKeyViolation { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::NotNullViolation { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::CheckViolation { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::StringTooLong { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::NumericOutOfRange => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::InvalidDatetime => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::SyntaxError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::UndefinedColumn { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::UndefinedTable { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::SerializationFailure => actix_web::http::StatusCode::CONFLICT,
            RepositoryError::QueryCanceled => actix_web::http::StatusCode::REQUEST_TIMEOUT,
            RepositoryError::DeadlockDetected => actix_web::http::StatusCode::CONFLICT,
            RepositoryError::InsufficientPrivilege => actix_web::http::StatusCode::FORBIDDEN,
            RepositoryError::Database { .. } | RepositoryError::Transaction { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            RepositoryError::InvalidToken => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::InvalidTimestamp => actix_web::http::StatusCode::BAD_REQUEST,
            RepositoryError::InvalidUuid => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    #[tracing::instrument(name = "Response error_response",level = tracing::Level::DEBUG)]
    fn error_response(&self) -> HttpResponse {
        let body = ErrorResponse {
            error: self.to_string(),
            details: match self {
                RepositoryError::Database { message } | RepositoryError::Transaction { message } => Some(message.clone()),
                RepositoryError::UniqueViolation { constraint } | RepositoryError::ForeignKeyViolation { constraint } | RepositoryError::CheckViolation { constraint } => Some(constraint.clone()),
                RepositoryError::NotNullViolation { column } | RepositoryError::StringTooLong { column } | RepositoryError::UndefinedColumn { column } => Some(column.clone()),
                RepositoryError::UndefinedTable { table } => Some(table.clone()),
                _ => None,
            },
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}
