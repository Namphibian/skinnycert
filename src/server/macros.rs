#[macro_export]
macro_rules! to_response_paged {
    ($result:expr, $dto_type:ty) => {
        match $result {
            Ok(paged) => {
                let items: Result<Vec<$dto_type>, _> = paged.items.into_iter().map(<$dto_type>::try_from).collect();
                match items {
                    Ok(items) => {
                        let response = $crate::server::models::shared::PagedResult {
                            items,
                            next_page_token: paged.next_page_token,
                            prev_page_token: paged.prev_page_token,
                            limit: paged.limit,
                        };
                        actix_web::HttpResponse::Ok().json(response)
                    }
                    Err(e) => {
                        tracing::error!(
                            error = %e,
                            context = "to_response_paged",
                            "DTO conversion failed for paged result"
                        );
                        actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Invalid format",
                            "message": e.to_string()
                        }))
                    }
                }
            }
            Err(e) => {
                let err: $crate::server::models::responses::RepositoryError = e.into();
                tracing::error!(
                    error = %err,
                    context = "to_response_paged",
                    "Repository error while fetching paged list"
                );
                actix_web::ResponseError::error_response(&err)
            }
        }
    };
}

#[macro_export]
macro_rules! to_response_list {
    ($result:expr, $dto_type:ty) => {
        match $result {
            Ok(models) => {
                let dtos: Result<Vec<$dto_type>, _> = models
                    .into_iter()
                    .map(<$dto_type>::try_from)
                    .collect();
                match dtos {
                    Ok(valid) => actix_web::HttpResponse::Ok().json(valid),
                    Err(e) => {
                        tracing::error!(
                            error = %e,
                            context = "to_response_list",
                            "DTO conversion failed for list"
                        );
                        actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Invalid format",
                            "message": e.to_string()
                        }))
                    }
                }
            }
            Err(e) => {
                let err: $crate::server::models::responses::RepositoryError = e.into();
                tracing::error!(
                    error = %err,
                    context = "to_response_list",
                    "Repository error while fetching list"
                );
                actix_web::HttpResponse::build(actix_web::ResponseError::status_code(&err)).json(serde_json::json!({
                    "error": err.to_string()
                }))
            }
        }
    };
}

#[macro_export]
macro_rules! to_response {
    ($result:expr, $dto_type:ty) => {
        match $result {
            Ok(Some(model)) => match <$dto_type>::try_from(model) {
                Ok(dto) => actix_web::HttpResponse::Ok().json(dto),
                Err(e) => {
                    tracing::error!(
                        error = %e,
                        context = "to_response/single",
                        "DTO conversion failed"
                    );
                    actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Invalid format",
                        "message": e.to_string()
                    }))
                }
            },
            Ok(None) => actix_web::HttpResponse::NotFound().json(serde_json::json!({
                "error": "Not found"
            })),
            Err(e) => {
                let err: $crate::server::models::responses::RepositoryError = e.into();
                tracing::error!(
                    error = %err,
                    context = "to_response/single",
                    "Repository error"
                );
                actix_web::HttpResponse::build(actix_web::ResponseError::status_code(&err)).json(serde_json::json!({
                    "error": err.to_string()
                }))
            }
        }
    };
}

#[macro_export]
macro_rules! to_patch_response {
    ($result:expr, $dto_type:ty) => {
        match $result {
            Ok($crate::server::models::responses::PatchResult::Updated(model)) => {
                match <$dto_type>::try_from(model) {
                    Ok(dto) => actix_web::HttpResponse::Ok().json(dto),
                    Err(e) => {
                        tracing::error!(
                            error = %e,
                            context = "to_patch_response",
                            "DTO conversion failed"
                        );
                        actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Invalid format",
                            "message": e.to_string()
                        }))
                    }
                }
            }
            Ok($crate::server::models::responses::PatchResult::NotFound) => actix_web::HttpResponse::NotFound().json(serde_json::json!({
                "error": "Resource not found. It may have been deleted after the patch request was processed."
            })),
            Ok($crate::server::models::responses::PatchResult::NotModified) => actix_web::HttpResponse::NotModified().finish(),
            Err(e) => {
                let err: $crate::server::models::responses::RepositoryError = e.into();
                tracing::error!(
                    error = %err,
                    context = "to_patch_response",
                    "Repository error while patching resource"
                );
                actix_web::HttpResponse::build(actix_web::ResponseError::status_code(&err)).json(serde_json::json!({
                    "error": err.to_string()
                }))
            }
        }
    };
}

#[macro_export]
macro_rules! key_pair_response {
    ($result:expr, $not_found_msg:expr) => {
        match $result {
            Ok(Some(model)) => match model.generate_key_pair() {
                Ok((private_key, public_key)) => actix_web::HttpResponse::Ok().json($crate::server::routes::responses::KeyPairResponse {
                    private_key,
                    public_key,
                }),
                Err(e) => actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to generate key pair",
                    "message": e.to_string()
                })),
            },
            Ok(None) => actix_web::HttpResponse::NotFound().json(serde_json::json!({
                "error": $not_found_msg
            })),
            Err(e) => {
                let err: $crate::server::models::responses::RepositoryError = e.into();
                actix_web::HttpResponse::build(actix_web::ResponseError::status_code(&err)).json(serde_json::json!({
                    "error": err.to_string()
                }))
            }
        }
    };
}

#[macro_export]
macro_rules! to_delete_response {
    ($result:expr) => {
        match $result {
            Ok(Some(_)) => actix_web::HttpResponse::NoContent().finish(),
            Ok(None) => actix_web::HttpResponse::NotFound().json(serde_json::json!({ "error": "Not found" })),
            Err(e) => {
                let err: $crate::server::models::responses::RepositoryError = e.into();
                actix_web::ResponseError::error_response(&err)
            }
        }
    };
}
