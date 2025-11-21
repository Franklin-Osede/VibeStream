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

// =============================================================================
// AUTHENTICATED USER EXTRACTOR
// =============================================================================
// 
// Extractor de Axum para obtener el usuario autenticado directamente en handlers
// Usa las claims insertadas por jwt_auth_middleware

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    async_trait,
};
use uuid::Uuid;
use crate::shared::domain::errors::AppError;

/// Usuario autenticado extraído del JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub tier: String,
}

impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        let user_id = Uuid::parse_str(&claims.sub)
            .unwrap_or_else(|_| Uuid::nil()); // Fallback si no se puede parsear
        
        Self {
            user_id,
            username: claims.username,
            email: claims.email,
            role: claims.role,
            tier: claims.tier,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, axum::response::Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extraer Claims de las extensions (insertadas por jwt_auth_middleware)
        let claims = parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or((
                axum::http::StatusCode::UNAUTHORIZED,
                axum::response::Json(serde_json::json!({
                    "success": false,
                    "message": "No se encontraron claims de autenticación. Asegúrate de usar jwt_auth_middleware.",
                    "error": "Missing authentication claims"
                })),
            ))?;
        
        // Convertir Claims a AuthenticatedUser
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| (
                axum::http::StatusCode::UNAUTHORIZED,
                axum::response::Json(serde_json::json!({
                    "success": false,
                    "message": "ID de usuario inválido en el token",
                    "error": "Invalid user ID in token"
                })),
            ))?;
        
        Ok(AuthenticatedUser {
            user_id,
            username: claims.username,
            email: claims.email,
            role: claims.role,
            tier: claims.tier,
        })
    }
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

