use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::server::models::base::PageDirection;

/// Represents a fully expanded certificate record from the `certificate_info` view.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CertificateInfo {
    // Certificate core fields
    pub id: Uuid,
    pub csr_pem: String,
    pub cert_pem: Option<String>,
    pub key_pem: String,
    pub public_key_pem: String,
    pub chain_pem: Option<String>,
    pub key_algorithm_id: Uuid,

    // Expanded algorithm metadata
    pub key_algorithm_display_name: String,
    pub key_algorithm_key_strength: i32,
    pub key_algorithm_nid_value: Option<i32>,
    pub key_algorithm_created_on: DateTime<Utc>,
    pub key_algorithm_updated_on: Option<DateTime<Utc>>,

    // Algorithm status
    pub status_id: Uuid,
    pub status_name: String,
    pub status_description: Option<String>,
    pub status_created_on: DateTime<Utc>,
    pub status_updated_on: Option<DateTime<Utc>>,

    // Algorithm type
    pub algorithm_type_id: Uuid,
    pub algorithm_type_name: String,
    pub algorithm_type_description: Option<String>,
    pub algorithm_type_requires_nid: bool,
    pub algorithm_type_requires_strength: bool,
    pub algorithm_type_created_on: DateTime<Utc>,
    pub algorithm_type_updated_on: Option<DateTime<Utc>>,

    // TLS status
    pub tls_status_id: Uuid,
    pub tls_status_name: String,
    pub tls_status_description: Option<String>,
    pub tls_status_created_on: DateTime<Utc>,
    pub tls_status_updated_on: Option<DateTime<Utc>>,

    // Subject details
    pub organization: String,
    pub organizational_unit: Option<String>,
    pub country: String,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,

    // SANs
    pub sans: Vec<String>,
    pub common_name: Option<String>,

    // Certificate metadata
    pub fingerprint: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,

    // Derived metadata
    pub is_signed: bool,
    pub is_expired: bool,

    // Audit timestamps
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub cert_uploaded_on: Option<DateTime<Utc>>,
    pub deleted_on: Option<DateTime<Utc>>,
}

/// Raw certificate table (not the view)
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

/// SAN table
#[derive(Debug, FromRow)]
pub struct CertificateSan {
    pub id: Uuid,
    pub certificate_id: Uuid,
    pub san_value: String,
    pub san_order: i32,
    pub created_on: DateTime<Utc>,
}

/// Subject fields used for CSR generation
#[derive(Debug, Clone)]
pub struct CertificateSubjectFields {
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

/// CSR generation parameters
#[derive(Debug, Clone)]
pub struct CsrGenerationParams {
    pub subject: CertificateSubjectFields,
    pub sans: Vec<String>,
}


