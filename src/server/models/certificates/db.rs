use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Certificate {
    pub id: Uuid,

    // PEM data
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub key_pem: String,
    pub public_key_pem: String,
    pub chain_pem: Option<String>,

    // Polymorphic key algorithm reference
    pub key_algorithm_id: Uuid,

    // Subject details
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,

    // Certificate metadata
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,

    // Audit timestamps
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub cert_uploaded_on: Option<DateTime<Utc>>,
    pub deleted_on: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
pub struct CertificateSan {
    pub id: Uuid,
    pub certificate_id: Uuid,
    pub san_value: String,
    pub san_order: i32,
    pub created_on: DateTime<Utc>,
}
