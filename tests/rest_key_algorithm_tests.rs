mod common;
use common::spawn_app;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAlgorithmResponse {
    pub id: Uuid,
    pub display_name: String,
    pub key_strength: Option<i32>,
    pub nid_value: Option<i32>,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
    pub algorithm_status: AlgorithmStatusResponse,
    pub algorithm_type: AlgorithmTypeResponse,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgorithmStatusResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgorithmTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub requires_nid: bool,
    pub requires_strength: bool,
    pub tls_status: TlsStatusResponse,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlsStatusResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_on: DateTime<Utc>,
}

#[tokio::test]
async fn get_all_keys_and_generate_key_pair_test() {
    // --- Arrange ---
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // --- Act: GET /keys ---
    let response = client
        .get(&format!("{}/keys", &address))
        .send()
        .await
        .expect("Failed to execute GET /keys");

    assert!(
        response.status().is_success(),
        "GET /keys should return 200 OK"
    );

    let body = response
        .text()
        .await
        .expect("Failed to read GET /keys body");

    assert!(!body.is_empty(), "GET /keys should return non-empty body");

    // Deserialize JSON array of keys
    let keys: Vec<KeyAlgorithmResponse> =
        serde_json::from_str(&body).expect("Failed to deserialize /keys response");

    assert!(
        !keys.is_empty(),
        "Expected at least one key algorithm in the database"
    );

    // --- Act + Assert: For each key, generate a keypair ---
    for key in keys {
        let url = format!("{}/keys/{}/keypair", &address, key.id);

        let resp = client
            .get(&url)
            .send()
            .await
            .expect("Failed to execute GET /keys/{id}/keypair");

        assert!(
            resp.status().is_success(),
            "Keypair generation should return 200 OK for key {}",
            key.id
        );

        let body = resp
            .text()
            .await
            .expect("Failed to read keypair generation response");

        assert!(
            !body.is_empty(),
            "Keypair generation response should not be empty"
        );

        println!("Generated keypair for key {}: {}", key.id, body);
    }
}
