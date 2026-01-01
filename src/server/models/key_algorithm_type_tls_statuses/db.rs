use crate::server::models::base::BaseModel;

#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithmTypeTlsStatus {
    #[sqlx(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub description: Option<String>,
}
