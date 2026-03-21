use crate::server::models::key_algorithm_statuses::db::KeyAlgorithmStatus;
use crate::server::routes::conversions::ConversionError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(as = KeyStatusResponse)]
pub struct KeyAlgorithmStatusResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

impl TryFrom<KeyAlgorithmStatus> for KeyAlgorithmStatusResponse {
    type Error = ConversionError;

    fn try_from(value: KeyAlgorithmStatus) -> Result<Self, Self::Error> {
        if value.name.is_empty() {
            return Err(ConversionError::MissingField("name"));
        }
        if value.description.is_none() {
            return Err(ConversionError::MissingField("description"));
        }

        Ok(Self {
            id: value.base.id,
            name: value.name,
            description: value.description,
            created_on: value.base.created_on,
            updated_on: value.base.updated_on,
        })
    }
}
