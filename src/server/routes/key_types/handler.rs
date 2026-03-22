use crate::server::models::key_algorithm_types::repository::KeyAlgorithmTypeRepository;
use crate::server::routes::key_types::dto::KeyAlgorithmTypeResponse;
use crate::server::routes::responses::ErrorResponse;

use actix_web::{web, Responder};
use crate::to_response_list;

#[utoipa::path(
    get,
    path = "/key_types",
    responses(
        (status = 200, description = "List all key algorithm types", body = [KeyAlgorithmTypeResponse]),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[tracing::instrument(name = "Get All Key Algorithm Types", skip(pool))]
pub async fn get_key_algorithm_types(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmTypeRepository::new(pool.get_ref().clone());
    to_response_list!(
        repo.find_all().await, KeyAlgorithmTypeResponse
    )
}
