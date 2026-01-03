mod common;
use common::spawn_app;
use serde::Deserialize;
use skinnycert::server::routes::keys::dto::KeyAlgorithmResponse;

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

    let keys: Vec<KeyAlgorithmResponse> = response
        .json()
        .await
        .expect("Failed to deserialize /keys response");

    assert!(!keys.is_empty(), "Expected at least one key algorithm");

    // --- Act + Assert: For each key ---
    for key in &keys {
        println!("Testing key: {:?}", key);

        // ---------------------------------------------------------
        // 1. Keypair generation test
        // ---------------------------------------------------------
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

        let body = resp.text().await.expect("Failed to read keypair response");
        assert!(!body.is_empty(), "Keypair response should not be empty");
        println!("Keypair response: {}", body);
        // ---------------------------------------------------------
        // 2. Filter tests (inside the loop)
        // ---------------------------------------------------------

        // Filter by algorithm_type_name
        let resp = client
            .get(&format!(
                "{}/keys?algorithm_type={}",
                &address, key.algorithm_type.name
            ))
            .send()
            .await
            .expect("Failed to GET /keys?algorithm_type");

        assert!(
            resp.status().is_success(),
            "Filter by algorithm_type_name failed for key {}",
            key.id
        );
        println!("Filter by algorithm_type_name: {:?}", resp);
        let filtered: Vec<KeyAlgorithmResponse> = resp
            .json()
            .await
            .expect("Failed to deserialize filtered keys");

        assert!(
            !filtered.is_empty(),
            "Filtering by algorithm_type_name should return >= 1 row"
        );

        // Filter by tls_status_name
        let resp = client
            .get(&format!(
                "{}/keys?tls_status={}",
                &address, key.algorithm_type.tls_status.name
            ))
            .send()
            .await
            .expect("Failed to GET /keys?tls_status");

        assert!(
            resp.status().is_success(),
            "Filter by tls_status failed for key {}",
            key.id
        );
        println!("Filter by tls_status: {:?}", resp);
        let filtered: Vec<KeyAlgorithmResponse> = resp
            .json()
            .await
            .expect("Failed to deserialize filtered keys");

        assert!(
            !filtered.is_empty(),
            "Filtering by tls_status should return >= 1 row"
        );

        // Filter by algorithm_status (status_name)
        let resp = client
            .get(&format!(
                "{}/keys?algorithm_status={}",
                &address, key.algorithm_status.name
            ))
            .send()
            .await
            .expect("Failed to GET /keys?algorithm_status");

        assert!(
            resp.status().is_success(),
            "Filter by algorithm_status failed for key {}",
            key.id
        );
        println!("Filter by algorithm_status: {:?}", resp);
        let filtered: Vec<KeyAlgorithmResponse> = resp
            .json()
            .await
            .expect("Failed to deserialize filtered keys");

        assert!(
            !filtered.is_empty(),
            "Filtering by algorithm_status should return >= 1 row"
        );

        // Filter by strength
        if let Some(strength) = key.key_strength {
            let resp = client
                .get(&format!("{}/keys?strength={}", &address, strength))
                .send()
                .await
                .expect("Failed to GET /keys?strength");
            assert!(
                resp.status().is_success(),
                "Filter by strength failed for key {}",
                key.id
            );
            let filtered: Vec<KeyAlgorithmResponse> = resp
                .json()
                .await
                .expect("Failed to deserialize filtered keys");
            assert!(
                !filtered.is_empty(),
                "Filtering by strength should return >= 1 row"
            );
        } else {
            println!(
                "Skipping strength filter test for key {} (no strength)",
                key.id
            );
        }
        // ---------------------------------------------------------
        // 3. Combined filter test (all filters together)
        // ---------------------------------------------------------
        let url = format!(
            "{}/keys?algorithm_type={}&tls_status={}&algorithm_status={}",
            &address,
            key.algorithm_type.name,
            key.algorithm_type.tls_status.name,
            key.algorithm_status.name,
        ); // Only include strength if present if let Some(strength) = key.key_strength { url.push_str(&format!("&strength={}", strength)); }

        let resp = client
            .get(&url)
            .send()
            .await
            .expect("Failed to GET /keys with combined filters");

        assert!(
            resp.status().is_success(),
            "Combined filter request should return 200 OK for key {}",
            key.id
        );
        println!("Combined filter response: {:?}", resp);
        let filtered: Vec<KeyAlgorithmResponse> = resp
            .json()
            .await
            .expect("Failed to deserialize combined filter response");

        assert!(
            !filtered.is_empty(),
            "Combined filter should return >= 1 row"
        );
    }
}
