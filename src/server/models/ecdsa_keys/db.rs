use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct EcdsaKeyAlgorithm {
    pub id: Uuid,
    pub algorithm: String,
    pub created_at: DateTime<Utc>,
    pub curve: String,
    pub nid_name: String,
    pub nid_value: i32,
    pub display_name: Option<String>,
    pub standard: Option<String>,
    pub deprecated: bool,
}
