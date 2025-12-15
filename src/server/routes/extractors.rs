//! The `PathUuid` struct is a wrapper around the `Uuid` type, which is used to extract and validate
//! a UUID from the path parameters of an HTTP request in an Actix Web application.
//!
//! # Usage
//!
//! This struct implements the `FromRequest` trait, allowing Actix Web to automatically
//! extract and validate a UUID from the request path. To use it, define a route with a path
//! parameter named `id` that contains a UUID.
//!
//! For example:
//! ```rust
//! use actix_web::{web, App, HttpServer, Responder};
//! use your_module::PathUuid; // Assuming that PathUuid is defined in `your_module`.
//!
//! async fn handler(path_uuid: PathUuid) -> impl Responder {
//!     format!("Extracted UUID: {}", path_uuid.0)
//! }
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| {
//!         App::new()
//!             .route("/items/{id}", web::get().to(handler)) // `id` is matched as a UUID.
//!     })
//!     .bind("127.0.0.1:8080")?
//!     .run()
//!     .await
//! }
//! ```
//!
//! # Implementation
//!
//! - The `type Error` is set to `actix_web::Error` to handle errors during the extraction.
//! - The `from_request` function extracts the `id` parameter from the request path using
//!   `req.match_info().query("id")`.
//! - The UUID string is parsed using `Uuid::parse_str`.
//! - If the parsing succeeds, the valid `Uuid` is wrapped in a `PathUuid` instance; otherwise,
//!   an error response is returned with a `400 Bad Request` status, indicating an invalid UUID format.
//!
//! # Errors
//!
//! - If the `id` parameter is missing or not a valid UUID string, a `BadRequest` error is returned.
//!
//! # Example Request
//!
//! Assuming your Actix Web app is running locally, the following request would extract the UUID:
//!
//! ```http
//! GET /items/550e8400-e29b-41d4-a716-446655440000 HTTP/1.1
//! Host: localhost:8080
//! ```
//!
//! In this case, the `PathUuid` would contain the parsed `Uuid` value `550e8400-e29b-41d4-a716-446655440000`.
//!
//! If an invalid UUID is provided (e.g., `/items/invalid-uuid`), the server will return a `400 Bad Request` response.
//!
//! # Fields
//!
//! - `pub 0: Uuid`
//!   The wrapped `Uuid` value extracted from the path parameter.
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PathUuid(pub Uuid);

impl FromRequest for PathUuid {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let id = req.match_info().query("id");
        match Uuid::parse_str(id) {
            Ok(uuid) => ready(Ok(PathUuid(uuid))),
            Err(_) => ready(Err(actix_web::error::ErrorBadRequest("Invalid UUID format"))),
        }
    }
}