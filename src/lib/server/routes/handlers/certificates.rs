use crate::server::routes::handlers::models::certificate::{
    CertificateGenerationRequest, CertificateSubject, KeyAlgorithm, KeyStrength, RsaKeySize,
    TlsCertificate,
};
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
#[tracing::instrument(name = "Certificate Get Request.")]
pub async fn get_handler() -> impl Responder {
    // TODO: Replace with actual certificate generation/retrieval
    let certificate = TlsCertificate {
        id: Default::default(),
        csr_pem: "".to_string(),
        cert_pem: Some(String::from(
            "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
        )),
        private_key_pem: String::from(
            "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
        ),
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
        public_key_pem: "".to_string(),
        cert_uploaded_at: None,
    };

    HttpResponse::Ok().json(certificate)
}
#[tracing::instrument(name = "Certificate Post Request.")]
pub async fn post_handler(payload: web::Json<CertificateGenerationRequest>) -> impl Responder {
    let request = payload.into_inner();
    
    let (private_key, csr, public_key) = match request.generate_key_and_csr() {
        Ok(keys) => keys,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "error": "Failed to generate key and CSR",
                    "message": e.to_string()
                }));
        }
    };
    
    // Continue with your logic here
    HttpResponse::NotImplemented().into()
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
