use crate::server::models::ecdsa_keys::db::EcdsaKeyAlgorithm;
use crate::server::models::ecdsa_keys::repository::EcdsaKeyRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::ecdsa_keys::dto::{EcdsaKeyAlgorithmResponse, EcdsaKeyKeyPairResponse};
use crate::server::routes::extractors::PathUuid;
use crate::server::routes::responses::{to_response, to_response_list};
use crate::server::routes::rsa_keys::dto::RsaKeyPairResponse;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use base64::engine::general_purpose;
use openssl::rsa::Rsa;

#[tracing::instrument(name = "Get All ECDSA Key Algorithms", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = EcdsaKeyRepository::new(pool.get_ref().clone());
    to_response_list::<EcdsaKeyAlgorithm, EcdsaKeyAlgorithmResponse, RepositoryError>(repo.find_all().await)
}


#[tracing::instrument(name = "Get ECDSA Algorithm By ID", skip(pool))]
pub async fn get_by_id_handler(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = EcdsaKeyRepository::new(pool.get_ref().clone());
    to_response::<EcdsaKeyAlgorithm, EcdsaKeyAlgorithmResponse, RepositoryError>(repo.find_by_id(id.0).await)
}

#[tracing::instrument(name = "Generate ECDSA Key Pair", skip(pool))]
pub async fn generate_key_pair(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = EcdsaKeyRepository::new(pool.get_ref().clone());
    let algo = repo.find_by_id(id.0).await;
    match algo {
        Ok(Some(model)) => {
            match model.generate_key_pair() {
                Ok((private_key, public_key)) => HttpResponse::Ok().json(EcdsaKeyKeyPairResponse{ private_key, public_key }),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to generate key pair",
                    "message": e.to_string()
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "ECDSA Key Algorithm not found"
        })),
        Err(e) => {
            tracing::error!(error = %e, context = "generate_key_pair", "Repository error while generating ECDSA key pair.");
            HttpResponse::build(e.status_code()).json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }


    }