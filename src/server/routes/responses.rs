use crate::server::models::responses::{PatchResult, RepositoryError};
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

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
            RepositoryError::InvalidDatetime => actix_web::http::StatusCode::BAD_REQUEST,   // 400
            RepositoryError::SyntaxError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // 500
            RepositoryError::UndefinedColumn { .. } => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            } // 500
            RepositoryError::UndefinedTable { .. } => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            } // 500
            RepositoryError::SerializationFailure => actix_web::http::StatusCode::CONFLICT, // 409
            RepositoryError::QueryCanceled => actix_web::http::StatusCode::REQUEST_TIMEOUT, // 408
            RepositoryError::DeadlockDetected => actix_web::http::StatusCode::CONFLICT,     // 409
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
pub fn to_response_list<M, D, E>(result: Result<Vec<M>, E>) -> HttpResponse
where
    D: TryFrom<M> + serde::Serialize,
    D::Error: std::fmt::Display,
    E: Into<RepositoryError> + std::fmt::Display,
{
    match result {
        Ok(models) => {
            let dtos: Result<Vec<_>, _> = models.into_iter().map(D::try_from).collect();
            match dtos {
                Ok(valid) => HttpResponse::Ok().json(valid),
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        context = "to_response_list",
                        "Conversion failed for list"
                    );
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Invalid format",
                        "message": e.to_string()
                    }))
                }
            }
        }
        Err(e) => {
            let err: RepositoryError = e.into();
            tracing::error!(
                error = %err,
                context = "to_response_list",
                "Repository error while fetching list"
            );
            HttpResponse::build(err.status_code()).json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

pub fn to_response<M, D, E>(result: Result<Option<M>, E>) -> HttpResponse
where
    D: TryFrom<M> + serde::Serialize,
    D::Error: std::fmt::Display,
    E: Into<RepositoryError> + std::fmt::Display,
{
    match result {
        Ok(Some(model)) => match D::try_from(model) {
            Ok(dto) => HttpResponse::Ok().json(dto),
            Err(e) => {
                tracing::error!(
                    error = %e,
                    context = "to_response/single",
                    "DTO conversion failed"
                );
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Invalid format",
                    "message": e.to_string()
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Not found"
        })),
        Err(e) => {
            let err: RepositoryError = e.into();
            tracing::error!(
                error = %err,
                context = "to_response/single",
                "Repository error"
            );
            HttpResponse::build(err.status_code()).json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}

pub fn to_patch_response<M, D, E>(result: Result<PatchResult<M>, E>) -> HttpResponse
where
    D: TryFrom<M> + serde::Serialize,
    D::Error: std::fmt::Display,
    E: Into<RepositoryError> + std::fmt::Display,
{
    match result {
        Ok(PatchResult::Updated(model)) => {
            match D::try_from(model) {
                Ok(dto) => HttpResponse::Ok().json(dto),
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        context = "to_patch_response",
                        "DTO conversion failed"
                    );
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Invalid format",
                        "message": e.to_string()
                    }))
                }
            }
        }
        Ok(PatchResult::NotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Resource not found. It may have been deleted after the patch request was processed."
        })),
        Ok(PatchResult::NotModified) => HttpResponse::NotModified().finish(),
        Err(e) => {
            let err: RepositoryError = e.into();
            tracing::error!(
                error = %err,
                context = "to_patch_response",
                "Repository error while patching resource"
            );
            HttpResponse::build(err.status_code()).json(serde_json::json!({
                "error": err.to_string()
            }))
        }
    }
}
