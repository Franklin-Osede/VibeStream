use axum::{
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
};
use std::time::Instant;

#[derive(Clone)]
pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn log_request(request: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    tracing::info!("Request started: {} {}", method, uri);
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    tracing::info!(
        "Request completed: {} {} - status: {} - duration: {:?}",
        method,
        uri,
        response.status(),
        duration
    );

    response
} 