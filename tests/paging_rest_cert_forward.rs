// tests/paging_rest_forward.rs
mod common;
mod common_helpers;

use common::spawn_app;
use common_helpers::{create_n_certificates_via_api, fetch_first_key_algorithm_id};

#[tokio::test]
async fn forward_paging_rest() {
    // Start server (uses your existing spawn_app)
    let test_app = spawn_app().await;
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .expect("client build");
    let health_resp = client
        .get(&format!("{}/health", &test_app))
        .header("connection", "close")
        .send()
        .await;
    match health_resp {
        Ok(r) => {
            println!("Health status: {}", r.status());
            let body = r.text().await.unwrap_or_default();
            println!("Health body: {}", body);
        }
        Err(e) => {
            println!("Health request error: {:?}", e);
        }
    }
    // Discover a valid key algorithm id via REST (server seeds algorithms on startup)
    let key_algorithm_id = fetch_first_key_algorithm_id(&test_app, &client)
        .await
        .expect("No key algorithm found via API");

    // Create test data via POST /certificates (REST only)
    create_n_certificates_via_api(&test_app, &client, &key_algorithm_id, 25).await;

    // First page (newest first)
    let resp1 = client
        .get(&format!(
            "{}/certificates?limit=10&direction=next",
            &test_app
        ))
        .send()
        .await
        .expect("GET /certificates failed");
    assert!(resp1.status().is_success());
    let page1: serde_json::Value = resp1.json().await.unwrap();
    assert_eq!(page1["items"].as_array().unwrap().len(), 10);

    let next_token = page1["nextPageToken"].as_str().map(|s| s.to_string());
    assert!(next_token.is_some(), "Expected nextPageToken on first page");

    // Second page using next token
    let resp2 = client
        .get(&format!(
            "{}/certificates?limit=10&pageToken={}&direction=next",
            &test_app,
            next_token.unwrap()
        ))
        .send()
        .await
        .expect("GET second page failed");
    assert!(resp2.status().is_success());
    let page2: serde_json::Value = resp2.json().await.unwrap();
    assert_eq!(page2["items"].as_array().unwrap().len(), 10);
}
