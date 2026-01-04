use crate::server::models::certificates::db::CertificateInfo;
use crate::server::routes::conversions::{
    is_valid_dns_name, is_valid_ip, validate_optional_str, ConversionError,
};
use crate::server::routes::key_type_tls_statuses::dto::KeyAlgorithmTlsStatusResponse;
use crate::server::routes::key_types::dto::KeyAlgorithmTypeResponse;
use crate::server::routes::keys::dto::{KeyAlgorithmResponse, KeyAlgorithmStatusResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PemDataResponse {
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub key_pem: String,
    pub public_key_pem: String,
    pub chain_pem: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectDataResponse {
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
pub struct SansDataResponse {
    // SANs
    pub sans: Vec<String>,
    pub common_name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct X509MetadataResponse {
    // Certificate metadata
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateInfoResponse {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub cert_uploaded_on: Option<DateTime<Utc>>,
    pub deleted_on: Option<DateTime<Utc>>,
    pub is_signed: bool,
    pub is_expired: bool,
    pub pem: PemDataResponse,
    pub key_algorithm: KeyAlgorithmResponse,
    pub subject: SubjectDataResponse,
    pub sans: SansDataResponse,
    pub x509: X509MetadataResponse,
}
impl TryFrom<CertificateInfo> for CertificateInfoResponse {
    type Error = ConversionError;
    fn try_from(c: CertificateInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            id: c.id,
            created_on: c.created_on,
            updated_on: c.updated_on,
            cert_uploaded_on: c.cert_uploaded_on,
            deleted_on: c.deleted_on,
            is_signed: c.is_signed,
            is_expired: c.is_expired,
            pem: PemDataResponse {
                csr_pem: c.csr_pem,
                cert_pem: c.cert_pem,
                key_pem: c.key_pem,
                public_key_pem: c.public_key_pem,
                chain_pem: c.chain_pem,
            },
            key_algorithm: KeyAlgorithmResponse {
                id: c.key_algorithm_id,
                display_name: c.key_algorithm_display_name,
                key_strength: Some(c.key_algorithm_key_strength),
                nid_value: c.key_algorithm_nid_value,
                created_on: c.key_algorithm_created_on,
                updated_on: c.key_algorithm_updated_on,
                algorithm_status: KeyAlgorithmStatusResponse {
                    id: c.status_id,
                    name: c.status_name.clone(),
                    description: c.status_description.clone(),
                    created_on: c.status_created_on,
                    updated_on: c.status_updated_on,
                },
                algorithm_type: KeyAlgorithmTypeResponse {
                    id: c.algorithm_type_id,
                    name: c.algorithm_type_name.clone(),
                    description: c.algorithm_type_description,
                    requires_nid: c.algorithm_type_requires_nid,
                    requires_strength: c.algorithm_type_requires_strength,
                    created_on: c.algorithm_type_created_on,
                    updated_on: c.algorithm_type_updated_on,
                    tls_status: KeyAlgorithmTlsStatusResponse {
                        id: c.tls_status_id,
                        name: c.tls_status_name.clone(),
                        description: c.tls_status_description.clone(),
                        created_on: c.tls_status_created_on,
                        updated_on: c.tls_status_updated_on,
                    },
                },
            },
            subject: SubjectDataResponse {
                organization: Some(c.organization),
                organizational_unit: c.organizational_unit,
                country: Some(c.country),
                state_or_province: c.state_or_province,
                locality: c.locality,
                email: c.email,
            },
            sans: SansDataResponse {
                sans: c.sans,
                common_name: c.common_name,
            },
            x509: X509MetadataResponse {
                fingerprint: c.fingerprint,
                valid_from: c.valid_from,
                valid_to: c.valid_to,
            },
        })
    }
}


/// DTO for creating a new certificate
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CertificateSubject {
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCertificateRequest {
    pub key_algorithm_id: Uuid,
    pub subject: CertificateSubject,
    pub sans: Vec<String>,
    #[serde(default = "default_validity_days")]
    pub validity_days: u32,
}

fn default_validity_days() -> u32 {
    365
}
impl CreateCertificateRequest {
    pub fn validate(&self) -> Result<(), ConversionError> {
        // --- SAN VALIDATION ---------------------------------------------------

        if self.sans.is_empty() {
            return Err(ConversionError::DomainViolation(
                "sans",
                "At least one SAN entry is required for DV certificates".into(),
            ));
        }

        for san in &self.sans {
            if san.trim().is_empty() {
                return Err(ConversionError::InvalidValue(
                    "sans",
                    "SAN entries cannot be empty".into(),
                ));
            }

            // DNS or IP allowed
            if !(is_valid_dns_name(san) || is_valid_ip(san)) {
                return Err(ConversionError::InvalidValue(
                    "sans",
                    format!(
                        "Invalid SAN entry it must either be an IP address or a domain: {}",
                        san
                    ),
                ));
            }
        }

        // --- SUBJECT VALIDATION ----------------------------------------------

        validate_optional_str("organization", &self.subject.organization, 256)?;
        validate_optional_str(
            "organizational_unit",
            &self.subject.organizational_unit,
            128,
        )?;
        validate_optional_str("state_or_province", &self.subject.state_or_province, 256)?;
        validate_optional_str("locality", &self.subject.locality, 256)?;
        validate_optional_str("email", &self.subject.email, 256)?;

        // Country must be exactly 2 chars if present
        if let Some(country) = &self.subject.country {
            if country.len() != 2 {
                return Err(ConversionError::InvalidValue(
                    "country",
                    "Country must be a 2‑letter ISO code".into(),
                ));
            }
        }

        // --- VALIDITY DAYS ----------------------------------------------------

        if self.validity_days == 0 {
            return Err(ConversionError::OutOfRange(
                "validity_days",
                "Validity must be at least 1 day".into(),
            ));
        }

        Ok(())
    }
}

/// DTO for patching a certificate with signed cert from CA
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchCertificateDto {
    pub cert_pem: String,
    pub chain_pem: Option<String>,
}


