use crate::server::models::responses::{PatchResult, RepositoryError};
use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;

use actix_web::{HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RsaKeyAlgorithmResponse {
    pub id: Uuid,
    pub display_name: String,
    pub algorithm: String,
    pub key_size: i32,
    pub deprecated: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRsaKeyAlgorithmRequest {
    pub rsa_key_size: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RsaKeyAlgorithmPatchRequest {
    pub deprecated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RsaKeyPairResponse {
    pub public_key: String,
    pub private_key: String,
}

impl TryFrom<RSAKeyAlgorithm> for RsaKeyAlgorithmResponse {
    type Error = Box<dyn std::error::Error + Send + Sync>;


    fn try_from(value: RSAKeyAlgorithm) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            display_name: value.display_name,
            algorithm: value.algorithm,
            key_size: value.key_size,
            deprecated: value.deprecated,
            created_on: value.created_on,
            updated_on: value.updated_on,
        })
    }
}


// pub fn to_response(result: Result<Option<RSAKeyAlgorithm>, RepositoryError>) -> HttpResponse {
//     match result {
//         Ok(Some(model)) => map_model(model),
//         Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
//             "error": "RSA Key not found"
//         })),
//         Err(e) => {
//             tracing::error!(error = %e, context = "to_response/find_by_id", "Repository error while fetching RSA key");
//             HttpResponse::build(e.status_code()).json(serde_json::json!({
//                 "error": e.to_string()
//             }))
//         }
//     }
// }



pub fn to_patch_response(
    result: Result<PatchResult<RSAKeyAlgorithm>, RepositoryError>,
) -> HttpResponse {
    match result {
        Ok(PatchResult::Updated(model)) => map_model(model),
        Ok(PatchResult::NotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Patch RSA Key not found. RSA Key may have been deleted after the patch request was processed."
        })),
        Ok(PatchResult::NotModified) => HttpResponse::NotModified().finish(),
        Err(e) => {
            tracing::error!(error = %e, context = "to_patch_response", "Repository error while patching RSA key");
            HttpResponse::build(e.status_code()).json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

pub fn to_create_response(result: Result<RSAKeyAlgorithm, RepositoryError>) -> HttpResponse {
    match result {
        Ok(model) => match RsaKeyAlgorithmResponse::try_from(model) {
            Ok(dto) => HttpResponse::Created().json(dto),
            Err(e) => {
                tracing::error!(error = %e, context = "to_create_response", "Conversion failed for newly created RSA key");
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Invalid RSA key algorithm format",
                    "message": e.to_string()
                }))
            }
        },
        Err(e) => {
            tracing::error!(error = %e, context = "to_create_response", "Repository error while creating RSA key");
            HttpResponse::build(e.status_code()).json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

pub fn to_delete_response(result: Result<Option<RSAKeyAlgorithm>, RepositoryError>) -> HttpResponse {
    match result {
        Ok(Some(_)) => HttpResponse::NoContent().finish(),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "RSA Key not found"
        })),
        Err(e) => {
            tracing::error!(error = %e, context = "to_delete_response", "Repository error while deleting RSA key");
            HttpResponse::build(e.status_code()).json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}
fn map_model(model: RSAKeyAlgorithm) -> HttpResponse {
    match RsaKeyAlgorithmResponse::try_from(model) {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            tracing::error!(error = %e, context = "map_model", "Conversion failed for RSA key model");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Invalid RSA key algorithm format",
                "message": e.to_string()
            }))
        }
    }
}
