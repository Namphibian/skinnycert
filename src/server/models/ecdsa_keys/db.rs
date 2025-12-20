use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct EcdsaKeyAlgorithm {
    pub id: Uuid,
    pub algorithm: String,
    pub display_name: String,
    pub nid_value: i32,
    pub curve_size: i32,
    pub deprecated: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

