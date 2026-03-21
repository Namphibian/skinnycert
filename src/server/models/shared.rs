use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::server::models::responses::RepositoryError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

/// Paging direction for cursor-based pagination
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PageDirection {
    Next,
    Prev,
}
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct BaseModel {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRequest {
    pub limit: Option<i64>,
    pub page_token: Option<String>,
}

/// Use a separator that cannot appear in an RFC3339 timestamp.
/// '|' is safe for this purpose.
pub fn encode_cursor(created_on: DateTime<Utc>, id: Uuid) -> String {
    let raw = format!("{}|{}", created_on.to_rfc3339(), id);
    URL_SAFE_NO_PAD.encode(raw)
}

pub fn decode_cursor(token: &str) -> Result<(DateTime<Utc>, Uuid), RepositoryError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(token)
        .map_err(|_| RepositoryError::InvalidToken)?;

    let decoded_str = String::from_utf8(decoded).map_err(|_| RepositoryError::InvalidToken)?;

    // Split on '|' which will not appear in the RFC3339 timestamp.
    let mut parts = decoded_str.splitn(2, '|');

    let ts_str = parts.next().ok_or(RepositoryError::InvalidToken)?;
    let id_str = parts.next().ok_or(RepositoryError::InvalidToken)?;

    // Parse RFC3339 timestamp into DateTime<Utc>
    let ts = DateTime::parse_from_rfc3339(ts_str)
        .map_err(|_| RepositoryError::InvalidTimestamp)?
        .with_timezone(&Utc);

    let id = Uuid::parse_str(id_str).map_err(|_| RepositoryError::InvalidUuid)?;

    Ok((ts, id))
}
/// Subject fields used for CSR generation
#[derive(Debug, Clone)]
pub struct CertificateSubjectFields {
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

/// CSR generation parameters
#[derive(Debug, Clone)]
pub struct CsrGenerationParams {
    pub subject: CertificateSubjectFields,
    pub sans: Vec<String>,
}
