use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, FromRow)]
pub struct RSAKeyAlgorithm {
    pub id: Uuid,
    pub display_name: String,
    pub algorithm: String,
    pub key_size: i32,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>
}
