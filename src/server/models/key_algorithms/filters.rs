use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAlgorithmFilterParams {
    pub algorithm_type: Option<String>,
    pub tls_status: Option<String>,
    pub algorithm_status: Option<String>,
    pub strength: Option<i32>,
}