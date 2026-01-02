use crate::server::models::key_algorithms::db::KeyAlgorithmInfo;
use crate::server::routes::conversions::ConversionError;
use crate::server::routes::key_type_tls_statuses::dto::KeyAlgorithmTlsStatusResponse;
use crate::server::routes::key_types::dto::KeyAlgorithmTypeResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAlgorithmStatusResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAlgorithmResponse {
    // key_algorithms
    pub id: Uuid,
    pub display_name: String,
    pub key_strength: Option<i32>,
    pub nid_value: Option<i32>,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
    // key_algorithm_statuses
    pub algorithm_status: KeyAlgorithmStatusResponse,
    pub algorithm_type: KeyAlgorithmTypeResponse,
}

impl TryFrom<KeyAlgorithmInfo> for KeyAlgorithmResponse {
    type Error = ConversionError;

    fn try_from(info: KeyAlgorithmInfo) -> Result<Self, Self::Error> {
        // Validate required fields
        if info.status_name.trim().is_empty() {
            return Err(ConversionError::MissingField("status_name"));
        }

        if info.algorithm_type_name.trim().is_empty() {
            return Err(ConversionError::MissingField("algorithm_type_name"));
        }

        if info.tls_status_name.trim().is_empty() {
            return Err(ConversionError::MissingField("tls_status_name"));
        }

        // Validate numeric fields
        if let Some(strength) = info.key_algorithm_strength {
            if strength < 0 {
                return Err(ConversionError::InvalidValue(
                    "key_algorithm_strength",
                    strength.to_string(),
                ));
            }
        }

        Ok(KeyAlgorithmResponse {
            id: info.key_algorithm_id,
            display_name: info.key_algorithm_display_name,
            key_strength: info.key_algorithm_strength,
            nid_value: info.key_algorithm_nid_value,
            created_on: info.key_algorithm_created_on,
            updated_on: info.key_algorithm_updated_on,

            algorithm_status: KeyAlgorithmStatusResponse {
                id: info.status_id,
                name: info.status_name,
                description: info.status_description,
                created_on: info.status_created_on,
                updated_on: info.status_updated_on,
            },

            algorithm_type: KeyAlgorithmTypeResponse {
                id: info.algorithm_type_id,
                name: info.algorithm_type_name,
                description: info.algorithm_type_description,
                requires_nid: info.algorithm_type_requires_nid,
                requires_strength: info.algorithm_type_requires_strength,
                created_on: info.algorithm_type_created_on,
                updated_on: info.algorithm_type_updated_on,

                tls_status: KeyAlgorithmTlsStatusResponse {
                    id: info.tls_status_id,
                    name: info.tls_status_name,
                    description: info.tls_status_description,
                    created_on: info.tls_status_created_on,
                    updated_on: info.tls_status_updated_on,
                },
            },
        })
    }
}
