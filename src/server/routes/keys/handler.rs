use crate::server::models::key_algorithms::db::{KeyAlgorithmFilterParams, KeyAlgorithmInfo};
use crate::server::models::key_algorithms::repository::KeyAlgorithmRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::extractors::PathUuid;
use crate::server::routes::keys::dto::KeyAlgorithmResponse;
use crate::server::routes::responses::{key_pair_response, to_response, to_response_list};
use actix_web::{web, Responder};


#[tracing::instrument(name = "Get All Key Algorithms", skip(pool))]
pub async fn get_handler(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<KeyAlgorithmFilterParams>,
) -> impl Responder {
    let filter = query.into_inner();
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    to_response_list::<KeyAlgorithmInfo, KeyAlgorithmResponse, RepositoryError>(
        repo.find_all(&filter).await,
    )
}

#[tracing::instrument(name = "Get Key Algorithm By ID", skip(pool))]
pub async fn get_by_id_handler(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    to_response::<KeyAlgorithmInfo, KeyAlgorithmResponse, RepositoryError>(
        repo.find_by_id(id.0).await,
    )
}

#[tracing::instrument(name = "Generate Crypto Key Pair", skip(pool))]
pub async fn generate_key_pair(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    let algo = repo.find_by_id(id.0).await;
    key_pair_response(algo, "Key Algorithm not found")
}
