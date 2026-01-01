use crate::server::models::key_algorithms::KeyPair;
use chrono::{DateTime, Utc};
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::sign::{Signer, Verifier};
use sqlx::FromRow;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct EcdsaKeyAlgorithm {
    pub id: Uuid,
    pub algorithm: String,
    pub display_name: String,
    pub nid_value: i32,
    pub curve_size: i32,
    pub deprecated: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}
impl KeyPair for EcdsaKeyAlgorithm {
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        let nid = Nid::from_raw(self.nid_value);
        let group = EcGroup::from_curve_name(nid)?;
        let ec_key = EcKey::generate(&group)?;
        // Convert to PKey so we can export PKCS#8
        let pkey = PKey::from_ec_key(ec_key)?;
        // Export private key (PKCS#8)
        let private_pem = pkey.private_key_to_pem_pkcs8()?;
        let private_pem = String::from_utf8(private_pem)?;
        // Export public key (SPKI)
        let public_pem = pkey.public_key_to_pem()?;
        let public_pem = String::from_utf8(public_pem)?;
        Ok((private_pem, public_pem))
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
