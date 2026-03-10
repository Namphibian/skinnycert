// tests/paging_rest_roundtrip_fixed.rs
mod common;
mod common_helpers;

use common::spawn_app;
use common_helpers::{create_n_certificates_via_api, };
use reqwest::Client;
use sqlx::Encode;
use urlencoding::encode;
use crate::common_helpers::fetch_first_key_algorithm_id;

#[tokio::test]
async fn round_trip_forward_backward() {
    let test_app = spawn_app().await;
    let client = Client::builder()
        .pool_max_idle_per_host(0)
        .build()
        .expect("client build");

    let key_algorithm_id = fetch_first_key_algorithm_id(&test_app, &client)
        .await
        .expect("No key algorithm found via API");

    // Create 40 certificates
    create_n_certificates_via_api(&test_app, &client, &key_algorithm_id, 40).await;

    // Page 1
    let resp1 = client
        .get(&format!("{}/certificates?limit=10&direction=next", &test_app))
        .send()
        .await
        .expect("GET page1 failed");
    let page1: serde_json::Value = resp1.json().await.unwrap();
    let next1 = page1["nextPageToken"].as_str().expect("expected nextPageToken").to_string();

    // Page 2
    let resp2 = client
        .get(&format!("{}/certificates?limit=10&pageToken={}&direction=next", &test_app, encode(&next1)))
        .send()
        .await
        .expect("GET page2 failed");
    let page2: serde_json::Value = resp2.json().await.unwrap();
    let prev2 = page2["prevPageToken"].as_str().expect("expected prevPageToken").to_string();

    // Back to Page 1 using prev token from page2
    let resp_back = client
        .get(&format!("{}/certificates?limit=10&pageToken={}&direction=prev", &test_app, encode(&prev2)))
        .send()
        .await
        .expect("GET page1 again failed");
    let page1_again: serde_json::Value = resp_back.json().await.unwrap();

    assert_eq!(page1["items"], page1_again["items"], "Round trip should return same items");
}
