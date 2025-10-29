use openssl::pkey::PKey;
use openssl::x509::X509Req;
use skinnycert::server::routes::handlers::models::certificate::{
    CertificateGenerationRequest, CertificateSubject, EcdsaCurve, KeyAlgorithm, KeyStrength,
    RsaKeySize, extract_public_key_from_private_key, validate_key_pair,
};

fn create_test_subject() -> CertificateSubject {
    CertificateSubject {
        organization: Some("Test Org".to_string()),
        organizational_unit: Some("Engineering".to_string()),
        country: Some("US".to_string()),
        state_or_province: Some("California".to_string()),
        locality: Some("San Francisco".to_string()),
        email: Some("test@example.com".to_string()),
    }
}

#[test]
fn test_rsa_2048_key_generation() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string(), "www.example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "RSA 2048 key generation failed");

    let (private_key, csr, public_key) = result.unwrap();

    // Verify PEM format
    assert!(private_key.contains("-----BEGIN PRIVATE KEY-----"));
    assert!(csr.contains("-----BEGIN CERTIFICATE REQUEST-----"));
    assert!(public_key.contains("-----BEGIN PUBLIC KEY-----"));

    // Verify key can be parsed
    let pkey = PKey::private_key_from_pem(private_key.as_bytes());
    assert!(pkey.is_ok(), "Private key is not valid");

    // Verify CSR can be parsed
    let csr_parsed = X509Req::from_pem(csr.as_bytes());
    assert!(csr_parsed.is_ok(), "CSR is not valid");
}

#[test]
fn test_rsa_4096_key_generation() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits4096),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "RSA 4096 key generation failed");

    let (private_key, _, _) = result.unwrap();
    let pkey = PKey::private_key_from_pem(private_key.as_bytes()).unwrap();
    let rsa = pkey.rsa().unwrap();

    // Verify key size
    assert_eq!(rsa.size() * 8, 4096, "RSA key size should be 4096 bits");
}

#[test]
fn test_ecdsa_p256_key_generation() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::ECDSA,
        key_strength: KeyStrength::Ecdsa(EcdsaCurve::P256),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "ECDSA P256 key generation failed");

    let (private_key, csr, public_key) = result.unwrap();

    // Verify PEM format
    assert!(private_key.contains("-----BEGIN PRIVATE KEY-----"));
    assert!(csr.contains("-----BEGIN CERTIFICATE REQUEST-----"));
    assert!(public_key.contains("-----BEGIN PUBLIC KEY-----"));

    // Verify it's an EC key
    let pkey = PKey::private_key_from_pem(private_key.as_bytes()).unwrap();
    assert!(pkey.ec_key().is_ok(), "Should be an EC key");
}

#[test]
fn test_ecdsa_p384_key_generation() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::ECDSA,
        key_strength: KeyStrength::Ecdsa(EcdsaCurve::P384),
        subject: create_test_subject(),
        sans: vec!["test.local".to_string()],
        validity_days: 730,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "ECDSA P384 key generation failed");
}

#[test]
fn test_ecdsa_p521_key_generation() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::ECDSA,
        key_strength: KeyStrength::Ecdsa(EcdsaCurve::P521),
        subject: create_test_subject(),
        sans: vec!["secure.example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "ECDSA P521 key generation failed");
}

#[test]
fn test_public_key_matches_private_key() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let (private_key, _, public_key) = request.generate_key_and_csr().unwrap();

    // Verify the public key matches the private key
    let matches = validate_key_pair(&private_key, &public_key).unwrap();
    assert!(matches, "Public key should match private key");
}

#[test]
fn test_csr_contains_subject_fields() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: CertificateSubject {
            organization: Some("Acme Corp".to_string()),
            organizational_unit: Some("IT".to_string()),
            country: Some("US".to_string()),
            state_or_province: Some("Texas".to_string()),
            locality: Some("Austin".to_string()),
            email: Some("admin@acme.com".to_string()),
        },
        sans: vec!["acme.com".to_string()],
        validity_days: 365,
    };

    let (_, csr, _) = request.generate_key_and_csr().unwrap();
    let csr_parsed = X509Req::from_pem(csr.as_bytes()).unwrap();
    let subject = csr_parsed.subject_name();

    // Verify subject fields
    let cn = subject.entries_by_nid(openssl::nid::Nid::COMMONNAME).next();
    assert!(cn.is_some(), "CN should be present");
    
    let org = subject.entries_by_nid(openssl::nid::Nid::ORGANIZATIONNAME).next();
    assert!(org.is_some(), "Organization should be present");
    assert_eq!(org.unwrap().data().as_utf8().unwrap().as_ref() as &str, "Acme Corp");
}

