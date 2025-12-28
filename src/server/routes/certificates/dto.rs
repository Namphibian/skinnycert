use crate::server::models::legacy_certificates::certificates_model::{
    CertificateSubject, KeyAlgorithm, KeyStrength,
};
use crate::server::models::legacy_certificates::db::DbCertificateWithSans;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::models::certificates::db::CertificateDetails;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordMetadata {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub cert_uploaded_on: Option<DateTime<Utc>>,
    pub deleted_on: Option<DateTime<Utc>>,
    pub is_signed: bool,
    pub is_expired: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PemData {
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub key_pem: String,
    pub public_key_pem: String,
    pub chain_pem: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAlgorithmData {
    pub id: Uuid,
    pub algorithm: String,
    pub key_size: i32,
    pub display_name: String,
    pub deprecated: bool
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectData{
    // Subject details
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SansData {
    // SANs
    pub sans: Vec<String>,
    pub common_name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct  X509Metadata {
    // Certificate metadata
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateDetailsResponse {
    pub metadata: RecordMetadata,
    pub pem: PemData,
    pub key_algorithm: KeyAlgorithmData,
    pub subject: SubjectData,
    pub sans: SansData,
    pub x509: X509Metadata,
}
impl TryFrom<CertificateDetails> for CertificateDetailsResponse {
    type Error = anyhow::Error;

    fn try_from(c: CertificateDetails) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: RecordMetadata {
                id: c.id,
                created_on: c.created_on,
                updated_on: c.updated_on,
                cert_uploaded_on: c.cert_uploaded_on,
                deleted_on: c.deleted_on,
                is_signed: c.is_signed,
                is_expired: c.is_expired,
            },

            pem: PemData {
                csr_pem: c.csr_pem,
                cert_pem: c.cert_pem,
                key_pem: c.key_pem,
                public_key_pem: c.public_key_pem,
                chain_pem: c.chain_pem,
            },

            key_algorithm: KeyAlgorithmData {
                id: c.key_algorithm_id,
                algorithm: c.algorithm,
                key_size: c.key_size,
                display_name: c.display_name,
                deprecated: c.deprecated,
            },

            subject: SubjectData {
                organization: c.organization,
                organizational_unit: c.organizational_unit,
                country: c.country,
                state_or_province: c.state_or_province,
                locality: c.locality,
                email: c.email,
            },

            sans: SansData {
                sans: c.sans,
                common_name: c.common_name,
            },

            x509: X509Metadata {
                fingerprint: c.fingerprint,
                valid_from: c.valid_from,
                valid_to: c.valid_to,
            },
        })
    }
}


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
    pub valid_to: Option<DateTime<Utc>>,
    pub created_on: DateTime<Utc>,
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
                let size = db_cert
                    .rsa_key_size
                    .ok_or("Missing RSA key size for RSA algorithm")?;
                KeyStrength::Rsa(size)
            }
            KeyAlgorithm::ECDSA => {
                let curve = db_cert
                    .ecdsa_curve
                    .ok_or("Missing ECDSA curve for ECDSA algorithm")?;
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
            valid_to: db_cert.expires_at,
            created_on: db_cert.created_at,
            cert_uploaded_at: db_cert.cert_uploaded_at,
        })
    }
}
