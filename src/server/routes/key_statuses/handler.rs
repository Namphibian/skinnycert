use crate::server::models::key_algorithm_statuses::db::KeyAlgorithmStatus;
use crate::server::models::key_algorithm_statuses::repository::KeyAlgorithmStatusRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::key_statuses::dto::KeyAlgorithmStatusResponse;
use crate::server::routes::responses::to_response_list;
use actix_web::{web, Responder};

#[tracing::instrument(name = "Get All Key Algorithm Statuses", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmStatusRepository::new(pool.get_ref().clone());
    to_response_list::<KeyAlgorithmStatus, KeyAlgorithmStatusResponse, RepositoryError>(
        repo.find_all().await,
    )
}
