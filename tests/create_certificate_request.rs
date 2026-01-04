use uuid::Uuid;

use skinnycert::server::routes::certificates::dto::{CertificateSubject, CreateCertificateRequest};
use skinnycert::server::routes::conversions::ConversionError;


fn base_subject() -> CertificateSubject {
    CertificateSubject {
        organization: Some("Org".into()),
        organizational_unit: Some("Unit".into()),
        country: Some("US".into()),
        state_or_province: Some("State".into()),
        locality: Some("City".into()),
        email: Some("test@example.com".into()),
    }
}


fn base_request() -> CreateCertificateRequest {
    CreateCertificateRequest {
        key_algorithm_id: Uuid::new_v4(),
        subject: base_subject(),
        sans: vec!["example.com".into()],
        validity_days: 365,
    }
}

#[test]
fn valid_request_passes() {
    let req = base_request();
    assert!(req.validate().is_ok());
}

#[test]
fn fails_when_sans_empty() {
    let mut req = base_request();
    req.sans = vec![];

    let err = req.validate().unwrap_err();
    match err {
        ConversionError::DomainViolation(field, _) => assert_eq!(field, "sans"),
        _ => panic!("Expected DomainViolation for sans"),
    }
}

#[test]
fn fails_when_san_is_empty_string() {
    let mut req = base_request();
    req.sans = vec!["".into()];

    let err = req.validate().unwrap_err();
    match err {
        ConversionError::InvalidValue(field, _) => assert_eq!(field, "sans"),
        _ => panic!("Expected InvalidValue for empty SAN"),
    }
}

#[test]
fn fails_when_san_is_invalid_format() {
    let mut req = base_request();
    req.sans = vec!["not a domain".into()];

    let err = req.validate().unwrap_err();
    match err {
        ConversionError::InvalidValue(field, _) => assert_eq!(field, "sans"),
        _ => panic!("Expected InvalidValue for invalid SAN"),
    }
}

#[test]
fn fails_when_country_not_two_chars() {
    let mut req = base_request();
    req.subject.country = Some("USA".into());

    let err = req.validate().unwrap_err();
    match err {
        ConversionError::InvalidValue(field, _) => assert_eq!(field, "country"),
        _ => panic!("Expected InvalidValue for country length"),
    }
}

#[test]
fn fails_when_validity_days_zero() {
    let mut req = base_request();
    req.validity_days = 0;

    let err = req.validate().unwrap_err();
    match err {
        ConversionError::OutOfRange(field, _) => assert_eq!(field, "validity_days"),
        _ => panic!("Expected OutOfRange for validity_days"),
    }
}

#[test]
fn fails_when_subject_field_too_long() {
    let mut req = base_request();
    req.subject.organization = Some("A".repeat(300)); // >256

    let err = req.validate().unwrap_err();
    match err {
        ConversionError::InvalidValue(field, _) => assert_eq!(field, "organization"),
        _ => panic!("Expected InvalidValue for organization length"),
    }
}

