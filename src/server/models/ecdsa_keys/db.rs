use chrono::{DateTime, Utc};
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::PKey;
use sqlx::FromRow;
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

impl EcdsaKeyAlgorithm {
    pub fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
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
}
