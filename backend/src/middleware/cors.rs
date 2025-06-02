use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
} 