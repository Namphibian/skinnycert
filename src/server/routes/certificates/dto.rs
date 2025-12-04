use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::models::certificates::certificates_model::{CertificateSubject, KeyAlgorithm, KeyStrength};
use crate::server::models::certificates::db::DbCertificateWithSans;

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
    pub common_name: String,
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
impl TryFrom<DbCertificateWithSans> for CertificateResponseDto {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(db_cert: DbCertificateWithSans) -> Result<Self, Self::Error> {
        let key_strength = match db_cert.key_algorithm {
            KeyAlgorithm::RSA => {
                let size = db_cert.rsa_key_size.ok_or("Missing RSA key size for RSA algorithm")?;
                KeyStrength::Rsa(size)
            }
            KeyAlgorithm::ECDSA => {
                let curve = db_cert.ecdsa_curve.ok_or("Missing ECDSA curve for ECDSA algorithm")?;
                KeyStrength::Ecdsa(curve)
            }
        };

        Ok(Self {
            id: db_cert.id,
            csr_pem: db_cert.csr_pem,
            cert_pem: db_cert.cert_pem,
            chain_pem: db_cert.chain_pem,
            key_algorithm: db_cert.key_algorithm,
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
            common_name: db_cert.common_name.unwrap_or_default(),
            fingerprint: db_cert.fingerprint,
            valid_from: db_cert.valid_from,
            expires_at: db_cert.expires_at,
            created_at: db_cert.created_at,
            cert_uploaded_at: db_cert.cert_uploaded_at,
        })
    }
}

