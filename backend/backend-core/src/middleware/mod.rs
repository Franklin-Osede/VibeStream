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
use std::sync::Arc;
use crate::repositories::UserRepository;

mod auth;
mod cors;
mod logging;

pub use auth::auth_middleware;
pub use cors::create_cors_layer;
pub use logging::LoggingMiddleware;

pub struct AuthConfig {
    pub jwt_config: auth::JwtConfig,
    pub user_repository: Arc<dyn UserRepository>,
}

// Re-export middleware constructors
pub fn create_auth_middleware(config: AuthConfig) -> auth::AuthState {
    auth::auth_middleware(config.jwt_config, config.user_repository)
}

pub fn create_trace_layer() -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    let make_span = DefaultMakeSpan::new().level(Level::INFO);
    let on_response = DefaultOnResponse::new().level(Level::INFO);
    
    TraceLayer::new_for_http()
        .make_span_with(make_span)
        .on_response(on_response)
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