#[test]
fn test_csr_with_ip_address_san() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec![
            "192.168.1.1".to_string(),
            "10.0.0.1".to_string(),
            "example.com".to_string(),
        ],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "Should handle IP addresses in SANs");

    let (_, csr, _) = result.unwrap();
    let csr_parsed = X509Req::from_pem(csr.as_bytes()).unwrap();
    assert!(
        csr_parsed
            .verify(&csr_parsed.public_key().unwrap())
            .unwrap()
    );
}

#[test]
fn test_csr_with_multiple_dns_sans() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec![
            "example.com".to_string(),
            "www.example.com".to_string(),
            "api.example.com".to_string(),
            "*.example.com".to_string(),
        ],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "Should handle multiple DNS SANs");
}

#[test]
fn test_csr_signature_verification() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let (_, csr, _) = request.generate_key_and_csr().unwrap();
    let csr_parsed = X509Req::from_pem(csr.as_bytes()).unwrap();
    let public_key = csr_parsed.public_key().unwrap();

    // Verify CSR is properly signed
    let is_valid = csr_parsed.verify(&public_key).unwrap();
    assert!(is_valid, "CSR signature should be valid");
}

#[test]
fn test_validation_rejects_mismatched_algorithm_and_strength() {
    // RSA algorithm with ECDSA strength
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Ecdsa(EcdsaCurve::P256),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(
        result.is_err(),
        "Should reject RSA algorithm with ECDSA strength"
    );
}

#[test]
fn test_validation_rejects_ecdsa_with_rsa_strength() {
    // ECDSA algorithm with RSA strength
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::ECDSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(
        result.is_err(),
        "Should reject ECDSA algorithm with RSA strength"
    );
}

#[test]
fn test_minimal_subject_with_cn_only() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: CertificateSubject {
            organization: None,
            organizational_unit: None,
            country: None,
            state_or_province: None,
            locality: None,
            email: None,
        },
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let result = request.generate_key_and_csr();
    assert!(result.is_ok(), "Should work with minimal subject (CN only)");

    let (_, csr, _) = result.unwrap();
    let csr_parsed = X509Req::from_pem(csr.as_bytes()).unwrap();
    let subject = csr_parsed.subject_name();

    let cn = subject.entries_by_nid(openssl::nid::Nid::COMMONNAME).next();
    assert!(cn.is_some(), "CN should be set from first SAN");
    assert_eq!(cn.unwrap().data().as_utf8().unwrap().as_ref() as &str, "example.com");
}

#[test]
fn test_extract_public_key_from_private_key_rsa() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::RSA,
        key_strength: KeyStrength::Rsa(RsaKeySize::Bits2048),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let (private_key, _, _) = request.generate_key_and_csr().unwrap();
    let extracted_public = extract_public_key_from_private_key(&private_key);

    assert!(
        extracted_public.is_ok(),
        "Should extract public key from RSA private key"
    );
    assert!(
        extracted_public
            .unwrap()
            .contains("-----BEGIN PUBLIC KEY-----")
    );
}

#[test]
fn test_extract_public_key_from_private_key_ecdsa() {
    let request = CertificateGenerationRequest {
        key_algorithm: KeyAlgorithm::ECDSA,
        key_strength: KeyStrength::Ecdsa(EcdsaCurve::P256),
        subject: create_test_subject(),
        sans: vec!["example.com".to_string()],
        validity_days: 365,
    };

    let (private_key, _, _) = request.generate_key_and_csr().unwrap();
    let extracted_public = extract_public_key_from_private_key(&private_key);

    assert!(
        extracted_public.is_ok(),
        "Should extract public key from ECDSA private key"
    );
    assert!(
        extracted_public
            .unwrap()
            .contains("-----BEGIN PUBLIC KEY-----")
    );
}