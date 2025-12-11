use crate::server::models::rsa_keys::repository::RsaKeyRepository;
use crate::server::routes::rsa_keys::dto::{NewRsaKeyAlgorithmRequest, to_response, to_response_list, RsaKeyAlgorithmPatchRequest, to_patch_response};
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

#[tracing::instrument(name = "Get All RSAKeys", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response_list(repo.find_all().await)
}

#[tracing::instrument(name = "Get RSA Key Algorithm By ID", skip(pool))]
pub async fn get_by_id_handler(
    pool: web::Data<sqlx::PgPool>,
    id: web::Path<String>,
) -> impl Responder {
    let rsa_key_id = match Uuid::parse_str(&id.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid UUID format"
            }));
        }
    };
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response(repo.find_by_id(rsa_key_id).await)
}

#[tracing::instrument(name = "Create RSAKey Algorithm", skip(pool, payload))]
pub async fn post_handler(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<NewRsaKeyAlgorithmRequest>,
) -> impl Responder {
    let dto = payload.into_inner();
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_response(repo.create(dto.rsa_key_size).await)
}

#[tracing::instrument(name = "Put RSAKeys", skip(_pool))]
pub async fn put_handler(_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    return HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Cannot Update RSA Key Algorithm",
        "message": "RSA Key Algorithms cannot be updated."
    }));
}

#[tracing::instrument(name = "Patch RSAKeys", skip(pool))]
pub async fn patch_handler(
    pool: web::Data<sqlx::PgPool>,
    id: web::Path<String>,
    payload: web::Json<RsaKeyAlgorithmPatchRequest>,
) -> impl Responder {

    let rsa_key_id = match Uuid::parse_str(&id.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid UUID format"
            }));
        }
    };
    let dto = payload.into_inner();
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    to_patch_response(repo.patch(rsa_key_id, dto.deprecated).await)
}
