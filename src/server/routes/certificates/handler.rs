use super::dto::CertificateInfoResponse;
use crate::server;
use crate::server::models::certificates::db::CertificateInfo;
use crate::server::models::certificates::repository::CertificateRepository;
use crate::server::models::responses::RepositoryError;
use crate::server::routes::responses::{to_response, to_response_list};
use actix_web::{web, Responder};
use sha2::Digest;
use uuid::Uuid;

#[tracing::instrument(name = "Get All Certificates", skip(pool))]
pub async fn get_handler(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let repo = CertificateRepository::new(
        pool.get_ref().clone(),
    );
    to_response_list::<CertificateInfo, CertificateInfoResponse, RepositoryError>(
        repo.find_all().await,
    )
    // match repo.find_all().await {
    //     Ok(certs) => {
    //         let dtos: Result<Vec<_>, _> = certs
    //             .into_iter()
    //             .map(CertificateResponse::try_from)
    //             .collect();
    //         match dtos {
    //             Ok(valid_dtos) => HttpResponse::Ok().json(valid_dtos),
    //             Err(e) => {
    //                 tracing::error!("Failed to convert certificate: {}", e);
    //                 HttpResponse::UnprocessableEntity().json(serde_json::json!({
    //                     "error": "Invalid certificate format",
    //                     "message": e.to_string()
    //                 }))
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         tracing::error!("Failed to retrieve legacy_certificates: {}", e);
    //         HttpResponse::InternalServerError().json(serde_json::json!({
    //             "error": "Failed to retrieve legacy_certificates",
    //             "message": e.to_string()
    //         }))
    //     }
    // }
}

/*#[tracing::instrument(name = "Create Certificate", skip(pool, payload))]
pub async fn post_handler(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<CreateCertificateRequest>,
) -> impl Responder {
    let create_certificate_request   = payload.into_inner();
    
    // Convert DTO to internal request model
    let request = CertificateGenerationRequest {
        key_algorithm: dto.key_algorithm,
        key_strength: dto.key_strength,
        subject: dto.subject.clone(),
        sans: dto.sans.clone(),
        validity_days: dto.validity_days,
    };

    // Generate key and CSR
    let (private_key_pem, csr_pem, public_key_pem) = match request.generate_key_and_csr() {
        Ok(keys) => keys,
        Err(e) => {
            tracing::error!("Failed to generate key and CSR: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to generate key and CSR",
                "message": e.to_string()
            }));
        }
    };

    // Store in database
    let repo = CertificateRepository::new(pool.get_ref().clone());

    let cert_id = match repo
        .create(
            &csr_pem,
            &private_key_pem,
            &public_key_pem,
            dto.key_algorithm,
            dto.key_strength,
            dto.subject.organization.as_deref(),
            dto.subject.organizational_unit.as_deref(),
            dto.subject.country.as_deref(),
            dto.subject.state_or_province.as_deref(),
            dto.subject.locality.as_deref(),
            dto.subject.email.as_deref(),
            &dto.sans,
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to store certificate in database: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to store certificate",
                "message": e.to_string()
            }));
        }
    };

    // Retrieve the created certificate
    match repo.find_by_id(cert_id).await {
        Ok(Some(cert)) => match CertificateResponseDto::try_from(cert) {
            Ok(response_dto) => HttpResponse::Created().json(response_dto),
            Err(e) => {
                tracing::error!("Failed to convert certificate: {}", e);
                HttpResponse::UnprocessableEntity().json(serde_json::json!({
                    "error": "Invalid certificate format",
                    "message": e.to_string()
                }))
            }
        },
        Ok(None) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Certificate created but not found"
        })),
        Err(e) => {
            tracing::error!("Failed to retrieve created certificate: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve certificate",
                "message": e.to_string()
            }))
        }
    }
}*/

#[tracing::instrument(name = "Get Certificate by ID", skip(pool))]
pub async fn get_by_id_handler(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let cert_id = path.into_inner();
    let repo = server::models::certificates::repository::CertificateRepository::new(
        pool.get_ref().clone(),
    );
    to_response::<CertificateInfo, CertificateInfoResponse, RepositoryError>(
        repo.find_by_id(cert_id).await,
    )
}

/*pub async fn put_handler() -> impl Responder {
    HttpResponse::MethodNotAllowed().json(serde_json::json!({
        "error": "PUT not supported. Use PATCH to upload signed certificate."
    }))
}

#[tracing::instrument(name = "Patch Certificate", skip(pool, payload))]
pub async fn patch_handler(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<PatchCertificateDto>,
) -> impl Responder {
    let cert_id = path.into_inner();
    let dto = payload.into_inner();

    // Parse and validate the certificate

    let pem = match pem::parse(&dto.cert_pem) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid PEM format",
                    "message": e.to_string()
            }));
        }
    };

    if pem.tag() != "CERTIFICATE" {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Not a certificate PEM block"
        }));
    }

    let (_, cert) = match X509Certificate::from_der(pem.contents()) {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Failed to parse certificate",
                "message": e.to_string()
            }));
        }
    };

    // Extract validity
    let valid_from =
        match chrono::DateTime::from_timestamp(cert.validity().not_before.timestamp(), 0) {
            Some(dt) => dt,
            None => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid validity start time"
                }));
            }
        };

    let expires_at =
        match chrono::DateTime::from_timestamp(cert.validity().not_after.timestamp(), 0) {
            Some(dt) => dt,
            None => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid expiry time"
                }));
            }
        };

    // Calculate fingerprint (SHA-256)
    let mut hasher = Sha256::new();
    hasher.update(pem.contents());
    let fingerprint = format!("{:x}", hasher.finalize());

    // Update database
    let repo = CertificateRepository::new(pool.get_ref().clone());

    match repo
        .patch_certificate(
            cert_id,
            &dto.cert_pem,
            dto.chain_pem.as_deref(),
            &fingerprint,
            valid_from,
            expires_at,
        )
        .await
    {
        Ok(_) => {
            // Retrieve updated certificate
            match repo.find_by_id(cert_id).await {
                Ok(Some(cert)) => match CertificateResponseDto::try_from(cert) {
                    Ok(response_dto) => HttpResponse::Ok().json(response_dto),
                    Err(e) => {
                        tracing::error!("Failed to convert updated certificate: {}", e);
                        HttpResponse::UnprocessableEntity().json(serde_json::json!({
                            "error": "Invalid certificate format after update",
                            "message": e.to_string()
                        }))
                    }
                },
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Certificate not found after update"
                })),
                Err(e) => {
                    tracing::error!("Failed to retrieve updated certificate: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to retrieve certificate",
                        "message": e.to_string()
                    }))
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to patch certificate: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Failed to patch certificate",
                "message": e.to_string()
            }))
        }
    }
}

#[tracing::instrument(name = "Delete Certificate", skip(pool))]
pub async fn delete_handler(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let cert_id = path.into_inner();
    let repo = CertificateRepository::new(pool.get_ref().clone());

    match repo.soft_delete(cert_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete certificate: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Failed to delete certificate",
                "message": e.to_string()
            }))
        }
    }
}
*/
