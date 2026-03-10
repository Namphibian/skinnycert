// tests/common_helpers.rs
use reqwest::Client;
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub async fn fetch_first_key_algorithm_id(base_url: &str, client: &Client) -> Option<String> {
    // Adjust path if your key algorithms route differs
    let resp = client
        .get(&format!("{}/keys?algorithmType=RSA", base_url))
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let body: Value = resp.json().await.ok()?;
    // Expecting list shape: { items: [ { id: "..." , ... }, ... ] } or plain array
    if let Some(items) = body.get("items").and_then(|v| v.as_array()) {
        items
            .get(0)
            .and_then(|it| it.get("id"))
            .and_then(|id| id.as_str())
            .map(|s| s.to_string())
    } else if let Some(arr) = body.as_array() {
        arr.get(0)
            .and_then(|it| it.get("id"))
            .and_then(|id| id.as_str())
            .map(|s| s.to_string())
    } else {
        None
    }
}

pub async fn create_certificate_via_api(
    base_url: &str,
    client: &Client,
    key_algorithm_id: &str,
) -> Value {
    let payload = serde_json::json!({
        "key_algorithm_id": key_algorithm_id,
        "subject":
          {
            "organization": "Integrated Skinny Cert Test",
            "organizational_unit": "IT",
            "country": "AU",
            "state_or_province": "NSW",
            "locality": "Sydney",
            "email": "admin@example.com"
        },
        "sans": [ "example.com", "www.example.com" ],
        "validity_days": 365
    });
    let resp = client
        .post(&format!("{}/certificates", base_url))
        .json(&payload)
        .send()
        .await
        .expect("POST /certificates request failed");

    assert!(
        resp.status().is_success(),
        "POST /certificates failed: {}",
        resp.status()
    );
    resp.json()
        .await
        .expect("Invalid JSON from POST /certificates")
}

pub async fn create_n_certificates_via_api(
    base_url: &str,
    client: &Client,
    key_algorithm_id: &str,
    n: usize,
) {
    for _ in 0..n {
        create_certificate_via_api(base_url, client, key_algorithm_id).await;
        // small delay to help ensure distinct created_on timestamps
        sleep(Duration::from_millis(10)).await;
    }
}
