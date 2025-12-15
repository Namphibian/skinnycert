mod common;

use common::spawn_app;

use skinnycert::server::routes::rsa_keys::dto::{NewRsaKeyAlgorithmRequest, RsaKeyAlgorithmPatchRequest, RsaKeyAlgorithmResponse};

#[tokio::test]
async fn get_rsa_key_algorithm_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app().await;
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();

    // --- Act ---
    // Send a GET request to the `/keys/rsa` endpoint.
    let response = client
        .get(&format!("{}/keys/rsa", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    let status = response.status();
    let content_length = response.content_length().unwrap_or(0);

    // Now consume the response to get the text body
    let response_text = response.text().await.unwrap();
    print!("Response: {}", response_text);
    // --- Assert ---
    // The endpoint should return a successful 2xx response.
    assert!(
        status.is_success(),
        "Get All RSA Keys did not return a successful status code"
    );

    // The body should not be empty — it should contain JSON data.
    assert!(
        content_length > 0,
        "Get All RSA Keys returned an empty response body"
    );
}
#[tokio::test]
async fn post_new_get_patch_and_delete_key_algorithm_test() {
    // --- Arrange ---
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Keep the same POST body you already had
    let body = NewRsaKeyAlgorithmRequest {
        rsa_key_size: 65536,
    };

    // --- Act: Create ---
    let response = client
        .post(&format!("{}/keys/rsa", &address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute POST request.");

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let created: RsaKeyAlgorithmResponse = response
        .json()
        .await
        .expect("Failed to deserialize POST response body");

    // --- Act: Get by ID ---
    let get_response = client
        .get(&format!("{}/keys/rsa/{}", &address, created.id))
        .send()
        .await
        .expect("Failed to execute GET request.");


    assert_eq!(get_response.status(), reqwest::StatusCode::OK);
    let fetched_rsa_key: RsaKeyAlgorithmResponse  = get_response.json().await.expect("Failed to deserialize GET response body");
    assert_eq!(fetched_rsa_key.id, created.id);
    // --- Act: Patch (deprecate) ---
    let patch_body = RsaKeyAlgorithmPatchRequest { deprecated: true };

    let patch_response = client
        .patch(&format!("{}/keys/rsa/{}", &address, created.id))
        .json(&patch_body)
        .send()
        .await
        .expect("Failed to execute PATCH request.");

    assert_eq!(patch_response.status(), reqwest::StatusCode::OK);


    let patched: RsaKeyAlgorithmResponse = patch_response
        .json()
        .await
        .expect("Failed to deserialize PATCH response body");
    assert!(patched.deprecated, "RSA key should be marked deprecated");
    // --- Act: Delete ---
    let delete_response = client
        .delete(&format!("{}/keys/rsa/{}", &address, created.id))
        .send()
        .await
        .expect("Failed to execute DELETE request.");
    assert_eq!(delete_response.status(), reqwest::StatusCode::NO_CONTENT);

    // --- Assert: Confirm deletion ---
    let confirm_response = client
        .get(&format!("{}/keys/rsa/{}", &address, created.id))
        .send()
        .await
        .expect("Failed to execute GET after delete.");

    assert_eq!(confirm_response.status(), reqwest::StatusCode::NOT_FOUND);
}


