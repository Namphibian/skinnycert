use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::{Deserialize, Serialize};
use skinnycert::server::models::key_algorithms::db::KeyPair;
use skinnycert::server::models::responses::{PatchResult, RepositoryError};
use skinnycert::server::routes::responses::{
    key_pair_response, to_patch_response, to_response, to_response_list, KeyPairResponse,
};

#[derive(Serialize)]
#[derive(Debug)]
struct MockModel {
    id: i32,
}
#[derive(Serialize, Deserialize)]
struct MockDto {
    id: i32,
}
impl TryFrom<MockModel> for MockDto {
    type Error = String;
    fn try_from(m: MockModel) -> Result<Self, Self::Error> {
        Ok(MockDto { id: m.id })
    }
}

#[actix_web::test]
async fn test_repository_error_status_codes() {
    assert_eq!(
        RepositoryError::UniqueViolation {
            constraint: "c".into()
        }
        .status_code(),
        StatusCode::CONFLICT
    );
    assert_eq!(
        RepositoryError::ForeignKeyViolation {
            constraint: "c".into()
        }
        .status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        RepositoryError::NotNullViolation { column: "c".into() }.status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        RepositoryError::CheckViolation {
            constraint: "c".into()
        }
        .status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        RepositoryError::StringTooLong { column: "c".into() }.status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        RepositoryError::NumericOutOfRange.status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        RepositoryError::InvalidDatetime.status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        RepositoryError::SyntaxError.status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        RepositoryError::UndefinedColumn { column: "c".into() }.status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        RepositoryError::UndefinedTable { table: "t".into() }.status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        RepositoryError::SerializationFailure.status_code(),
        StatusCode::CONFLICT
    );
    assert_eq!(
        RepositoryError::QueryCanceled.status_code(),
        StatusCode::REQUEST_TIMEOUT
    );
    assert_eq!(
        RepositoryError::DeadlockDetected.status_code(),
        StatusCode::CONFLICT
    );
    assert_eq!(
        RepositoryError::InsufficientPrivilege.status_code(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        RepositoryError::Database {
            message: "m".into()
        }
        .status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        RepositoryError::Transaction {
            message: "m".into()
        }
        .status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[actix_web::test]
async fn test_repository_error_response() {
    let err = RepositoryError::UniqueViolation {
        constraint: "unique_name".into(),
    };
    let resp = err.error_response();
    assert_eq!(resp.status(), StatusCode::CONFLICT);

    let body_bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body["details"], "unique_name");
}
//
#[actix_web::test]
async fn test_to_response_list_ok() {
    let models = vec![MockModel { id: 1 }, MockModel { id: 2 }];
    let result: Result<Vec<MockModel>, RepositoryError> = Ok(models);
    let resp = to_response_list::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::OK);

    let body_bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
    let body: Vec<MockDto> = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body.len(), 2);
    assert_eq!(body[0].id, 1);
    assert_eq!(body[1].id, 2);
}

#[actix_web::test]
async fn test_to_response_list_err() {
    let result: Result<Vec<MockModel>, RepositoryError> =
        Err(RepositoryError::SerializationFailure);
    let resp = to_response_list::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_web::test]
async fn test_to_response_ok() {
    let result: Result<Option<MockModel>, RepositoryError> = Ok(Some(MockModel { id: 1 }));
    let resp = to_response::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
    let body: MockDto = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body.id, 1);
}

#[actix_web::test]
async fn test_to_response_none() {
    let result: Result<Option<MockModel>, RepositoryError> = Ok(None);
    let resp = to_response::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_to_patch_response_updated() {
    let result: Result<PatchResult<MockModel>, RepositoryError> =
        Ok(PatchResult::Updated(MockModel { id: 1 }));
    let resp = to_patch_response::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_to_patch_response_not_modified() {
    let result: Result<PatchResult<MockModel>, RepositoryError> = Ok(PatchResult::NotModified);
    let resp = to_patch_response::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::NOT_MODIFIED);
}

#[actix_web::test]
async fn test_to_patch_response_not_found() {
    let result: Result<PatchResult<MockModel>, RepositoryError> = Ok(PatchResult::NotFound);
    let resp = to_patch_response::<MockModel, MockDto, RepositoryError>(result);
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[derive(Debug)]
struct MockKeyAlg;
impl KeyPair for MockKeyAlg {
    fn generate_key_pair(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        Ok(("priv".into(), "pub".into()))
    }
    fn verify_key_pair(
        &self,
        _priv: String,
        _pub: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[actix_web::test]
async fn test_key_pair_response_ok() {
    let result: Result<Option<MockKeyAlg>, RepositoryError> = Ok(Some(MockKeyAlg));
    let resp = key_pair_response(result, "not found");
    assert_eq!(resp.status(), StatusCode::OK);
    let body_bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
    let body: KeyPairResponse = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body.private_key, "priv");
    assert_eq!(body.public_key, "pub");
}

#[actix_web::test]
async fn test_key_pair_response_none() {
    let result: Result<Option<MockKeyAlg>, RepositoryError> = Ok(None);
    let resp = key_pair_response(result, "custom not found msg");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let body_bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body["error"], "custom not found msg");
}
