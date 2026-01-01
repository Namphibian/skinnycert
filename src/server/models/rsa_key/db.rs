//! The `RSAKeyAlgorithm` struct represents metadata and operations for RSA key generation and validation.
//!
//! # Fields
//!
//! * `id` - A unique identifier for the RSA key algorithm instance, represented as a `Uuid`.
//! * `display_name` - A human-readable name for the RSA key algorithm.
//! * `algorithm` - The name of the cryptographic algorithm (e.g., "RSA").
//! * `key_size` - The size of the RSA key (e.g., 1024, 2048, etc.), stored as a 32-bit integer.
//! * `deprecated` - A boolean value indicating whether this key algorithm is deprecated.
//! * `created_on` - An optional timestamp indicating when the key algorithm was created.
//! * `updated_on` - An optional timestamp indicating when the key algorithm was last updated.
//!
//! # Methods
//!
//! ## `generate_key_pair`
//!
//! Generates a new RSA key pair.
//!
//! ### Returns
//!
//! * `Ok((String, String))` - A tuple containing the private key in PEM format and the public key in PEM format, as UTF-8 encoded strings.
//! * `Err(Box<dyn Error>)` - An error if key generation fails.
//!
//! ### Example
//!
//! ```
//! let rsa_algorithm = RSAKeyAlgorithm {
//!     id: Uuid::new_v4(),
//!     display_name: String::from("Example RSA"),
//!     algorithm: String::from("RSA"),
//!     key_size: 2048,
//!     deprecated: false,
//!     created_on: None,
//!     updated_on: None,
//! };
//! let (private_key, public_key) = rsa_algorithm.generate_key_pair().unwrap();
//! println!("Private Key: {}", private_key);
//! println!("Public Key: {}", public_key);
//! ```
//!
//! ## `verify_key_pair`
//!
//! Verifies the validity of an RSA key pair by signing and verifying sample data.
//!
//! ### Parameters
//!
//! * `private_key_pem` - A `String` containing the private key in PEM format.
//! * `public_key_pem` - A `String` containing the public key in PEM format.
//!
//! ### Returns
//!
//! * `Ok(())` - If the key pair is valid and the public key can correctly verify data signed with the private key.
//! * `Err(Box<dyn Error>)` - If verification fails or an error occurs.
//!
//! ### Example
//!
//! ```
//! let rsa_algorithm = RSAKeyAlgorithm {
//!     id: Uuid::new_v4(),
//!     display_name: String::from("Example RSA"),
//!     algorithm: String::from("RSA"),
//!     key_size: 2048,
//!     deprecated: false,
//!     created_on: None,
//!     updated_on: None,
//! };
//! let (private_key, public_key) = rsa_algorithm.generate_key_pair().unwrap();
//! rsa_algorithm.verify_key_pair(private_key.clone(), public_key.clone()).unwrap();
//! println!("Key pair is valid!");
//! ```
//!
//! ### Errors
//!
//! * Returns an error if:
//!   - The keys are invalid or mismatched.
//!   - Key parsing from PEM format fails.
//!   - Signing or verification of the sample data fails.
use chrono::{DateTime, Utc};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};


use crate::server::models::key_algorithms::KeyPair;
use sqlx::FromRow;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct RSAKeyAlgorithm {
    pub id: Uuid,
    pub display_name: String,
    pub algorithm: String,
    pub key_size: i32,
    pub deprecated: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

impl KeyPair for RSAKeyAlgorithm {
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn Error>> {
        let rsa = Rsa::generate(self.key_size as u32)?;
        let pkey = PKey::from_rsa(rsa)?;
        // Extract private key PEM
        let private_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;

        // Extract public key PEM
        let public_key_pem = String::from_utf8(pkey.public_key_to_pem()?)?;
        Ok((private_key_pem, public_key_pem))
    }

    fn verify_key_pair(
        &self,
        private_key_pem: String,
        public_key_pem: String,
    ) -> Result<(), Box<dyn Error>> {
        let private_key = PKey::private_key_from_pem(private_key_pem.as_bytes())?;
        let public_key = PKey::public_key_from_pem(public_key_pem.as_bytes())?;
        // Sign some data with the private key
        let data = b"Validate the hash of this string by using the public key";
        let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
        signer.update(data)?;
        let signature = signer.sign_to_vec()?;

        // Verify the signature with the public key
        let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)?;
        verifier.update(data)?;
        if verifier.verify(&signature)? {
            Ok(())
        } else {
            Err("Key pair verification failed".into())
        }
    }
}
