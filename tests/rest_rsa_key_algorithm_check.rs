mod common;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use common::spawn_app;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};

use uuid::Uuid;
use skinnycert::server::models::certificates::certificates_model::RsaKeySize;
use skinnycert::server::models::rsa_keys::db::RSAKeyAlgorithm;
use skinnycert::server::routes::rsa_keys::dto::{NewRsaKeyAlgorithmRequest, RsaKeyAlgorithmResponse};

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
async fn post_new_and_get_by_id_key_algorithm_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app().await;
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();
    let body  =  NewRsaKeyAlgorithmRequest {
        rsa_key_size: 65536
    };
    // --- Act ---
    // Send a GET request to the `/keys/rsa` endpoint.
    let response = client
        .post(&format!("{}/keys/rsa", &address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let created: RsaKeyAlgorithmResponse = response
        .json()
        .await
        .expect("Failed to deserialize response body");

    assert_eq!(created.key_size, 65536);

    // Optionally: GET by id to verify persistence
    let get_response = client
        .get(&format!("{}/keys/rsa/{}", &address, created.id))
        .send()
        .await
        .expect("Failed to execute GET request.");

    assert_eq!(get_response.status(), reqwest::StatusCode::OK);
    let fetched: RsaKeyAlgorithmResponse = get_response
        .json()
        .await
        .expect("Failed to deserialize GET response body");

    assert_eq!(fetched.id, created.id);
}