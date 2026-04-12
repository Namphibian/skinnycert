use utoipa::OpenApi;
use crate::server::routes;
use crate::server::routes::keys::dto::KeyAlgorithmResponse;
use crate::server::routes::keys::dto::KeyAlgorithmStatusResponse;
use crate::server::routes::certificates::dto::*;
use crate::server::routes::key_types::dto::KeyAlgorithmTypeResponse;
use crate::server::routes::key_statuses::dto::KeyAlgorithmStatusResponse as KeyStatusResponse;
use crate::server::routes::key_type_tls_statuses::dto::KeyAlgorithmTlsStatusResponse;
use crate::server::routes::responses::{ErrorResponse, KeyPairResponse};
use crate::server::routes::health_check::dto::{HealthCheckResponse, MemoryInfo};
use crate::server::models::shared::PageDirection;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SkinnyCert API",
        version = "0.1.0",
        description = "Industrial-strength TLS certificate API built in Rust",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT"),
        contact(name = "Cornelius Franken", email = "catch22@tuta.com", url = "https://github.com/Namphibian/skinnycert")
    ),
    paths(
        routes::keys::handler::get_key_algorithms,
        routes::keys::handler::get_key_algorithm_by_id,
        routes::keys::handler::generate_key_pair,
        routes::certificates::handler::get_certificates,
        routes::certificates::handler::create_certificate,
        routes::certificates::handler::get_certificate_by_id,
        routes::certificates::handler::patch_certificate,
        routes::certificates::handler::delete_certificate,
        routes::key_types::handler::get_key_algorithm_types,
        routes::key_statuses::handler::get_key_algorithm_statuses,
        routes::key_type_tls_statuses::handler::get_key_algorithm_type_tls_statuses,
        routes::health_check::handler::get_health,
    ),
    components(
        schemas(
            KeyAlgorithmResponse,
            KeyAlgorithmStatusResponse,
            KeyAlgorithmTypeResponse,
            KeyAlgorithmTlsStatusResponse,
            CertificateInfoResponse,
            CertificateListResponse,
            CreateCertificateRequest,
            PatchCertificateRequest,
            CertificateSubject,
            PemDataResponse,
            SubjectDataResponse,
            SansDataResponse,
            X509MetadataResponse,
            KeyPairResponse,
            ErrorResponse,
            HealthCheckResponse,
            MemoryInfo,
            PageDirection,
            KeyStatusResponse
        )
    ),
    tags(
        (name = "skinnycert", description = "SkinnyCert API")
    )
)]
pub struct ApiDoc;
