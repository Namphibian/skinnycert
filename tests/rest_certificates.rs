mod common;
use common::spawn_app;
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::extension::{BasicConstraints, KeyUsage};
use openssl::x509::{X509NameBuilder, X509};
use skinnycert::server::routes::certificates::dto::{
    CertificateInfoResponse, CertificateSubject, CreateCertificateRequest,
    PatchCertificateRequest,
};
use skinnycert::server::routes::conversions::ConversionError;
use skinnycert::server::routes::keys::dto::KeyAlgorithmResponse;

#[tokio::test]
async fn get_certificates_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app().await;
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();

    // --- Act ---
    // Send a GET request to the `/health` endpoint.
    let response = client
        .get(&format!("{}/certificates", &address))
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
        "Get all certificates did not return a successful status code"
    );

    // The body should not be empty — it should contain JSON data.
    assert!(
        content_length > 0,
        "Get all certificates did not return any content"
    );
}
#[tokio::test]
async fn put_certificates_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app().await;
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();

    // --- Act ---
    // Send a PATCH request to the `/health` endpoint.
    let response = client
        .put(&format!("{}/certificates", &address))
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
        status.is_client_error(),
        "Get all certificates did not return a successful status code"
    );
}

fn create_test_cert(
    common_name: &str,
    issuer: Option<(&X509, &PKey<Private>)>,
    is_ca: bool,
) -> (X509, PKey<Private>) {
    let rsa = Rsa::generate(2048).unwrap();
    let privkey = PKey::from_rsa(rsa).unwrap();

    let mut name = X509NameBuilder::new().unwrap();
    name.append_entry_by_text("CN", common_name).unwrap();
    let name = name.build();

    let mut builder = X509::builder().unwrap();
    builder.set_version(2).unwrap();
    builder.set_subject_name(&name).unwrap();

    if let Some((issuer_cert, _)) = issuer {
        builder.set_issuer_name(issuer_cert.subject_name()).unwrap();
    } else {
        builder.set_issuer_name(&name).unwrap();
    }

    let not_before = Asn1Time::days_from_now(0).unwrap();
    let not_after = Asn1Time::days_from_now(365).unwrap();
    builder.set_not_before(&not_before).unwrap();
    builder.set_not_after(&not_after).unwrap();

    builder.set_pubkey(&privkey).unwrap();

    if is_ca {
        let bc = BasicConstraints::new().ca().build().unwrap();
        builder.append_extension(bc).unwrap();
        let ku = KeyUsage::new()
            .critical()
            .key_cert_sign()
            .crl_sign()
            .build()
            .unwrap();
        builder.append_extension(ku).unwrap();
    }

    let serial_number = {
        let mut serial = BigNum::new().unwrap();
        serial.rand(159, MsbOption::MAYBE_ZERO, false).unwrap();
        serial.to_asn1_integer().unwrap()
    };
    builder.set_serial_number(&serial_number).unwrap();

    let signing_key = if let Some((_, key)) = issuer {
        key
    } else {
        &privkey
    };

    builder.sign(signing_key, MessageDigest::sha256()).unwrap();
    let cert = builder.build();

    (cert, privkey)
}

#[test]
fn test_patch_validate_valid_chain() {
    let (root_ca, root_key) = create_test_cert("Root CA", None, true);
    let (int_ca, int_key) = create_test_cert("Intermediary CA", Some((&root_ca, &root_key)), true);
    let (leaf_cert, _leaf_key) = create_test_cert("Leaf", Some((&int_ca, &int_key)), false);

    let leaf_pem = String::from_utf8(leaf_cert.to_pem().unwrap()).unwrap();
    let int_pem = String::from_utf8(int_ca.to_pem().unwrap()).unwrap();
    let root_pem = String::from_utf8(root_ca.to_pem().unwrap()).unwrap();

    let chain_pem = format!("{}\n{}", int_pem, root_pem);

    let pubkey_pem =
        String::from_utf8(leaf_cert.public_key().unwrap().public_key_to_pem().unwrap()).unwrap();

    let request = PatchCertificateRequest {
        cert_pem: leaf_pem,
        chain_pem: Some(chain_pem),
    };

    let result = request.validate("fake-csr", &pubkey_pem);
    assert!(result.is_ok());
}

