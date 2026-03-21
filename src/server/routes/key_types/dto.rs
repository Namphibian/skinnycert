use crate::server::models::key_algorithm_types::db::KeyAlgorithmTypeInfo;
use crate::server::routes::conversions::ConversionError;
use crate::server::routes::key_type_tls_statuses::dto::KeyAlgorithmTlsStatusResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyAlgorithmTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub requires_nid: bool,
    pub requires_strength: bool,
    pub tls_status: KeyAlgorithmTlsStatusResponse,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

impl TryFrom<KeyAlgorithmTypeInfo> for KeyAlgorithmTypeResponse {
    type Error = ConversionError;

    fn try_from(info: KeyAlgorithmTypeInfo) -> Result<Self, Self::Error> {
        if info.key_algorithm_type_name.trim().is_empty() {
            return Err(ConversionError::MissingField("algorithm_type_name"));
        }

        if info.key_algorithm_type_tls_status_name.trim().is_empty() {
            return Err(ConversionError::MissingField(
                "key_algorithm_type_tls_status_name",
            ));
        }

        Ok(KeyAlgorithmTypeResponse {
            id: info.key_algorithm_type_id,
            name: info.key_algorithm_type_name,
            description: info.key_algorithm_type_description,
            requires_nid: info.key_algorithm_type_requires_nid,
            requires_strength: info.key_algorithm_type_requires_strength,
            created_on: info.key_algorithm_type_created_on,
            updated_on: info.key_algorithm_type_updated_on,
            tls_status: KeyAlgorithmTlsStatusResponse {
                id: info.key_algorithm_type_tls_status_id,
                name: info.key_algorithm_type_tls_status_name,
                description: info.key_algorithm_type_tls_status_description,
                created_on: info.key_algorithm_type_tls_status_created_on,
                updated_on: info.key_algorithm_type_tls_status_updated_on,
            },
        })
    }
}
