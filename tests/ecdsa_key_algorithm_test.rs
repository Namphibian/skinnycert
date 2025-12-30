use openssl::nid::Nid;
use openssl::pkey::PKey;
use skinnycert::server::models::ecdsa_key::db::EcdsaKeyAlgorithm;
use uuid::Uuid;
use skinnycert::server::models::key_algorithms::KeyAlgorithm;

#[test]
fn test_generate_ecdsa_key_pair() {
    // Use a well‑known curve (P‑256)
    let algo = EcdsaKeyAlgorithm {
        id: Uuid::new_v4(),
        algorithm: "ECDSA".into(),
        display_name: "Test Curve".into(),
        nid_value: Nid::X9_62_PRIME256V1.as_raw(),
        curve_size: 256,
        deprecated: false,
        created_on: None,
        updated_on: None,
    };
    let result = algo.generate_key_pair();
    assert!(result.is_ok(), "Key generation should succeed");
    let (private_pem, public_pem) = result.unwrap();
    // Basic sanity checks
    assert!(
        !private_pem.is_empty(),
        "Private key PEM should not be empty"
    );
    assert!(!public_pem.is_empty(), "Public key PEM should not be empty");
    // Validate private key parses as PKCS#8
    let private_key = PKey::private_key_from_pem(private_pem.as_bytes());
    assert!(
        private_key.is_ok(),
        "Private key should be valid PKCS#8 PEM"
    );
    // Validate public key parses as SPKI
    let public_key = PKey::public_key_from_pem(public_pem.as_bytes());
    assert!(public_key.is_ok(), "Public key should be valid SPKI PEM");
    let match_result = algo.verify_key_pair(private_pem.clone(), public_pem.clone());
    assert!(match_result.is_ok(), "Key pair verification should succeed");
}
