use crate::server::models::key_algorithm_type_tls_statuses::repository::KeyAlgorithmTypeTlsStatusRepository;
use crate::server::routes::key_type_tls_statuses::dto::KeyAlgorithmTlsStatusResponse;
use crate::server::routes::responses::ErrorResponse;

use actix_web::{web, Responder};
use crate::to_response_list;

#[utoipa::path(
    get,
    path = "/key-type-tls-statuses",
    responses(
        (status = 200, description = "List all key algorithm type TLS statuses", body = [KeyAlgorithmTlsStatusResponse]),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[tracing::instrument(name = "Get All Key Algorithm Type Tls Status", skip(pool))]
pub async fn get_key_algorithm_type_tls_statuses(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = KeyAlgorithmTypeTlsStatusRepository::new(pool.get_ref().clone());
    to_response_list!(
        repo.find_all().await,KeyAlgorithmTlsStatusResponse
    )
}
