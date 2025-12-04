use crate::server::models::rsa_keys::repository::RsaKeyRepository;
use crate::server::routes::rsa_keys::dto::RsaKeyAlgorithmResponse;
use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

#[tracing::instrument(name = "Get All RSAKeys", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());

    match repo.find_all().await {
        Ok(rsa_keys) => {
            let dtos: Result<Vec<_>, _> = rsa_keys
                .into_iter()
                .map(RsaKeyAlgorithmResponse::try_from)
                .collect();
            match dtos {
                Ok(valid_dtos) => HttpResponse::Ok().json(valid_dtos),
                Err(e) => {
                    tracing::error!("Failed to convert certificate: {}", e);
                    HttpResponse::UnprocessableEntity().json(serde_json::json!({
                        "error": "Invalid rsa key format",
                        "message": e.to_string()
                    }))
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to retrieve certificates: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve certificates",
                "message": e.to_string()
            }))
        }
    }
}

#[tracing::instrument(name = "Get RSA Key Algorithm By ID", skip(pool))]
pub async fn get_by_id_handler(
    pool: web::Data<sqlx::PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let repo = RsaKeyRepository::new(pool.get_ref().clone());
    let rsa_key_id: Uuid = id.into_inner();
    match repo.find_by_id(rsa_key_id).await {
        Ok(Some(rsa_key)) => match RsaKeyAlgorithmResponse::try_from(rsa_key) {
            Ok(dto) => HttpResponse::Ok().json(dto),
            Err(e) => {
                tracing::error!("Failed to convert RSA Key algorithm: {}", e);
                HttpResponse::UnprocessableEntity().json(serde_json::json!({
                    "error": "Invalid RSA key algorithm format",
                    "message": e.to_string()
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "RSA Key not found"
        })),
        Err(e) => {
            tracing::error!("Failed to retrieve RSA Key: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve RSA key algorithm.",
                "message": e.to_string()
            }))
        }
    }
}
