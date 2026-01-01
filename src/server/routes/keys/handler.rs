use crate::server::models::key_algorithms::db::KeyAlgorithmInfo;
use crate::server::models::key_algorithms::repository::KeyAlgorithmRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::keys::dto::KeyAlgorithmResponse;
use crate::server::routes::responses::{key_pair_response, to_response, to_response_list};
use actix_web::{web, Responder};

use crate::server::routes::extractors::PathUuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyAlgorithmFilter {
    pub algorithm_type: Option<String>,
    pub tls_status: Option<String>,
    pub algorithm_status: Option<String>,
    pub strength: Option<i32>,
}

#[tracing::instrument(name = "Get All Key Algorithms", skip(pool))]
pub async fn get_handler(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<KeyAlgorithmFilter>,
) -> impl Responder {
    let filter = query.into_inner();
    let repo = KeyAlgorithmRepository::new(pool.get_ref().clone());

    match repo.find_all().await {
        Ok(models) => {
            let filtered: Vec<KeyAlgorithmInfo> = models
                .into_iter()
                .filter(|m| {
                    filter
                        .algorithm_type
                        .as_ref()
                        .map(|t| m.algorithm_type_name.eq_ignore_ascii_case(t))
                        .unwrap_or(true)
                })
                .filter(|m| {
                    filter
                        .tls_status
                        .as_ref()
                        .map(|t| m.tls_status_name.eq_ignore_ascii_case(t))
                        .unwrap_or(true)
                })
                .filter(|m| {
                    filter
                        .algorithm_status
                        .as_ref()
                        .map(|s| m.status_name.eq_ignore_ascii_case(s))
                        .unwrap_or(true)
                })
                .filter(|m| {
                    filter
                        .strength
                        .map(|s| m.key_algorithm_strength == Some(s))
                        .unwrap_or(true)
                })
                .collect();

            to_response_list::<KeyAlgorithmInfo, KeyAlgorithmResponse, RepositoryError>(Ok(
                filtered,
            ))
        }

        Err(e) => to_response_list::<KeyAlgorithmInfo, KeyAlgorithmResponse, _>(Err(e)),
    }
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
