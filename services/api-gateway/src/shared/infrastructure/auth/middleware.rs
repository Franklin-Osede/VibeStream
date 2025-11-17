// =============================================================================
// JWT AUTHENTICATION MIDDLEWARE
// =============================================================================

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::shared::infrastructure::auth::{JwtService, Claims};
use std::sync::Arc;

/// Extract JWT token from Authorization header
fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_string())
            } else {
                None
            }
        })
}

/// JWT Authentication Middleware
/// Validates JWT token from Authorization header and extracts claims
/// Compatible with axum::middleware::from_fn
pub async fn jwt_auth_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // Get JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_in_production".to_string());
    
    // Validate JWT token
    let jwt_service = match JwtService::new(&jwt_secret) {
        Ok(service) => service,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("JWT service initialization failed"))
                .unwrap()
                .into();
        }
    };
    
    // Extract token from headers
    let token = match extract_token(request.headers()) {
        Some(token) => token,
        None => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("Missing or invalid authorization header"))
                .unwrap()
                .into();
        }
    };
    
    // Validate token
    let claims = match jwt_service.validate_access_token(&token) {
        Ok(claims) => claims,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::from("Invalid or expired token"))
                .unwrap()
                .into();
        }
    };
    
    // Add claims to request extensions for use in handlers
    request.extensions_mut().insert(claims);
    
    next.run(request).await
}

/// Extract Claims from request extensions (for use in handlers)
pub fn extract_claims(request: &Request) -> Option<Claims> {
    request.extensions().get::<Claims>().cloned()
}

/// Optional JWT Authentication Middleware
/// Allows requests to proceed even without a token, but validates if present
pub async fn optional_jwt_auth_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret_change_in_production".to_string());
    
    if let Ok(jwt_service) = JwtService::new(&jwt_secret) {
        if let Some(token) = extract_token(request.headers()) {
            if let Ok(claims) = jwt_service.validate_access_token(&token) {
                request.extensions_mut().insert(claims);
            }
        }
    }
    
    next.run(request).await
}

