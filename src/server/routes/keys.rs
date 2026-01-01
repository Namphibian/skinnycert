use actix_web::web;

pub mod dto;
pub mod handler;

macro_rules! key_path {
    () => {
        "/keys"
    };
    ($suffix:literal) => {
        concat!("/keys", $suffix)
    };
}
const PATH: &str = key_path!();
const PATH_WITH_ID: &str = key_path!("/{id}");
const PATH_WITH_ID_KEYPAIR: &str = key_path!("/{id}/keypair");

pub fn configure_key_algorithm_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(PATH, web::get().to(handler::get_handler));
    cfg.route(PATH_WITH_ID, web::get().to(handler::get_by_id_handler));
    cfg.route(
        PATH_WITH_ID_KEYPAIR,
        web::get().to(handler::generate_key_pair),
    );
    //cfg.route(PATH, web::post().to(method_not_allowed_handler));
    // cfg.route(PATH_WITH_ID, web::put().to(handler::put_handler));
    // cfg.route(PATH_WITH_ID, web::patch().to(handler::patch_handler));
    //cfg.route(PATH_WITH_ID, web::delete().to(handler::delete_handler));
}
