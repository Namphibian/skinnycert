use super::dto::{CertificateInfoResponse, CreateCertificateRequest, PatchCertificateRequest};
use crate::server::models::certificates::repository::CertificateRepository;
use crate::server::models::key_algorithms::repository::KeyAlgorithmRepository;

use actix_web::{web, HttpResponse, Responder};

use crate::server::models::certificates::filters::CertificateFilterParams;
use crate::server::models::key_algorithms::db::{GenerateCertificateSigningRequest, KeyPair};
use crate::server::models::shared::{CertificateSubjectFields, CsrGenerationParams};
use crate::{to_delete_response, to_response, to_response_paged};
use uuid::Uuid;

#[tracing::instrument(name = "Get All Certificates", skip(pool))]
pub async fn get_handler(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<CertificateFilterParams>,
) -> impl Responder {
    let repo = CertificateRepository::new(pool.get_ref().clone());
    let filter = query.into_inner();
    to_response_paged!(repo.find_all_paged(&filter).await, CertificateInfoResponse)
}

#[tracing::instrument(name = "Create Certificate Handler", skip(pool, payload))]
pub async fn post_handler(
    pool: web::Data<sqlx::PgPool>,
    payload: web::Json<CreateCertificateRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let key_repo = KeyAlgorithmRepository::new(pool.get_ref().clone());
    let create_certificate_request = payload.into_inner();
    create_certificate_request.validate().map_err(|e| {
        tracing::error!("Validation error: {:?}", e);
        actix_web::error::ErrorBadRequest("Validation error")
    })?;

    // --- Lookup key algorithm ---
    let key_algorithm = key_repo
        .find_by_id(create_certificate_request.key_algorithm_id)
        .await
        .map_err(|e| {
            tracing::error!("Database error fetching key algorithm: {:?}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| {
            tracing::error!(
                "Key algorithm not found for ID: {}",
                create_certificate_request.key_algorithm_id
            );
            actix_web::error::ErrorNotFound("Key algorithm not found")
        })?;

    // --- Generate keypair ---
    let (private_key_pem, public_key_pem) = key_algorithm.generate_key_pair().map_err(|e| {
        tracing::error!("Keypair generation failed: {:?}", e);
        actix_web::error::ErrorInternalServerError("Keypair generation failed")
    })?;

    // --- Build CSR params (domain struct, not DTO) ---
    let csr_params = CsrGenerationParams {
        subject: CertificateSubjectFields {
            organization: create_certificate_request.subject.organization.clone(),
            organizational_unit: create_certificate_request
                .subject
                .organizational_unit
                .clone(),
            country: create_certificate_request.subject.country.clone(),
            state_or_province: create_certificate_request.subject.state_or_province.clone(),
            locality: create_certificate_request.subject.locality.clone(),
            email: create_certificate_request.subject.email.clone(),
        },
        sans: create_certificate_request.sans.clone(),
    };

    // --- Generate CSR ---
    let csr_pem = key_algorithm
        .generate_csr(&private_key_pem, &public_key_pem, &csr_params)
        .map_err(|e| {
            tracing::error!("CSR generation failed: {:?}", e);
            actix_web::error::ErrorInternalServerError("CSR generation failed")
        })?;
    let certificate_repo = CertificateRepository::new(pool.get_ref().clone());
    let cert_id = certificate_repo
        .create(
            &csr_pem,
            &private_key_pem,
            &public_key_pem,
            create_certificate_request.key_algorithm_id,
            create_certificate_request.subject.organization.as_deref(),
            create_certificate_request
                .subject
                .organizational_unit
                .as_deref(),
            create_certificate_request.subject.country.as_deref(),
            create_certificate_request
                .subject
                .state_or_province
                .as_deref(),
            create_certificate_request.subject.locality.as_deref(),
            create_certificate_request.subject.email.as_deref(),
            create_certificate_request.sans.as_slice(),
        )
        .await;
    match cert_id {
        Ok(cert_id) => Ok(to_response!(
            certificate_repo.find_by_id(cert_id).await,
            CertificateInfoResponse
        )),
        Err(e) => {
            tracing::error!("Certificate creation failed: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Certificate creation failed",
            ))
        }
    }
}

#[tracing::instrument(name = "Get Certificate by ID Handler", skip(pool))]
pub async fn get_by_id_handler(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let cert_id = path.into_inner();
    let repo = CertificateRepository::new(pool.get_ref().clone());
    to_response!(repo.find_by_id(cert_id).await,CertificateInfoResponse
    )
}

#[tracing::instrument(name = "Put Certificate Handler")]
pub async fn put_handler() -> impl Responder {
    HttpResponse::MethodNotAllowed().json(serde_json::json!({
        "error": "PUT not supported. Use PATCH to upload signed certificate."
    }))
}

#[tracing::instrument(name = "Patch Certificate", skip(pool, payload))]
pub async fn patch_handler(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
    payload: web::Json<PatchCertificateRequest>,
) -> impl Responder {
    let cert_id = path.into_inner();
    let dto = payload.into_inner();
    let repo = CertificateRepository::new(pool.get_ref().clone());

    // 1. Fetch existing certificate to get CSR and public key
    let existing_cert = match repo.find_by_id(cert_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Certificate not found"
            }));
        }
        Err(e) => {
            tracing::error!("Failed to fetch certificate: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }));
        }
    };

    // 2. Validate the patch request
    let (valid_from, expires_at, fingerprint) =
        match dto.validate(&existing_cert.csr_pem, &existing_cert.public_key_pem) {
            Ok(v) => v,
            Err(e) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Validation failed",
                    "message": e.to_string()
                }));
            }
        };

    // 3. Update database
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
            to_response!(
                repo.find_by_id(cert_id).await, CertificateInfoResponse
            )
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
#[tracing::instrument(name = "Delete Certificate Handler", skip(pool))]
pub async fn delete_handler(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let cert_id = path.into_inner();
    let repo = CertificateRepository::new(pool.get_ref().clone());
    to_delete_response!(repo.delete_by_id(cert_id).await)
}
