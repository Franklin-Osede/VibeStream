use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            "authorization",
            "content-type",
            "x-requested-with",
            "accept",
            "origin",
            "access-control-request-method",
            "access-control-request-headers",
        ])
        .allow_credentials(true)
        .max_age(3600)
}

pub fn create_mobile_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("exp://localhost:8081".parse::<HeaderValue>().unwrap())
        .allow_origin("exp://192.168.1.100:8081".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            "authorization",
            "content-type",
            "x-requested-with",
            "accept",
            "origin",
            "expo-dev-client",
        ])
        .allow_credentials(true)
        .max_age(3600)
} 