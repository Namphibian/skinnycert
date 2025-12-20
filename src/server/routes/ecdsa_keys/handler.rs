use crate::server::models::ecdsa_keys::db::EcdsaKeyAlgorithm;
use crate::server::models::ecdsa_keys::repository::EcdsaKeyRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::ecdsa_keys::dto::EcdsaKeyAlgorithmResponse;
use crate::server::routes::extractors::PathUuid;
use crate::server::routes::responses::{to_response, to_response_list};
use actix_web::{web, Responder};

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