use crate::server::models::responses::RepositoryError;
use crate::server::models::rsa_key::db::RSAKeyAlgorithm;
use crate::server::models::rsa_key::repository::RsaKeyRepository;
use crate::server::routes::extractors::PathUuid;
use crate::server::routes::responses::{
    key_pair_response, to_patch_response, to_response, to_response_list,
};
use crate::server::routes::rsa_keys::dto::{
    to_create_response, to_delete_response, NewRsaKeyAlgorithmRequest,
    RsaKeyAlgorithmPatchRequest, RsaKeyAlgorithmResponse,
};
use actix_web::{web, HttpResponse, Responder};

#[tracing::instrument(name = "Get All RSA Keys", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response_list::<RSAKeyAlgorithm, RsaKeyAlgorithmResponse, RepositoryError>(
        repo.find_all().await,
    )
}

#[tracing::instrument(name = "Get RSA Key Algorithm By ID", skip(pool))]
pub async fn get_by_id_handler(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response::<RSAKeyAlgorithm, RsaKeyAlgorithmResponse, RepositoryError>(
        repo.find_by_id(id.0).await,
    )
}

#[tracing::instrument(name = "Create RSA Key Algorithm", skip(pool, payload))]
pub async fn post_handler(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<NewRsaKeyAlgorithmRequest>,
) -> impl Responder {
    let dto = payload.into_inner();
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    if dto.rsa_key_size < 1024 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "RSA key size too small.",
            "message": "RSA key size must be at least 1024 bits"
        }));
    }
    to_create_response(repo.create(dto.rsa_key_size).await)
}

#[tracing::instrument(name = "Put RSA Keys", skip(_pool))]
pub async fn put_handler(_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    return HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Cannot Update RSA Key Algorithm",
        "message": "RSA Key Algorithms cannot be updated."
    }));
}

#[tracing::instrument(name = "Patch RSA Keys", skip(pool))]
pub async fn patch_handler(
    pool: web::Data<sqlx::PgPool>,
    id: PathUuid,
    payload: web::Json<RsaKeyAlgorithmPatchRequest>,
) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    //to_patch_response(repo.patch(id.0, payload.deprecated).await)
    to_patch_response::<RSAKeyAlgorithm, RsaKeyAlgorithmResponse, RepositoryError>(
        repo.patch(id.0, payload.deprecated).await,
    )
}

#[tracing::instrument(name = "Delete RSAKeys", skip(pool))]
pub async fn delete_handler(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_delete_response(repo.delete(id.0).await)
}

#[tracing::instrument(name = "Generate RSA Key Pair", skip(pool))]
pub async fn generate_key_pair(pool: web::Data<sqlx::PgPool>, id: PathUuid) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    let algo = repo.find_by_id(id.0).await;
    key_pair_response(algo, "RSA Key Algorithm not found")
}
