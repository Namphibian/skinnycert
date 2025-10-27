use actix_web::web;
use crate::server::routes::handlers::certificates::{get_handler, patch_handler, post_handler, put_handler, delete_handler};

const PATH: &'static str = "/certificates";

pub fn configure_certificate_route(cfg: &mut web::ServiceConfig) {
    // Register a GET route for health checking
    cfg.route(PATH, web::get().to(get_handler));
    cfg.route(PATH, web::post().to(post_handler));
    (cfg).route(PATH, web::put().to(put_handler));
    (cfg).route(PATH, web::patch().to(patch_handler));
    (cfg).route(PATH, web::delete().to(delete_handler));

}