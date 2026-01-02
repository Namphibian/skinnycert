//! Conversion errors used when transforming database models into DTOs.
//!
//! ## Why validate DB → DTO conversions?
//!
//! Even though the database enforces constraints, we treat all data coming from
//! persistent storage as potentially untrusted. This is especially important in
//! a cryptographic system, where malformed, inconsistent, or compromised values
//! could weaken security guarantees or cause unsafe behavior.
//!
//! By validating every field during conversion, we ensure that:
//! - Only well‑formed, expected values reach the API layer.
//! - Corrupted or tampered database entries are detected early.
//! - Domain invariants are enforced consistently.
//! - Unsafe or forbidden cryptographic parameters never propagate outward.
//!
//! This level of defensive validation may seem excessive, but in cryptography
//! it is a deliberate design choice. It is far safer to fail loudly during
//! conversion than to silently accept values that could compromise the system.

use thiserror::Error;


#[derive(Debug, Error)]
pub enum ConversionError {
    /// A required field was missing in the database model.
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// A field contained an invalid or unexpected value.
    #[error("Invalid value for field {0}: {1}")]
    InvalidValue(&'static str, String),

    /// A field failed to parse into the expected type.
    #[error("Failed to parse field {0}: {1}")]
    ParseError(&'static str, String),

    /// A referenced entity was expected but not found.
    #[error("Missing related entity: {0}")]
    MissingRelation(&'static str),

    /// A field contained a value that violates domain rules.
    #[error("Domain violation in field {0}: {1}")]
    DomainViolation(&'static str, String),

    /// A cryptographic parameter was invalid or unsupported.
    #[error("Invalid cryptographic parameter in field {0}: {1}")]
    CryptoParameter(&'static str, String),

    /// A field contained a value outside the allowed range.
    #[error("Out-of-range value for field {0}: {1}")]
    OutOfRange(&'static str, String),

    /// A field contained a value that is inconsistent with other fields.
    #[error("Inconsistent value for field {0}: {1}")]
    Inconsistent(&'static str, String),

    /// A field contained a value that should never appear in a trusted DB.
    #[error("Unexpected or forbidden value for field {0}: {1}")]
    ForbiddenValue(&'static str, String),
}

pub fn validate_optional_str(
    field: &'static str,
    value: &Option<String>,
    max_len: usize,
) -> Result<(), ConversionError> {
    if let Some(v) = value {
        if v.trim().is_empty() {
            return Err(ConversionError::InvalidValue(
                field,
                "Field cannot be empty or whitespace".into(),
            ));
        }

        if v.len() > max_len {
            return Err(ConversionError::OutOfRange(
                field,
                format!("Field exceeds maximum length of {}", max_len),
            ));
        }
    }
    Ok(())
}


pub fn is_valid_dns_name(s: &str) -> bool {
    let dns_regex = regex::Regex::new(r"^[a-zA-Z0-9.-]+$").unwrap();
    dns_regex.is_match(s)
}

pub fn is_valid_ip(s: &str) -> bool {
    s.parse::<std::net::IpAddr>().is_ok()
}


