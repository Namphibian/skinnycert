use crate::server::models::key_algorithm_types::repository::KeyAlgorithmTypeRepository;
use crate::server::routes::key_types::dto::KeyAlgorithmTypeResponse;

use actix_web::{web, Responder};
use crate::to_response_list;

#[tracing::instrument(name = "Get All Key Algorithm Types", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmTypeRepository::new(pool.get_ref().clone());
    to_response_list!(
        repo.find_all().await, KeyAlgorithmTypeResponse
    )
}
