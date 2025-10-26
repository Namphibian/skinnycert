use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::HeaderValue;
use actix_web::Error;
use std::io::Stdout;
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};
use tracing_bunyan_formatter::{BunyanFormattingLayer};


pub struct SkinnycertRouteSpanBuilder;

impl RootSpanBuilder for SkinnycertRouteSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let x_request_id_header = HeaderValue::from_static("not available");
        let x_request_id = request
            .headers()
            .get("X-Request-Id")
            .unwrap_or_else(|| &x_request_id_header)
            .to_str()
            .ok();
        // All fields you want to capture must be declared upfront.
        // If you don't know the value (yet), use tracing's `Empty`
        tracing_actix_web::root_span!(request, x_request_id)
    }

    fn on_request_end<B: MessageBody>(span: Span, response: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, response);
    }
}

pub fn configure_bunyan_logger_format() -> BunyanFormattingLayer<fn() -> Stdout> {
    return BunyanFormattingLayer::new(
        "skinnycert".into(),
        // Output the formatted spans to stdout.
        std::io::stdout,
    );
}