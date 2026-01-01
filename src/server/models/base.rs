use chrono::{DateTime, Utc};
use uuid::Uuid;
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BaseModel {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

pub trait HasMetadata {
    fn id(&self) -> Uuid;
    fn created_on(&self) -> DateTime<Utc>;
    fn updated_on(&self) -> Option<DateTime<Utc>>;
}

impl HasMetadata for BaseModel {
    fn id(&self) -> Uuid {
        self.id
    }
    fn created_on(&self) -> DateTime<Utc> {
        self.created_on
    }
    fn updated_on(&self) -> Option<DateTime<Utc>> {
        self.updated_on
    }
}
