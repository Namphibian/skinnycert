use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use crate::server::models::certificates::{EcdsaCurve, KeyAlgorithm, RsaKeySize};

/// Database model for the certificates table
#[derive(Debug, FromRow)]
pub struct DbCertificate {
    pub id: Uuid,
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub key_pem: String,
    pub public_key_pem: String,
    pub chain_pem: Option<String>,
    pub key_algorithm: String,
    pub rsa_key_size: Option<String>,
    pub ecdsa_curve: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cert_uploaded_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Database model for certificate SANs
#[derive(Debug, FromRow)]
pub struct DbCertificateSan {
    pub id: Uuid,
    pub certificate_id: Uuid,
    pub san_value: String,
    pub san_order: i32,
    pub created_at: DateTime<Utc>,
}

/// View model with SANs aggregated
#[derive(Debug, FromRow)]
pub struct DbCertificateWithSans {
    pub id: Uuid,
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub key_pem: String,
    pub public_key_pem: String,
    pub chain_pem: Option<String>,
    pub key_algorithm: KeyAlgorithm,
    pub rsa_key_size: Option<RsaKeySize>,
    pub ecdsa_curve: Option<EcdsaCurve>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cert_uploaded_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub sans: Vec<String>,
    pub common_name: Option<String>,
}