use actix_web::web;
pub mod dto;
pub mod handler;

const PATH: &str = "/certificates";
const PATH_WITH_ID: &str = "/certificates/{id}";

pub fn configure_certificate_route(cfg: &mut web::ServiceConfig) {
    cfg.route(PATH, web::get().to(handler::get_handler));
    cfg.route(PATH, web::post().to(handler::post_handler));
    cfg.route(PATH_WITH_ID, web::get().to(handler::get_by_id_handler));
    cfg.route(PATH_WITH_ID, web::put().to(handler::put_handler));
    /*    cfg.route(PATH_WITH_ID, web::patch().to(handler::patch_handler));*/
    cfg.route(PATH_WITH_ID, web::delete().to(handler::delete_handler));
}
