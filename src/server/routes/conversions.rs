//! The conversion error is used when converting between DB model and DTO objects.
//! Why do we check conversions from the db to dto?
//! This is to protect against invalid data being passed to the client.
//! Since we are dealing with cryptography here we need to protect against a database containing compromised values.
//! Overkill? I hope it is, but it's better to be safe than sorry.

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
    #[error("Invalid value for field {0}: {1}")]
    InvalidValue(&'static str, String),
}