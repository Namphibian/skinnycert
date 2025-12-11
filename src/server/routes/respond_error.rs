use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use crate::server::models::repository_errors::RepositoryError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

impl ResponseError for RepositoryError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            RepositoryError::UniqueViolation { .. } => actix_web::http::StatusCode::CONFLICT, // 409
            RepositoryError::ForeignKeyViolation { .. } => actix_web::http::StatusCode::BAD_REQUEST, // 400
            RepositoryError::NotNullViolation { .. } => actix_web::http::StatusCode::BAD_REQUEST, // 400
            RepositoryError::CheckViolation { .. } => actix_web::http::StatusCode::BAD_REQUEST, // 400
            RepositoryError::StringTooLong { .. } => actix_web::http::StatusCode::BAD_REQUEST, // 400
            RepositoryError::NumericOutOfRange => actix_web::http::StatusCode::BAD_REQUEST, // 400
            RepositoryError::InvalidDatetime => actix_web::http::StatusCode::BAD_REQUEST, // 400
            RepositoryError::SyntaxError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // 500
            RepositoryError::UndefinedColumn { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // 500
            RepositoryError::UndefinedTable { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // 500
            RepositoryError::SerializationFailure => actix_web::http::StatusCode::CONFLICT, // 409
            RepositoryError::QueryCanceled => actix_web::http::StatusCode::REQUEST_TIMEOUT, // 408
            RepositoryError::DeadlockDetected => actix_web::http::StatusCode::CONFLICT, // 409
            RepositoryError::InsufficientPrivilege => actix_web::http::StatusCode::FORBIDDEN, // 403
            RepositoryError::Database { .. } | RepositoryError::Transaction { .. } => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorResponse {
            error: self.to_string(),
            details: match self {
                RepositoryError::Database { message }
                | RepositoryError::Transaction { message } => Some(message.clone()),
                RepositoryError::UniqueViolation { constraint }
                | RepositoryError::ForeignKeyViolation { constraint }
                | RepositoryError::CheckViolation { constraint } => Some(constraint.clone()),
                RepositoryError::NotNullViolation { column }
                | RepositoryError::StringTooLong { column }
                | RepositoryError::UndefinedColumn { column } => Some(column.clone()),
                RepositoryError::UndefinedTable { table } => Some(table.clone()),
                _ => None,
            },
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}
