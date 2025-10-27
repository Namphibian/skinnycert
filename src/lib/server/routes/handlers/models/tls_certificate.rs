use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use std::error::Error;
use pem:: parse;
use x509_parser::prelude::{ASN1Time, FromDer, X509Certificate};

/// Supported key algorithm types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum KeyAlgorithm {
    RSA,
    ECDSA,
}

/// Supported RSA key sizes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RsaKeySize {
    #[serde(rename = "2048")]
    Bits2048,
    #[serde(rename = "3072")]
    Bits3072,
    #[serde(rename = "4096")]
    Bits4096,
}

impl RsaKeySize {
    pub fn as_bits(&self) -> u32 {
        match self {
            RsaKeySize::Bits2048 => 2048,
            RsaKeySize::Bits3072 => 3072,
            RsaKeySize::Bits4096 => 4096,
        }
    }
}

/// Supported ECDSA curves
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EcdsaCurve {
    /// NIST P-256 (secp256r1) - 256-bit security
    #[serde(rename = "P256")]
    P256,
    /// NIST P-384 (secp384r1) - 384-bit security
    #[serde(rename = "P384")]
    P384,
    /// NIST P-521 (secp521r1) - 521-bit security
    #[serde(rename = "P521")]
    P521,
}

impl EcdsaCurve {
    pub fn as_openssl_name(&self) -> &'static str {
        match self {
            EcdsaCurve::P256 => "prime256v1",
            EcdsaCurve::P384 => "secp384r1",
            EcdsaCurve::P521 => "secp521r1",
        }
    }
    
    pub fn security_bits(&self) -> u32 {
        match self {
            EcdsaCurve::P256 => 256,
            EcdsaCurve::P384 => 384,
            EcdsaCurve::P521 => 521,
        }
    }
}

/// Key strength configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyStrength {
    Rsa(RsaKeySize),
    Ecdsa(EcdsaCurve),
}

/// Certificate generation request parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateGenerationRequest {
    /// Key algorithm type (RSA or ECDSA)
    pub key_algorithm: KeyAlgorithm,
    
    /// Key strength (RSA size or ECDSA curve)
    pub key_strength: KeyStrength,
    
    /// Subject details
    pub subject: CertificateSubject,

    /// Subject Alternative Names (SANs)
    /// The first entry will be used as the Common Name (CN) for legacy compatibility
    pub sans: Vec<String>,
    
    /// Certificate validity period in days
    #[serde(default = "default_validity_days")]
    pub validity_days: u32,
}

fn default_validity_days() -> u32 {
    365
}

impl CertificateGenerationRequest {
    /// Validates that the key_algorithm matches the key_strength type
    pub fn validate(&self) -> Result<(), String> {
        match (&self.key_algorithm, &self.key_strength) {
            (KeyAlgorithm::RSA, KeyStrength::Rsa(_)) => Ok(()),
            (KeyAlgorithm::ECDSA, KeyStrength::Ecdsa(_)) => Ok(()),
            (KeyAlgorithm::RSA, KeyStrength::Ecdsa(_)) => {
                Err("RSA algorithm requires RSA key size".to_string())
            }
            (KeyAlgorithm::ECDSA, KeyStrength::Rsa(_)) => {
                Err("ECDSA algorithm requires ECDSA curve".to_string())
            }
        }
    }
}

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

    /// Key algorithm used (RSA or ECDSA)
    pub key_algorithm: KeyAlgorithm,

    /// Key strength used for generation
    pub key_strength: KeyStrength,

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

        // Use the tag() method to get the tag string
        if pem.tag() != "CERTIFICATE" {
            return Err("Not a certificate PEM block".into());
        }

        // Use contents() method to get the DER bytes
        let (_, cert) = X509Certificate::from_der(pem.contents())?;

        // Access not_before and not_after as fields, not methods
        let not_before = asn1time_to_datetime(&cert.validity().not_before)?;
        let not_after = asn1time_to_datetime(&cert.validity().not_after)?;

        Ok((not_before, not_after))
    }
}

/// Converts ASN1Time to chrono DateTime<Utc>
fn asn1time_to_datetime(time: &ASN1Time) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let timestamp = time.timestamp();
    DateTime::<Utc>::from_timestamp(timestamp, 0)
        .ok_or_else(|| "Invalid timestamp".into())
}
