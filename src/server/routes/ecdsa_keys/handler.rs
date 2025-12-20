use actix_web::{web, Responder};
use crate::server::models::ecdsa_keys::db::EcdsaKeyAlgorithm;
use crate::server::models::ecdsa_keys::repository::EcdsaKeyRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::ecdsa_keys::dto::EcdsaKeyAlgorithmResponse;
use crate::server::routes::responses::to_response_list;

#[tracing::instrument(name = "Get All ECDSA Key Algorithms", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = EcdsaKeyRepository::new(pool.get_ref().clone());
    to_response_list::<EcdsaKeyAlgorithm, EcdsaKeyAlgorithmResponse, RepositoryError>(repo.find_all().await)
}