use crate::server::models::key_algorithm_type_tls_statuses::db::KeyAlgorithmTypeTlsStatus;
use crate::server::routes::conversions::ConversionError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct KeyAlgorithmTlsStatusResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
}

impl TryFrom<KeyAlgorithmTypeTlsStatus> for KeyAlgorithmTlsStatusResponse {
    type Error = ConversionError;

    fn try_from(info: KeyAlgorithmTypeTlsStatus) -> Result<Self, Self::Error> {
        if info.name.trim().is_empty() {
            return Err(ConversionError::MissingField("name"));
        }
        if info.description.is_none() {
            return Err(ConversionError::MissingField("description"));
        }

        Ok(KeyAlgorithmTlsStatusResponse {
            id: info.base.id,
            name: info.name,
            description: info.description,
            created_on: info.base.created_on,
        })
    }
}
