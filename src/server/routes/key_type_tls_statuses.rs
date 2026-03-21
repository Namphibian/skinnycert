use actix_web::web;

pub mod dto;
pub mod handler;

macro_rules! key_path {
    () => {
        "/key_type_tls_statuses"
    };
    ($suffix:literal) => {
        concat!("/key_type_tls_statuses", $suffix)
    };
}
const PATH: &str = key_path!();
// const PATH_WITH_ID: &str = key_path!("/{id}");

pub fn configure_key_algorithm_type_tls_status_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(PATH, web::get().to(handler::get_key_algorithm_type_tls_statuses));
    // cfg.route(PATH_WITH_ID, web::get().to(crate::server::routes::keys::handler::get_certificate_by_id));
    // cfg.route(PATH, web::post().to(handler::create_certificate));
    // cfg.route(PATH_WITH_ID, web::put().to(handler::put_handler));
    // cfg.route(PATH_WITH_ID, web::patch().to(handler::patch_certificate));
    // cfg.route(PATH_WITH_ID, web::delete().to(handler::delete_certificate));
}