#[test]
fn test_patch_validate_invalid_pubkey() {
    let (root_ca, root_key) = create_test_cert("Root CA", None, true);
    let (leaf_cert, _) = create_test_cert("Leaf", Some((&root_ca, &root_key)), false);
    let (other_cert, _) = create_test_cert("Other", None, false);

    let leaf_pem = String::from_utf8(leaf_cert.to_pem().unwrap()).unwrap();
    let other_pubkey_pem = String::from_utf8(
        other_cert
            .public_key()
            .unwrap()
            .public_key_to_pem()
            .unwrap(),
    )
    .unwrap();

    let request = PatchCertificateRequest {
        cert_pem: leaf_pem,
        chain_pem: None,
    };

    let result = request.validate("fake-csr", &other_pubkey_pem);
    assert!(result.is_err());
    match result.unwrap_err() {
        ConversionError::Inconsistent(field, _) => assert_eq!(field, "cert_pem"),
        _ => panic!("Expected Inconsistent error"),
    }
}

#[test]
fn test_patch_validate_broken_chain() {
    let (root_ca, root_key) = create_test_cert("Root CA", None, true);
    let (other_root, other_key) = create_test_cert("Other Root", None, true);
    let (leaf_cert, _) = create_test_cert("Leaf", Some((&root_ca, &root_key)), false);

    let leaf_pem = String::from_utf8(leaf_cert.to_pem().unwrap()).unwrap();
    let other_root_pem = String::from_utf8(other_root.to_pem().unwrap()).unwrap();

    let pubkey_pem =
        String::from_utf8(leaf_cert.public_key().unwrap().public_key_to_pem().unwrap()).unwrap();

    let request = PatchCertificateRequest {
        cert_pem: leaf_pem,
        chain_pem: Some(other_root_pem),
    };

    let result = request.validate("fake-csr", &pubkey_pem);
    assert!(result.is_err());
    match result.unwrap_err() {
        ConversionError::DomainViolation(field, _) => assert_eq!(field, "chain_pem"),
        _ => panic!("Expected DomainViolation error"),
    }
}

#[test]
fn test_patch_validate_cross_signed_scenario() {
    // Cross-signing scenario: Leaf is signed by Int, Int is signed by Root1.
    // We also have Root2.
    // In the chain provided by user, they might include Int, Root1, Root2.
    // Our current validation only checks that each cert signs the PREVIOUS one.
    // So if they provide [Int, Root1, Root2], it will FAIL because Root2 doesn't sign Root1.
    // IF they provide [Int, Root1], it succeeds.

    let (root1, root1_key) = create_test_cert("Root 1", None, true);
    let (int_ca, int_key) = create_test_cert("Intermediary CA", Some((&root1, &root1_key)), true);
    let (leaf_cert, _) = create_test_cert("Leaf", Some((&int_ca, &int_key)), false);

    let leaf_pem = String::from_utf8(leaf_cert.to_pem().unwrap()).unwrap();
    let int_pem = String::from_utf8(int_ca.to_pem().unwrap()).unwrap();
    let root1_pem = String::from_utf8(root1.to_pem().unwrap()).unwrap();

    let pubkey_pem =
        String::from_utf8(leaf_cert.public_key().unwrap().public_key_to_pem().unwrap()).unwrap();

    // Valid chain [Int, Root1]
    let chain_pem = format!("{}\n{}", int_pem, root1_pem);
    let request = PatchCertificateRequest {
        cert_pem: leaf_pem.clone(),
        chain_pem: Some(chain_pem),
    };
    assert!(request.validate("fake-csr", &pubkey_pem).is_ok());

    // Now test where the chain has an unrelated cert at the end.
    let (root2, _) = create_test_cert("Root 2", None, true);
    let root2_pem = String::from_utf8(root2.to_pem().unwrap()).unwrap();
    let bad_chain_pem = format!("{}\n{}\n{}", int_pem, root1_pem, root2_pem);
    let request_bad = PatchCertificateRequest {
        cert_pem: leaf_pem,
        chain_pem: Some(bad_chain_pem),
    };
    let result = request_bad.validate("fake-csr", &pubkey_pem);
    assert!(result.is_err());
}

