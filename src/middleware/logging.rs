use axum::BoxError;
use axum::{body::Body, http::Request, response::Response};
use std::error::Error;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

pub fn logging_middleware() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    tower_http::trace::DefaultMakeSpan,
    impl Fn(&Request<Body>) + Clone,
    impl Fn(&axum::http::Response<Body>) + Clone,
    impl Fn(&[u8]) + Clone,
    impl Fn(Option<&Box<dyn Error + Send + Sync>>) + Clone,
    impl Fn(&Box<dyn Error + Send + Sync>) + Clone,
> {
    TraceLayer::new_for_http()
        .on_request(|request: &Request<Body>| {
            info!("Received request: {} {}", request.method(), request.uri());
        })
        .on_response(|response: &Response<Body>| {
            info!("Sending response: {}", response.status());
        })
        .on_body_chunk(|chunk: &[u8]| {
            info!("Writing {} bytes", chunk.len());
        })
        .on_eos(|_: Option<&BoxError>| {
            info!("Finished sending response");
        })
        .on_failure(|error: &BoxError| {
            error!("Error processing request: {}", error);
        })
}
