// tests/paging_rest_backward_fixed.rs
mod common;
mod common_helpers;

use common::spawn_app;
use common_helpers::{create_n_certificates_via_api};
use reqwest::Client;
use urlencoding::encode;
use crate::common_helpers::fetch_first_key_algorithm_id;

#[tokio::test]
async fn backward_paging_forward_then_back() {
    // Start server and client
    let test_app = spawn_app().await;
    // wait_for_server_ready(test_app.address) if you have that helper
    let client = Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .expect("client build");

    // Discover a valid key algorithm id via REST
    let key_algorithm_id = fetch_first_key_algorithm_id(&test_app, &client)
        .await
        .expect("No key algorithm found via API");

    // Create 25 certificates via REST
    create_n_certificates_via_api(&test_app, &client, &key_algorithm_id, 25).await;

    // Page 1 (initial, newest-first)
    let resp1 = client
        .get(&format!("{}/certificates?limit=10&direction=next", &test_app))
        .send()
        .await
        .expect("GET page1 failed");
    assert!(resp1.status().is_success());
    let page1: serde_json::Value = resp1.json().await.unwrap();

    // First page must NOT have a prevPageToken (initial page)
    assert!(page1.get("prevPageToken").is_none() || page1["prevPageToken"].is_null());

    // Get nextPageToken to fetch page 2
    let next_token = page1["nextPageToken"].as_str().expect("expected nextPageToken").to_string();
    let next_token_enc = encode(&next_token);

    // Page 2 (forward)
    let resp2 = client
        .get(&format!("{}/certificates?limit=10&pageToken={}&direction=next", &test_app, next_token_enc))
        .send()
        .await
        .expect("GET page2 failed");
    assert!(resp2.status().is_success());
    let page2: serde_json::Value = resp2.json().await.unwrap();

    // Page 2 should have a prevPageToken (because it was produced from a cursor)
    let prev_token = page2["prevPageToken"].as_str().expect("expected prevPageToken").to_string();
    let prev_token_enc = encode(&prev_token);

    // Now request prev (should return the original page1 items)
    let resp_back = client
        .get(&format!("{}/certificates?limit=10&pageToken={}&direction=prev", &test_app, prev_token_enc))
        .send()
        .await
        .expect("GET prev page failed");
    assert!(resp_back.status().is_success());
    let page_back: serde_json::Value = resp_back.json().await.unwrap();

    // Compare items: page_back should equal page1 items
    assert_eq!(page1["items"], page_back["items"], "Backward paging should return the original first page items");
}
