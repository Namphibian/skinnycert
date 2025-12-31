use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::server::routes::keys::dto::KeyAlgorithmTlsStatusResponse;

#[derive(Debug, Serialize)]
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