use crate::server::models::base::BaseModel;

#[derive(Debug, sqlx::FromRow)]
pub struct KeyAlgorithmStatus {
    #[sqlx(flatten)]
    pub base: BaseModel,
    pub name: String,
    pub description: Option<String>,
}
