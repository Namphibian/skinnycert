pub mod app;
pub mod routes;
pub mod config;
mod logger;
pub mod models;

use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;
use logger::SkinnycertRouteSpanBuilder;
use routes::certificates::configure_certificate_route;
use routes::health_check::configure_health_check;
use routes::rsa_keys::configure_rsa_key_route;

pub fn run(listener: TcpListener, worker_threads: u16) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::<SkinnycertRouteSpanBuilder>::new())
            .configure(configure_health_check)
            .configure(configure_certificate_route)

    })
        .workers(worker_threads as usize)
        .listen(listener)?
        .run();
    Ok(server)
}