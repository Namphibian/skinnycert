use actix_web::{HttpResponse, Responder};
use chrono::Utc;
use crate::server::routes::handlers::models::tls_certificate::{
    TlsCertificate, KeyAlgorithm, KeyStrength, RsaKeySize, CertificateSubject
};
#[tracing::instrument(name = "Certificate Get Request.")]
pub async fn get_handler() -> impl Responder {
    // TODO: Replace with actual certificate generation/retrieval
    let certificate = TlsCertificate {
        cert_pem: String::from("-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----"),
        key_pem: String::from("-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"),
        chain_pem: None,
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: CertificateSubject {
            organization: Some(String::from("Example Corp")),
            organizational_unit: None,
            country: Some(String::from("US")),
            state_or_province: None,
            locality: None,
            email: None,
        },
        sans: vec![String::from("example.com"), String::from("www.example.com")],
        fingerprint: None,
        valid_from: None,
        expires_at: None,
        created_at: Utc::now(),
    };

    HttpResponse::Ok().json(certificate)
}

pub async fn post_handler() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn put_handler() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn patch_handler() -> impl Responder {
    HttpResponse::NotImplemented()
}


pub async fn delete_handler() -> impl Responder {
    HttpResponse::NotImplemented()  
}