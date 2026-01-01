use crate::server::models::key_algorithm_type_tls_statuses::db::KeyAlgorithmTypeTlsStatus;
use crate::server::models::key_algorithm_type_tls_statuses::repository::KeyAlgorithmTypeTlsStatusRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::key_type_tls_statuses::dto::KeyAlgorithmTlsStatusResponse;
use crate::server::routes::responses::to_response_list;
use actix_web::{web, Responder};

#[tracing::instrument(name = "Get All Key Algorithm Type Tls Status", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmTypeTlsStatusRepository::new(pool.get_ref().clone());
    to_response_list::<KeyAlgorithmTypeTlsStatus, KeyAlgorithmTlsStatusResponse, RepositoryError>(
        repo.find_all().await,
    )
}
