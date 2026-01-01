use crate::server::models::key_algorithm_types::db::KeyAlgorithmTypeInfo;
use crate::server::models::key_algorithm_types::repository::KeyAlgorithmTypeRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::key_types::dto::KeyAlgorithmTypeResponse;
use crate::server::routes::responses::to_response_list;
use actix_web::{web, Responder};

#[tracing::instrument(name = "Get All Key Algorithm Types", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmTypeRepository::new(pool.get_ref().clone());
    to_response_list::<KeyAlgorithmTypeInfo, KeyAlgorithmTypeResponse, RepositoryError>(
        repo.find_all().await,
    )
}
