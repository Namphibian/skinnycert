use crate::server::logger::SkinnycertRouteSpanBuilder;
use crate::server::routes::certificates::configure_certificate_route;
use crate::server::routes::health_check::configure_health_check;
use crate::server::routes::key_statuses::configure_key_algorithm_status_routes;
use crate::server::routes::key_type_tls_statuses::configure_key_algorithm_type_tls_status_routes;
use crate::server::routes::key_types::configure_key_algorithm_type_routes;
use crate::server::routes::keys::configure_key_algorithm_routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    worker_threads: u16,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(TracingLogger::<SkinnycertRouteSpanBuilder>::new())
            .configure(configure_health_check)
            .configure(configure_key_algorithm_routes)
            .configure(configure_key_algorithm_type_routes)
            .configure(configure_key_algorithm_type_tls_status_routes)
            .configure(configure_key_algorithm_status_routes)
            .configure(configure_certificate_route)
    })
    .workers(worker_threads as usize)
    .listen(listener)?
    .run();
    Ok(server)
}
