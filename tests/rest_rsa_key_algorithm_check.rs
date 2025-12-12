mod common;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use common::spawn_app;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use uuid::Uuid;
use skinnycert::server::models::rsa_keys::db::RSAKeyAlgorithm;

#[test]
fn test_generate_key_pair_valid() {
    let algo = RSAKeyAlgorithm {
        id: Uuid::new_v4(),
        display_name: "Test RSA".to_string(),
        algorithm: "RSA".to_string(),
        key_size: 2048,
        deprecated: false,
        created_on: None,
        updated_on: None,
    };

    let result = algo.generate_key_pair();
    assert!(result.is_ok(), "Key generation should succeed");

    let (private_pem, public_pem) = result.unwrap();

    // Check that PEM strings contain expected headers
    assert!(private_pem.contains("BEGIN PRIVATE KEY"));
    assert!(public_pem.contains("BEGIN PUBLIC KEY"));

    // Verify that the public key PEM can be parsed back
    let parsed_rsa = Rsa::public_key_from_pem(public_pem.as_bytes());
    assert!(parsed_rsa.is_ok(), "Public key PEM should be parsable");
}

#[test]
fn test_generate_key_pair_invalid_size() {
    let algo = RSAKeyAlgorithm {
        id: Uuid::new_v4(),
        display_name: "Bad RSA".to_string(),
        algorithm: "RSA".to_string(),
        key_size: 0, // invalid size
        deprecated: false,
        created_on: None,
        updated_on: None,
    };

    let result = algo.generate_key_pair();
    assert!(result.is_err(), "Key generation should fail for invalid size");
}

#[test]
fn test_generate_and_verify_key_pair() {
    let algo = RSAKeyAlgorithm {
        id: Uuid::new_v4(),
        display_name: "Test RSA".to_string(),
        algorithm: "RSA".to_string(),
        key_size: 2048,
        deprecated: false,
        created_on: None,
        updated_on: None,
    };

    let (private_pem, public_pem) = algo.generate_key_pair().unwrap();

    // Verify that the generated pair is valid
    let result = algo.verify_key_pair(private_pem.clone(), public_pem.clone());
    assert!(result.is_ok(), "Generated key pair should verify correctly");
}

#[test]
fn test_verify_key_pair_with_wrong_public_key() {
    let algo = RSAKeyAlgorithm {
        id: Uuid::new_v4(),
        display_name: "Test RSA".to_string(),
        algorithm: "RSA".to_string(),
        key_size: 2048,
        deprecated: false,
        created_on: None,
        updated_on: None,
    };

    let (private_pem, _) = algo.generate_key_pair().unwrap();

    // Generate a different public key to simulate mismatch
    let other_rsa = Rsa::generate(2048).unwrap();
    let other_pkey = PKey::from_rsa(other_rsa).unwrap();
    let wrong_public_pem = String::from_utf8(other_pkey.public_key_to_pem().unwrap()).unwrap();

    let result = algo.verify_key_pair(private_pem, wrong_public_pem);
    assert!(result.is_err(), "Verification should fail with mismatched public key");
}

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
        "Health check did not return a successful status code"
    );

    // The body should not be empty — it should contain JSON data.
    assert!(
        content_length > 0,
        "Health check returned an empty response body"
    );
}
