use crate::server::models::base::BaseModel;
use crate::server::models::key_algorithms::KeyPair;

use chrono::{DateTime, Utc};
use openssl::derive::Deriver;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{Id, PKey};
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithm {
    #[sqlx(flatten)]
    pub base: BaseModel,
    pub algorithm_type_id: Uuid,
    pub status_id: Uuid,
    pub key_strength: Option<i32>,
    pub nid_value: Option<i32>,
    pub display_name: String,
    pub deprecated: bool,
}
#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithmInfo {
    // key_algorithms
    pub key_algorithm_id: Uuid,
    pub key_algorithm_type_id: Uuid,
    pub key_algorithm_status_id: Uuid,
    pub key_algorithm_strength: Option<i32>,
    pub key_algorithm_nid_value: Option<i32>,
    pub key_algorithm_display_name: String,
    pub key_algorithm_created_on: DateTime<Utc>,
    pub key_algorithm_updated_on: Option<DateTime<Utc>>,

    // key_algorithm_types
    pub algorithm_type_id: Uuid,
    pub algorithm_type_name: String,
    pub algorithm_type_description: Option<String>,
    pub algorithm_type_requires_nid: bool,
    pub algorithm_type_requires_strength: bool,
    pub algorithm_type_tls_status_id: Uuid,
    pub algorithm_type_created_on: DateTime<Utc>,
    pub algorithm_type_updated_on: Option<DateTime<Utc>>,

    // key_algorithm_statuses
    pub status_id: Uuid,
    pub status_name: String,
    pub status_description: Option<String>,
    pub status_created_on: DateTime<Utc>,
    pub status_updated_on: Option<DateTime<Utc>>,

    // key_algorithm_type_tls_statuses
    pub tls_status_id: Uuid,
    pub tls_status_name: String,
    pub tls_status_description: Option<String>,
    pub tls_status_created_on: DateTime<Utc>,
    pub tls_status_updated_on: Option<DateTime<Utc>>,
}

impl KeyPair for KeyAlgorithmInfo {
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn Error>> {
        let algo = self.algorithm_type_name.to_uppercase();

        match algo.as_str() {
            "RSA" => {
                let strength = self
                    .key_algorithm_strength
                    .ok_or("RSA requires key_algorithm_strength")?;

                let rsa = Rsa::generate(strength as u32)?;
                let pkey = PKey::from_rsa(rsa)?;

                let private_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;
                let public_key_pem = String::from_utf8(pkey.public_key_to_pem()?)?;

                Ok((private_key_pem, public_key_pem))
            }

            "ECDSA" => {
                let nid_value = self
                    .key_algorithm_nid_value
                    .ok_or("ECDSA requires key_algorithm_nid_value")?;

                let nid = Nid::from_raw(nid_value);
                let group = EcGroup::from_curve_name(nid)?;
                let ec_key = EcKey::generate(&group)?;
                let pkey = PKey::from_ec_key(ec_key)?;

                let private_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;
                let public_pem = String::from_utf8(pkey.public_key_to_pem()?)?;

                Ok((private_pem, public_pem))
            }

            // -----------------------------
            // Ed25519 (signing)
            // -----------------------------
            "ED25519" => {
                let pkey = PKey::generate_ed25519()?;

                let private_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;
                let public_pem = String::from_utf8(pkey.public_key_to_pem()?)?;

                Ok((private_pem, public_pem))
            }

            // -----------------------------
            // X25519 (key exchange)
            // -----------------------------
            "X25519" => {
                let pkey = PKey::generate_x25519()?;

                let private_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;
                let public_pem = String::from_utf8(pkey.public_key_to_pem()?)?;

                Ok((private_pem, public_pem))
            }

            other => Err(format!("Unsupported algorithm type: {}", other).into()),
        }
    }

    fn verify_key_pair(
        &self,
        private_key_pem: String,
        public_key_pem: String,
    ) -> Result<(), Box<dyn Error>> {
        let algo = self.algorithm_type_name.to_uppercase();

        match algo.as_str() {
            "RSA" | "ECDSA" | "ED25519" => {
                let private_key = PKey::private_key_from_pem(private_key_pem.as_bytes())?;
                let public_key = PKey::public_key_from_pem(public_key_pem.as_bytes())?;

                let data = b"Validate the hash of this string by using the public key";

                let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
                signer.update(data)?;
                let signature = signer.sign_to_vec()?;

                let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)?;
                verifier.update(data)?;

                if verifier.verify(&signature)? {
                    Ok(())
                } else {
                    Err("Key pair verification failed".into())
                }
            }

            "X25519" => {
                // Algorithm-appropriate “verification”
                let private_key = PKey::private_key_from_pem(private_key_pem.as_bytes())?;
                let public_key = PKey::public_key_from_pem(public_key_pem.as_bytes())?;

                if private_key.id() != Id::X25519 || public_key.id() != Id::X25519 {
                    return Err("Keys are not X25519".into());
                }

                let mut deriver = Deriver::new(&private_key)?;
                deriver.set_peer(&public_key)?;
                let secret = deriver.derive_to_vec()?;

                if secret.is_empty() {
                    Err("X25519 key agreement failed".into())
                } else {
                    Ok(())
                }
            }

            other => Err(format!("Unsupported algorithm type: {}", other).into()),
        }
    }
}
