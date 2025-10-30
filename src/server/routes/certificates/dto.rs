use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::models::certificates::{
    CertificateSubject, KeyAlgorithm, KeyStrength, RsaKeySize, EcdsaCurve
};
use crate::server::models::db_certificate::DbCertificateWithSans;

/// DTO for certificate response (sent to clients)
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateResponseDto {
    pub id: Uuid,
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub chain_pem: Option<String>,
    pub key_algorithm: KeyAlgorithm,
    pub key_strength: KeyStrength,
    pub subject: CertificateSubject,
    pub sans: Vec<String>,
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub cert_uploaded_at: Option<DateTime<Utc>>,
}

/// DTO for creating a new certificate
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCertificateDto {
    pub key_algorithm: KeyAlgorithm,
    pub key_strength: KeyStrength,
    pub subject: CertificateSubject,
    pub sans: Vec<String>,
    #[serde(default = "default_validity_days")]
    pub validity_days: u32,
}

fn default_validity_days() -> u32 {
    365
}

/// DTO for patching a certificate with signed cert from CA
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchCertificateDto {
    pub cert_pem: String,
    pub chain_pem: Option<String>,
}

/// Convert DbCertificateWithSans to CertificateResponseDto
impl From<DbCertificateWithSans> for CertificateResponseDto {
    fn from(db_cert: DbCertificateWithSans) -> Self {
        let key_strength = match db_cert.key_algorithm {

            key_algorithm @ KeyAlgorithm::RSA => {
                let size = match db_cert.rsa_key_size.as_deref() {
                    Some("2048") => RsaKeySize::Bits2048,
                    Some("3072") => RsaKeySize::Bits3072,
                    Some("4096") => RsaKeySize::Bits4096,
                    _ => RsaKeySize::Bits2048, // default
                };
                KeyStrength::Rsa(size)
            }
           key_algorithm @ KeyAlgorithm::ECDSA => {
                let curve = match db_cert.ecdsa_curve.as_deref() {
                    Some("P256") => EcdsaCurve::P256,
                    Some("P384") => EcdsaCurve::P384,
                    Some("P521") => EcdsaCurve::P521,
                    _ => EcdsaCurve::P256, // default
                };
                KeyStrength::Ecdsa(curve)
            }
            _ => KeyStrength::Rsa(RsaKeySize::Bits2048), // fallback
        };

        let key_algorithm = match db_cert.key_algorithm {
            KeyAlgorithm::RSA => KeyAlgorithm::RSA,
            KeyAlgorithm::ECDSA => KeyAlgorithm::ECDSA,
            _ => KeyAlgorithm::RSA, // fallback
        };

        Self {
            id: db_cert.id,
            csr_pem: db_cert.csr_pem,
            cert_pem: db_cert.cert_pem,
            chain_pem: db_cert.chain_pem,
            key_algorithm,
            key_strength,
            subject: CertificateSubject {
                organization: db_cert.organization,
                organizational_unit: db_cert.organizational_unit,
                country: db_cert.country,
                state_or_province: db_cert.state_or_province,
                locality: db_cert.locality,
                email: db_cert.email,
            },
            sans: db_cert.sans,
            fingerprint: db_cert.fingerprint,
            valid_from: db_cert.valid_from,
            expires_at: db_cert.expires_at,
            created_at: db_cert.created_at,
            cert_uploaded_at: db_cert.cert_uploaded_at,
        }
    }
}