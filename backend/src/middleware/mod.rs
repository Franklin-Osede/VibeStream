use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod logging;

pub use auth::AuthMiddleware;
pub use logging::LoggingMiddleware;

// Re-export middleware constructors
pub fn create_auth_middleware() -> AuthMiddleware {
    AuthMiddleware::new()
}

pub fn create_logging_middleware() -> TraceLayer {
    TraceLayer::new_for_http()
}

// Middleware trait for custom implementations
#[async_trait::async_trait]
pub trait Middleware<S>: Send + Sync + 'static {
    async fn handle<B>(
        &self,
        state: State<S>,
        req: Request<B>,
        next: Next<B>,
    ) -> Result<Response, Response>;
} 