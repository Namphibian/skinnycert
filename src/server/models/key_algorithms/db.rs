use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::models::base::BaseModel;

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

#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithmStatus {
    #[sqlx(flatten)]
    pub base: BaseModel,

    pub name: String,
    pub description: Option<String>,
}
#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithmTypeTlsStatus {
    #[sqlx(flatten)]
    pub base: BaseModel,

    pub name: String,
    pub description: Option<String>,
}
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
    pub key_algorithm_deprecated: bool,
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


