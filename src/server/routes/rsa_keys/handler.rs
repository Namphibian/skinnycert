use crate::server::models::rsa_keys::repository::RsaKeyRepository;
use crate::server::routes::rsa_keys::dto::{NewRsaKeyAlgorithmRequest, RsaKeyAlgorithmPatchRequest, to_create_response, to_patch_response, to_response, to_response_list, to_delete_response};
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;
use crate::server::routes::extractors::PathUuid;

#[tracing::instrument(name = "Get All RSAKeys", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response_list(repo.find_all().await)
}

pub async fn get_by_id_handler(
    pool: web::Data<sqlx::PgPool>,
    id: PathUuid,
) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response(repo.find_by_id(id.0).await)
}

#[tracing::instrument(name = "Create RSAKey Algorithm", skip(pool, payload))]
pub async fn post_handler(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<NewRsaKeyAlgorithmRequest>,
) -> impl Responder {
    let dto = payload.into_inner();
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_create_response(repo.create(dto.rsa_key_size).await)
}

#[tracing::instrument(name = "Put RSAKeys", skip(_pool))]
pub async fn put_handler(_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    return HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Cannot Update RSA Key Algorithm",
        "message": "RSA Key Algorithms cannot be updated."
    }));
}

pub async fn patch_handler(
    pool: web::Data<sqlx::PgPool>,
    id: PathUuid,
    payload: web::Json<RsaKeyAlgorithmPatchRequest>,
) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_patch_response(repo.patch(id.0, payload.deprecated).await)
}

pub async fn delete_handler(
    pool: web::Data<sqlx::PgPool>,
    id: PathUuid,
) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_delete_response(repo.delete(id.0).await)
}