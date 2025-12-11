use std::error::Error;
use actix_web::{HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::models::repository_errors::RepositoryError;
use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;

#[derive(Debug, Serialize, Deserialize)]
pub struct RsaKeyAlgorithmResponse{
    pub id: Uuid,
    pub display_name: String,
    pub algorithm: String,
    pub key_size: i32,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRsaKeyAlgorithmRequest {
    pub rsa_key_size: i32
}
impl TryFrom<RSAKeyAlgorithm> for RsaKeyAlgorithmResponse {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(db_rsa_key: RSAKeyAlgorithm) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_rsa_key.id,
            display_name: db_rsa_key.display_name,
            algorithm: db_rsa_key.algorithm,
            key_size: db_rsa_key.key_size,
            created_on: db_rsa_key.created_on,
            updated_on: db_rsa_key.updated_on
        })
    }
}

pub fn to_response(result: Result<Option<RSAKeyAlgorithm>, RepositoryError>) -> HttpResponse {
    match result {
        Ok(Some(model)) => match RsaKeyAlgorithmResponse::try_from(model) {
            Ok(dto) => HttpResponse::Ok().json(dto),
            Err(e) => {
                tracing::error!("Conversion failed: {}", e);
                HttpResponse::UnprocessableEntity().json(serde_json::json!({
                    "error": "Invalid RSA key algorithm format",
                    "message": e.to_string()
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "RSA Key not found"
        })),
        Err(e) => {
            tracing::error!("Repository error: {}", e);
            HttpResponse::build(e.status_code()).json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

pub fn to_response_list(
    result: Result<Vec<RSAKeyAlgorithm>, Box<dyn Error>>
) -> HttpResponse {
    match result {
        Ok(models) => {
            let dtos: Result<Vec<_>, _> = models.into_iter()
                .map(RsaKeyAlgorithmResponse::try_from)
                .collect();
            match dtos {
                Ok(valid) => HttpResponse::Ok().json(valid),
                Err(e) => {
                    tracing::error!("Conversion failed: {}", e);
                    HttpResponse::UnprocessableEntity().json(serde_json::json!({
                        "error": "Invalid RSA key format",
                        "message": e.to_string()
                    }))
                }
            }
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error",
                "message": e.to_string()
            }))
        }
    }
}