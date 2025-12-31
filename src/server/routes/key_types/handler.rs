use actix_web::{web, Responder};
use crate::server::models::responses::RepositoryError;
use crate::server::models::rsa_key::db::RSAKeyAlgorithm;
use crate::server::models::rsa_key::repository::RsaKeyRepository;
use crate::server::routes::responses::to_response_list;
use crate::server::routes::rsa_keys::dto::RsaKeyAlgorithmResponse;

#[tracing::instrument(name = "Get All Key Algorithm Types", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response_list::<RSAKeyAlgorithm, RsaKeyAlgorithmResponse, RepositoryError>(
        repo.find_all().await,
    )
}