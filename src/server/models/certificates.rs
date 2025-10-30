use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use openssl::pkey::PKey;
use pem::parse;
use std::error::Error;
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

    /// Generates a private key and CSR based on the request parameters
    /// Returns (private_key_pem, csr_pem, public_key_pem)
    pub fn generate_key_and_csr(&self) -> Result<(String, String, String), Box<dyn Error>> {
        // Validate the request first
        self.validate().map_err(|e| -> Box<dyn Error> { e.into() })?;

        // Generate the private key based on algorithm and strength
        use openssl::rsa::Rsa;
        use openssl::ec::{EcKey, EcGroup};
        use openssl::nid::Nid;
        use openssl::pkey::PKey;
        use openssl::x509::{X509Name, X509Req};
        use openssl::hash::MessageDigest;
        
        let pkey = match (&self.key_algorithm, &self.key_strength) {
            (KeyAlgorithm::RSA, KeyStrength::Rsa(rsa_size)) => {
                let rsa = Rsa::generate(rsa_size.as_bits())?;
                PKey::from_rsa(rsa)?
            }
            (KeyAlgorithm::ECDSA, KeyStrength::Ecdsa(curve)) => {
                let group = EcGroup::from_curve_name(match curve {
                    EcdsaCurve::P256 => Nid::X9_62_PRIME256V1,
                    EcdsaCurve::P384 => Nid::SECP384R1,
                    EcdsaCurve::P521 => Nid::SECP521R1,
                })?;
                let ec_key = EcKey::generate(&group)?;
                PKey::from_ec_key(ec_key)?
            }
            _ => return Err("Invalid key algorithm and strength combination".into()),
        };

        // Extract private key PEM
        let private_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;

        // Extract public key PEM
        let public_key_pem = String::from_utf8(pkey.public_key_to_pem()?)?;

        // Build X509 Name (subject)
        let mut name_builder = X509Name::builder()?;
        
        // Use first SAN as Common Name for legacy compatibility
        if let Some(cn) = self.sans.first() {
            name_builder.append_entry_by_text("CN", cn)?;
        }
        
        if let Some(ref org) = self.subject.organization {
            name_builder.append_entry_by_text("O", org)?;
        }
        if let Some(ref ou) = self.subject.organizational_unit {
            name_builder.append_entry_by_text("OU", ou)?;
        }
        if let Some(ref country) = self.subject.country {
            name_builder.append_entry_by_text("C", country)?;
        }
        if let Some(ref state) = self.subject.state_or_province {
            name_builder.append_entry_by_text("ST", state)?;
        }
        if let Some(ref locality) = self.subject.locality {
            name_builder.append_entry_by_text("L", locality)?;
        }
        if let Some(ref email) = self.subject.email {
            name_builder.append_entry_by_text("emailAddress", email)?;
        }
        
        let name = name_builder.build();

        // Create CSR
        let mut req_builder = X509Req::builder()?;
        req_builder.set_subject_name(&name)?;
        req_builder.set_pubkey(&pkey)?;

        // Add Subject Alternative Names (SANs) if present
        if !self.sans.is_empty() {
            use openssl::x509::extension::SubjectAlternativeName;
            let mut san_builder = SubjectAlternativeName::new();
            
            for san in &self.sans {
                // Determine if it's an IP or DNS name
                if san.parse::<std::net::IpAddr>().is_ok() {
                    san_builder.ip(san);
                } else {
                    san_builder.dns(san);
                }
            }
            
            let san_extension = san_builder.build(&req_builder.x509v3_context(None))?;
            let mut stack = openssl::stack::Stack::new()?;
            stack.push(san_extension)?;
            req_builder.add_extensions(&stack)?;
        }

        // Sign the CSR with the private key
        req_builder.sign(&pkey, MessageDigest::sha256())?;
        let req = req_builder.build();

        // Extract CSR PEM
        let csr_pem = String::from_utf8(req.to_pem()?)?;

        Ok((private_key_pem, csr_pem, public_key_pem))
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchCertificateRequest {
    /// PEM-encoded signed certificate from CA
    pub cert_pem: String,
    
    /// Optional PEM-encoded certificate chain
    pub chain_pem: Option<String>,
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
/// Used as the response for all certificate operations
#[derive(Debug, Serialize, Deserialize)]
pub struct TlsCertificate {
    /// Unique identifier
    pub id: Uuid,
    
    /// PEM-encoded Certificate Signing Request (CSR)
    pub csr_pem: String,

    /// PEM-encoded private key
    pub private_key_pem: String,

    /// PEM-encoded public key (extracted from private key)
    pub public_key_pem: String,

    /// PEM-encoded certificate (None until patched with signed cert)
    pub cert_pem: Option<String>,

    /// Optional PEM-encoded certificate chain
    pub chain_pem: Option<String>,

    /// Key algorithm used (RSA or ECDSA)
    pub key_algorithm: KeyAlgorithm,

    /// Key strength used for generation
    pub key_strength: KeyStrength,

    /// Subject details
    pub subject: CertificateSubject,

    /// Subject Alternative Names (SANs)
    pub sans: Vec<String>,

    /// SHA-256 fingerprint (None until cert is patched)
    pub fingerprint: Option<String>,

    /// Certificate validity start time (Not Before)
    /// Only available after certificate is patched
    pub valid_from: Option<DateTime<Utc>>,

    /// Certificate expiration time (Not After)
    /// Only available after certificate is patched
    pub expires_at: Option<DateTime<Utc>>,

    /// When the CSR was created
    pub created_at: DateTime<Utc>,

    /// When the signed certificate was patched (ONE-TIME operation)
    pub cert_uploaded_at: Option<DateTime<Utc>>,
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

/// Trait to extract a validity period from a signed certificate using x509-parser
pub trait CertificateValidity {
    fn extract_validity(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), Box<dyn Error>>;
}

impl CertificateValidity for TlsCertificate {
    fn extract_validity(&self) -> Result<(DateTime<Utc>, DateTime<Utc>), Box<dyn Error>> {
        // Handle the Option properly - return error if cert_pem is None
        let cert_pem_str = self
            .cert_pem
            .as_ref()
            .ok_or("Certificate PEM is not available")?;

        let pem = parse(cert_pem_str.as_bytes())?;

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

/// Trait to extract the public key from a private key PEM
pub trait PublicKeyExtractor {
    fn extract_public_key_pem(&self) -> Result<String, Box<dyn Error>>;
}

impl PublicKeyExtractor for TlsCertificate {
    fn extract_public_key_pem(&self) -> Result<String, Box<dyn Error>> {
        extract_public_key_from_private_key(&self.private_key_pem)
    }
}

/// Extracts the public key in PEM format from a private key PEM string
/// Supports both RSA and ECDSA keys
pub fn extract_public_key_from_private_key(
    private_key_pem: &str,
) -> Result<String, Box<dyn Error>> {
    // Parse the private key PEM
    let pkey = PKey::private_key_from_pem(private_key_pem.as_bytes())?;

    // Export the public key as PEM
    let public_key_pem = pkey.public_key_to_pem()?;

    // Convert to string
    let public_key_str = String::from_utf8(public_key_pem)?;

    Ok(public_key_str)
}

/// Validates that a public key matches a private key
pub fn validate_key_pair(
    private_key_pem: &str,
    public_key_pem: &str,
) -> Result<bool, Box<dyn Error>> {
    let extracted_public = extract_public_key_from_private_key(private_key_pem)?;

    // Normalize whitespace for comparison
    let extracted_normalized = extracted_public.replace("\r\n", "\n").trim().to_string();
    let provided_normalized = public_key_pem.replace("\r\n", "\n").trim().to_string();

    Ok(extracted_normalized == provided_normalized)
}

/// Converts ASN1Time to chrono DateTime<Utc>
fn asn1time_to_datetime(time: &ASN1Time) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let timestamp = time.timestamp();
    DateTime::<Utc>::from_timestamp(timestamp, 0).ok_or_else(|| "Invalid timestamp".into())
}
