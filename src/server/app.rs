use crate::server::logger::SkinnycertRouteSpanBuilder;
use crate::server::routes::health_check::configure_health_check;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use crate::server::routes::certificates::configure_certificate_route;
use sqlx::PgPool;

pub fn run(listener: TcpListener, worker_threads: u16, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(TracingLogger::<SkinnycertRouteSpanBuilder>::new())
            .configure(configure_health_check)
            .configure(configure_certificate_route)
    })
    .workers(worker_threads as usize)
    .listen(listener)?
    .run();
    Ok(server)
}
