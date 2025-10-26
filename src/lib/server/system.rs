use crate::server::logger::SkinnycertRouteSpanBuilder;
use crate::server::routes::health_check::configure_health_check;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, worker_threads: u16) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::<SkinnycertRouteSpanBuilder>::new())
            .configure(configure_health_check)
    })
    .workers(worker_threads as usize)
    .listen(listener)?
    .run();
    Ok(server)
}
