mod common;

use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use skinnycert::server::models::key_algorithms::KeyPair;
use skinnycert::server::models::rsa_key::db::RSAKeyAlgorithm;
use uuid::Uuid;

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
