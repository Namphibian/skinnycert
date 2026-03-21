pub mod dto;
pub mod handler;

use actix_web::web;
use handler::{get_health, post_health};

/// The base path for the health check endpoint.
///
/// This constant defines the URL path where the health check routes will be mounted.
/// Both GET and POST requests will respond on this path.
const PATH: &'static str = "/health";

/// Configures the health check routes for the Actix Web application.
///
/// This function is intended to be called inside your Actix Web application setup:
///
/// It registers two routes:
/// - `GET /health` → handled by [`get_health`], typically used for readiness checks.
/// - `POST /health` → handled by [`post_health`], often used for more detailed or authenticated liveness checks.
///
/// # Parameters
/// - `cfg`: The mutable [`ServiceConfig`] used to register routes with the Actix Web app.

pub fn configure_health_check(cfg: &mut web::ServiceConfig) {
    // Register a GET route for health checking
    cfg.route(PATH, web::get().to(get_health));
    // Register a POST route for health checking
    cfg.route(PATH, web::post().to(post_health));
}
