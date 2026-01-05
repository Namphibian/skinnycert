use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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


#[derive(Debug, Clone)]
pub struct CertificateSubjectFields {
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CsrGenerationParams {
    pub subject: CertificateSubjectFields,
    pub sans: Vec<String>,
}

/// ```rust
/// Represents a set of filterable parameters for querying and identifying certificates.
///
/// This structure provides various optional fields that can be used to apply filters
/// to certificate queries based on attributes such as the certificate's subject,
/// algorithms, status, creation/expiration dates, and unique identifiers.
/// All fields are optional, allowing partial or targeted filtering when searching for
/// certificates.
///
/// # Fields
///
/// - `common_name`:
///   An optional string filter based on the certificate's Common Name (CN).
///
/// - `san`:
///   An optional string filter based on the Subject Alternative Name (SAN) of the certificate.
///
/// ## Subject Fields
///
/// - `organization`:
///   An optional string filter for the Organization (O) attribute in the certificate's subject.
/// - `organizational_unit`:
///   An optional string filter for the Organizational Unit (OU) attribute in the certificate's subject.
/// - `country`:
///   An optional string filter for the Country (C) attribute in the certificate's subject.
/// - `state_or_province`:
///   An optional string filter for the State or Province (ST) attribute in the certificate's subject.
/// - `locality`:
///   An optional string filter for the Locality (L) attribute in the certificate's subject.
/// - `email`:
///   An optional string filter for the email address included in the certificate.
///
/// ## Algorithm Filters
///
/// - `algorithm_type_name`:
///   An optional string filter based on the type name of the certificate's signature algorithm.
/// - `key_algorithm_display_name`:
///   An optional string filter based on the display name of the key algorithm.
/// - `key_algorithm_key_strength`:
///   An optional integer filter for the key strength (in bits) of the key algorithm.
/// - `key_algorithm_nid_value`:
///   An optional integer filter for the NID (Numeric Identifier) value of the key algorithm.
///
/// ## Status Filters
///
/// - `tls_status_name`:
///   An optional string filter for the TLS status of the certificate.
/// - `status_name`:
///   An optional string filter for the general status of the certificate.
/// - `is_signed`:
///   An optional boolean filter to indicate if the certificate is signed or unsigned.
/// - `is_expired`:
///   An optional boolean filter to indicate if the certificate is expired or not.
///
/// ## Date Filters
///
/// - `created_after`:
///   An optional filter to include only certificates created after the specified date and time.
/// - `created_before`:
///   An optional filter to include only certificates created before the specified date and time.
/// - `valid_to_after`:
///   An optional filter to include only certificates whose validity end date is after the specified date and time.
/// - `valid_to_before`:
///   An optional filter to include only certificates whose validity end date is before the specified date and time.
///
/// ## Identifiers
///
/// - `fingerprint`:
///   An optional string filter to match the certificate's fingerprint.
/// - `id`:
///   An optional UUID filter to match the certificate's unique identifier.
///
/// # Derive Attributes
/// - `Debug`:
///   Enables formatting of the structure for debugging purposes.
/// - `Default`:
///   Provides a default implementation for the structure, initializing all fields as `None`.
/// - `Serialize` & `Deserialize`:
///   Enables serialization and deserialization of the structure, facilitating data exchange.
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CertificateFilterParams {
    pub common_name: Option<String>,
    pub san: Option<String>,

    // Subject fields
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,

    // Algorithm filters
    pub algorithm_type_name: Option<String>,
    pub key_algorithm_display_name: Option<String>,
    pub key_algorithm_key_strength: Option<i32>,
    pub key_algorithm_nid_value: Option<i32>,

    // Status filters
    pub tls_status_name: Option<String>,
    pub status_name: Option<String>,
    pub is_signed: Option<bool>,
    pub is_expired: Option<bool>,

    // Date filters
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub valid_to_after: Option<DateTime<Utc>>,
    pub valid_to_before: Option<DateTime<Utc>>,

    // Identifiers
    pub fingerprint: Option<String>,

    pub limit: Option<i64>, // default 100
    pub offset: Option<i64>, // default 0
}
