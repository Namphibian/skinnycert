use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, FromRow)]
pub struct RSAKeyAlgorithm {
    pub id: Uuid,
    pub display_name: String,
    pub algorithm: String,
    pub rsa_key_size: i32,
    pub created_at: Option<DateTime<Utc>>,
}
