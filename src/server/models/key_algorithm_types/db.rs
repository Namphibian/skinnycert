use crate::server::models::base::BaseModel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithmType {
    #[sqlx(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub description: Option<String>,
    pub requires_nid: bool,
    pub requires_strength: bool,
    pub tls_status_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KeyAlgorithmTypeInfo {
    // key_algorithm_types
    pub key_algorithm_type_id: Uuid,
    pub key_algorithm_type_name: String,
    pub key_algorithm_type_description: Option<String>,
    pub key_algorithm_type_requires_nid: bool,
    pub key_algorithm_type_requires_strength: bool,
    pub key_algorithm_type_created_on: DateTime<Utc>,
    pub key_algorithm_type_updated_on: Option<DateTime<Utc>>,
    // key_algorithm_type_tls_statuses
    pub key_algorithm_type_tls_status_id: Uuid,
    pub key_algorithm_type_tls_status_name: String,
    pub key_algorithm_type_tls_status_description: Option<String>,
    pub key_algorithm_type_tls_status_created_on: DateTime<Utc>,
    pub key_algorithm_type_tls_status_updated_on: Option<DateTime<Utc>>,
}
