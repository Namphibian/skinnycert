use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use x509_parser::prelude::*;

use std::error::Error;
use pem::parse;

/// Represents the subject details of a TLS certificate (X.509 Distinguished Name)
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateSubject {
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

/// Represents a TLS certificate and its metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct TlsCertificate {
    /// PEM-encoded certificate
    pub cert_pem: String,

    /// PEM-encoded private key
    pub key_pem: String,

    /// Optional PEM-encoded certificate chain
    pub chain_pem: Option<String>,

    /// Subject details (excluding CN)
    pub subject: CertificateSubject,

    /// Subject Alternative Names (SANs)
    pub sans: Vec<String>,

    /// SHA-256 fingerprint
    pub fingerprint: Option<String>,

    /// Certificate validity start time (Not Before)
    pub valid_from: Option<DateTime<Utc>>,

    /// Certificate expiration time (Not After)
    pub expires_at: Option<DateTime<Utc>>,

    /// When the certificate was created in your system
    pub created_at: DateTime<Utc>,
}

/// Trait to derive the Common Name (CN) from the first SAN entry
pub trait HasCommonName {
    fn common_name(&self) -> Option<&str>;
}

impl HasCommonName for TlsCertificate {
    fn common_name(&self) -> Option<&str> {
        self.sans.first().map(|s| s.as_str())
    }
}

/// Trait to extract validity period from a signed certificate using x509-parser
pub trait CertificateValidity {
    fn extract_validity(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), Box<dyn Error>>;
}

impl CertificateValidity for TlsCertificate {
    fn extract_validity(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), Box<dyn Error>> {
        let pem = parse(self.cert_pem.as_bytes())?;
        if pem.tag != "CERTIFICATE" {
            return Err("Not a certificate PEM block".into());
        }

        let (_, cert) = X509Certificate::from_der(&pem.contents)?;
        let not_before = asn1time_to_datetime(cert.validity().not_before())?;
        let not_after = asn1time_to_datetime(cert.validity().not_after())?;

        Ok((not_before, not_after))
    }
}

/// Converts ASN1Time to chrono DateTime<Utc>
fn asn1time_to_datetime(time: &ASN1Time) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let timestamp = time.timestamp()?;
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .ok_or("Invalid timestamp")?;
    Ok(DateTime::<Utc>::from_utc(naive, Utc))
}
