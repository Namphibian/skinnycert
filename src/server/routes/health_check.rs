mod handler;
mod model;

use actix_web::web;
use handler::{get_handler, post_handler};

/// The base path for the health check endpoint.
///
/// This constant defines the URL path where the health check routes will be mounted.
/// Both GET and POST requests will respond on this path.
const PATH: &'static str = "/health";

/// Configures the health check routes for the Actix Web application.
///
/// This function is intended to be called inside your Actix Web application setup:
///
/// ```rust
/// use actix_web::App;
/// use crate::server::routes::configure_health_check;
///
/// let app = App::new().configure(configure_health_check);
/// ```
///
/// It registers two routes:
/// - `GET /health` → handled by [`get_handler`], typically used for readiness checks.
/// - `POST /health` → handled by [`post_handler`], often used for more detailed or authenticated liveness checks.
///
/// # Parameters
/// - `cfg`: The mutable [`ServiceConfig`] used to register routes with the Actix Web app.
///
/// # Example
/// ```rust
/// use actix_web::{App, HttpServer};
/// use crate::server::routes::configure_health_check;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new()
///             .configure(configure_health_check)
///     })
///     .bind(("127.0.0.1", 8080))?
///     .run()
///     .await
/// }
/// ```
///

pub fn configure_health_check(cfg: &mut web::ServiceConfig) {
    // Register a GET route for health checking
    cfg.route(PATH, web::get().to(get_handler));
    // Register a POST route for health checking
    cfg.route(PATH, web::post().to(post_handler));
}
