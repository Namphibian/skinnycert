use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::models::rsa_keys::db::RSAKeyAlgorithm;

#[derive(Debug, Serialize, Deserialize)]
pub struct RsaKeyAlgorithmResponse{
    pub id: Uuid,
    pub display_name: String,
    pub algorithm: String,
    pub rsa_key_size: i32,
    pub created_at: Option<DateTime<Utc>>
}

impl TryFrom<RSAKeyAlgorithm> for RsaKeyAlgorithmResponse {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(db_rsa_key: RSAKeyAlgorithm) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_rsa_key.id,
            display_name: db_rsa_key.display_name,
            algorithm: db_rsa_key.algorithm,
            rsa_key_size: db_rsa_key.rsa_key_size,
            created_at: db_rsa_key.created_at
        })
    }
}