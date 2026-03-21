use crate::server::models::key_algorithm_statuses::repository::KeyAlgorithmStatusRepository;
use crate::server::routes::key_statuses::dto::KeyAlgorithmStatusResponse;
use crate::server::routes::responses::ErrorResponse;

use actix_web::{web, Responder};
use crate::to_response_list;

#[utoipa::path(
    get,
    path = "/key_statuses",
    responses(
        (status = 200, description = "List all key algorithm statuses", body = [KeyAlgorithmStatusResponse]),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[tracing::instrument(name = "Get All Key Algorithm Statuses", skip(pool))]
pub async fn get_key_algorithm_statuses(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmStatusRepository::new(pool.get_ref().clone());
    // to_response_list::<KeyAlgorithmStatus, KeyAlgorithmStatusResponse, RepositoryError>(
    //     repo.find_all().await,
    // )
    to_response_list!(repo.find_all().await, KeyAlgorithmStatusResponse)
}
