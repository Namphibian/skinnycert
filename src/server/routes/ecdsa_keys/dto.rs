use crate::server::models::ecdsa_key::db::EcdsaKeyAlgorithm;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EcdsaKeyAlgorithmResponse {
    pub id: Uuid,
    pub algorithm: String,
    pub display_name: String,
    pub curve_size: i32,
    pub deprecated: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EcdsaKeyAlgorithmPatchRequest {
    pub deprecated: bool,
}

impl TryFrom<EcdsaKeyAlgorithm> for EcdsaKeyAlgorithmResponse {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    fn try_from(value: EcdsaKeyAlgorithm) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            display_name: value.display_name,
            algorithm: value.algorithm,
            curve_size: value.curve_size,
            deprecated: value.deprecated,
            created_on: value.created_on,
            updated_on: value.updated_on,
        })
    }
}