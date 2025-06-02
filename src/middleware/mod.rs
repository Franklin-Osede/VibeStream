use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
    body::Body,
};
use tower_http::trace::{
    TraceLayer,
    DefaultMakeSpan,
    DefaultOnResponse,
};
use tracing::Level;

pub mod auth;
pub mod logging;

// Re-export middleware constructors
pub fn create_auth_middleware() -> impl axum::middleware::FromRequest<(), Body> + Clone {
    auth::require_auth
}

pub fn create_logging_middleware() -> TraceLayer<DefaultMakeSpan, DefaultOnResponse> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

pub async fn log_request(request: Request<Body>, next: Next) -> Response {
    tracing::info!("Request: {} {}", request.method(), request.uri());
    next.run(request).await
}

// Middleware trait for custom implementations
#[async_trait::async_trait]
pub trait Middleware<S>: Send + Sync + 'static {
    async fn handle(
        &self,
        state: State<S>,
        request: Request<Body>,
        next: Next,
    ) -> Result<Response, Response>;
} 