#[tokio::test]
async fn patch_certificate_test() {
    // --- Arrange ---
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // 1. Get a valid key algorithm ID
    let algos_response = client
        .get(&format!("{}/keys", &address))
        .send()
        .await
        .expect("Failed to get keys");
    assert!(algos_response.status().is_success());
    let algos: Vec<KeyAlgorithmResponse> = algos_response.json().await.unwrap();
    // Use the first RSA algorithm (or any, but we know RSA exists from seeding)
    let algo = algos
        .iter()
        .find(|a| a.algorithm_type.name == "RSA")
        .expect("RSA algorithm not found");

    // 2. Create a certificate
    let create_request = CreateCertificateRequest {
        key_algorithm_id: algo.id,
        subject: CertificateSubject {
            organization: Some("Test Org".into()),
            organizational_unit: Some("Test OU".into()),
            country: Some("US".into()),
            state_or_province: Some("California".into()),
            locality: Some("San Francisco".into()),
            email: Some("test@example.com".into()),
        },
        sans: vec!["example.com".into(), "127.0.0.1".into()],
        validity_days: 365,
    };

    let create_response = client
        .post(&format!("{}/certificates", &address))
        .json(&create_request)
        .send()
        .await
        .expect("Failed to create certificate");
    assert!(create_response.status().is_success());
    let cert_info: CertificateInfoResponse = create_response.json().await.unwrap();
    let cert_id = cert_info.id;
    let public_key_pem = cert_info.pem.public_key_pem;

    // 3. Generate a signed certificate for it
    // We'll create a Root CA and sign the CSR's public key
    let (root_ca, root_key) = create_test_cert("Root CA", None, true);

    // To sign the leaf properly, we need its public key from PEM
    let leaf_pubkey = PKey::public_key_from_pem(public_key_pem.as_bytes()).unwrap();

    let mut builder = X509::builder().unwrap();
    builder.set_version(2).unwrap();
    let mut name = X509NameBuilder::new().unwrap();
    name.append_entry_by_text("CN", "example.com").unwrap();
    let name = name.build();
    builder.set_subject_name(&name).unwrap();
    builder.set_issuer_name(root_ca.subject_name()).unwrap();
    let not_before = Asn1Time::days_from_now(0).unwrap();
    let not_after = Asn1Time::days_from_now(365).unwrap();
    builder.set_not_before(&not_before).unwrap();
    builder.set_not_after(&not_after).unwrap();
    builder.set_pubkey(&leaf_pubkey).unwrap();

    let serial_number = {
        let mut serial = BigNum::new().unwrap();
        serial.rand(159, MsbOption::MAYBE_ZERO, false).unwrap();
        serial.to_asn1_integer().unwrap()
    };
    builder.set_serial_number(&serial_number).unwrap();
    builder.sign(&root_key, MessageDigest::sha256()).unwrap();
    let leaf_cert = builder.build();

    let leaf_pem = String::from_utf8(leaf_cert.to_pem().unwrap()).unwrap();
    let root_pem = String::from_utf8(root_ca.to_pem().unwrap()).unwrap();

    // --- Act ---
    let patch_request = PatchCertificateRequest {
        cert_pem: leaf_pem,
        chain_pem: Some(root_pem),
    };

    let patch_response = client
        .patch(&format!("{}/certificates/{}", &address, cert_id))
        .json(&patch_request)
        .send()
        .await
        .expect("Failed to patch certificate");

    // --- Assert ---
    let status = patch_response.status();
    let response_text = patch_response.text().await.unwrap();
    println!("Patch Response: {}", response_text);

    assert!(status.is_success(), "Patch certificate failed: {}", response_text);

    let updated_info: CertificateInfoResponse = serde_json::from_str(&response_text).unwrap();
    assert!(updated_info.is_signed);
    assert!(updated_info.pem.cert_pem.is_some());
    assert!(updated_info.pem.chain_pem.is_some());
    assert!(updated_info.x509.fingerprint.is_some());
}
