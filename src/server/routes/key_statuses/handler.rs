use crate::server::models::key_algorithm_statuses::repository::KeyAlgorithmStatusRepository;
use crate::server::routes::key_statuses::dto::KeyAlgorithmStatusResponse;

use actix_web::{web, Responder};
use crate::to_response_list;

#[tracing::instrument(name = "Get All Key Algorithm Statuses", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmStatusRepository::new(pool.get_ref().clone());
    // to_response_list::<KeyAlgorithmStatus, KeyAlgorithmStatusResponse, RepositoryError>(
    //     repo.find_all().await,
    // )
    to_response_list!(repo.find_all().await, KeyAlgorithmStatusResponse)
}
