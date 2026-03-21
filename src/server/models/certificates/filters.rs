use crate::server::models::shared::PageDirection;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

/// Filter parameters for querying certificates (cursor‑paging enabled)
#[derive(Debug, Default, Serialize, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CertificateFilterParams {
    pub common_name: Option<String>,
    pub san: Option<String>,

    // Subject fields
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,

    // Algorithm filters
    pub algorithm_type_name: Option<String>,
    pub key_algorithm_display_name: Option<String>,
    pub key_algorithm_key_strength: Option<i32>,
    pub key_algorithm_nid_value: Option<i32>,

    // Status filters
    pub tls_status_name: Option<String>,
    pub status_name: Option<String>,
    pub is_signed: Option<bool>,
    pub is_expired: Option<bool>,

    // Date filters
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub valid_to_after: Option<DateTime<Utc>>,
    pub valid_to_before: Option<DateTime<Utc>>,

    // Identifiers
    pub fingerprint: Option<String>,

    /// Cursor‑based paging
    pub limit: Option<i64>, // default 100
    pub page_token: Option<String>,       // cursor
    pub direction: Option<PageDirection>, // next or prev
}
