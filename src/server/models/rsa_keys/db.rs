use chrono::{DateTime, Utc};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};

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

impl RSAKeyAlgorithm {
    pub fn generate_key_pair(&self) -> Result<(String, String), Box<dyn Error>> {
        let rsa = Rsa::generate(self.key_size as u32)?;
        let pkey = PKey::from_rsa(rsa)?;
        // Extract private key PEM
        let private_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;

        // Extract public key PEM
        let public_key_pem = String::from_utf8(pkey.public_key_to_pem()?)?;
        Ok((private_key_pem, public_key_pem))
    }
    pub fn verify_key_pair(
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
