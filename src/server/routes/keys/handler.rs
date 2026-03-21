use crate::server::models::key_algorithms::db::KeyPair;
use crate::server::models::key_algorithms::filters::KeyAlgorithmFilterParams;
use crate::server::models::key_algorithms::repository::KeyAlgorithmRepository;
use crate::server::routes::extractors::PathUuid;
use crate::server::routes::keys::dto::KeyAlgorithmResponse;
use crate::server::routes::responses::{ErrorResponse, KeyPairResponse};

use actix_web::{web, Responder};
use crate::{key_pair_response, to_response, to_response_list};

#[utoipa::path(
    get,
    path = "/keys",
    params(KeyAlgorithmFilterParams),
    responses(
        (status = 200, description = "List all key algorithms", body = [KeyAlgorithmResponse]),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
#[tracing::instrument(name = "Get All Key Algorithms", skip(pool))]
pub async fn get_key_algorithms(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<KeyAlgorithmFilterParams>,
) -> impl Responder {
    let filter = query.into_inner();
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    to_response_list!(
        repo.find_all(&filter).await, KeyAlgorithmResponse
    )
}

#[utoipa::path(
    get,
    path = "/keys/{id}",
    responses(
        (status = 200, description = "Get key algorithm by ID", body = KeyAlgorithmResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Key Algorithm ID")
    )
)]
#[tracing::instrument(name = "Get Key Algorithm By ID", skip(pool))]
pub async fn get_key_algorithm_by_id(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    to_response!(
        repo.find_by_id(id.0).await, KeyAlgorithmResponse
    )
}

#[utoipa::path(
    get,
    path = "/keys/{id}/keypair",
    responses(
        (status = 200, description = "Generate crypto key pair", body = KeyPairResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Key Algorithm ID")
    )
)]
#[tracing::instrument(name = "Generate Crypto Key Pair", skip(pool))]
pub async fn generate_key_pair(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    let algo = repo.find_by_id(id.0).await;
    key_pair_response!(algo, "Key Algorithm not found")
}
