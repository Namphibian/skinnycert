use actix_web::web;

mod handler;
mod dto;



macro_rules! ecdsa_path {
    () => { "/keys/ecdsa" };
    ($suffix:literal) => { concat!("/keys/ecdsa", $suffix) };
}

const PATH: &str = ecdsa_path!();
const PATH_WITH_ID: &str = ecdsa_path!("/{id}");
const PATH_WITH_ID_KEYPAIR: &str = ecdsa_path!("/{id}/keypair");

pub fn configure_ecdsa_key_route(cfg: &mut web::ServiceConfig) {
    cfg.route(PATH, web::get().to(handler::get_handler));
    cfg.route(PATH_WITH_ID, web::get().to(handler::get_by_id_handler));
    //
    // cfg.route(PATH_WITH_ID, web::put().to(handler::put_handler));
    // cfg.route(PATH_WITH_ID, web::patch().to(handler::patch_handler));
    // cfg.route(PATH_WITH_ID, web::delete().to(handler::delete_handler));
    // cfg.route(PATH_WITH_ID_KEYPAIR, web::get().to(handler::generate_key_pair));
}