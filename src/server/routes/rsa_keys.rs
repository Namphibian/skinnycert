mod handler;
pub mod dto;

use actix_web::web;

const PATH: &str = "/keys/rsa";
const PATH_WITH_ID: &str = concat!("/keys/rsa/{id}");
const PATH_WITH_ID_KEYPAIR: &str = concat!("/keys/rsa/{id}/keypair");

pub fn configure_rsa_key_route(cfg: &mut web::ServiceConfig) {
    cfg.route(PATH, web::get().to(handler::get_handler));
    cfg.route(PATH_WITH_ID, web::get().to(handler::get_by_id_handler));
    cfg.route(PATH, web::post().to(handler::post_handler));
    cfg.route(PATH_WITH_ID, web::put().to(handler::put_handler));
    cfg.route(PATH_WITH_ID, web::patch().to(handler::patch_handler));
    cfg.route(PATH_WITH_ID, web::delete().to(handler::delete_handler));
    cfg.route(PATH_WITH_ID_KEYPAIR, web::get().to(handler::generate_key_pair));
}