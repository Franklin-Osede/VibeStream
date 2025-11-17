// =============================================================================
// OAUTH HANDLERS
// =============================================================================
// 
// This module provides HTTP handlers for OAuth authentication
// Integrates with real OAuth providers (Google, Apple)

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Json, Redirect},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::auth::{LoginResponse, UserInfo, Claims};
use crate::oauth::{RealOAuthService, OAuthConfig};

// =============================================================================
// OAUTH REQUEST/RESPONSE TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackRequest {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuthUrlResponse {
    pub auth_url: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthTokenRequest {
    pub provider: String,
    pub code: String,
    pub state: Option<String>,
}

// =============================================================================
// OAUTH HANDLERS
// =============================================================================

/// GET /auth/google - Redirect to Google OAuth
pub async fn google_auth() -> Redirect {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .unwrap_or_else(|_| "your-google-client-id".to_string());
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:3000/auth/google/callback".to_string());
    
    let state = uuid::Uuid::new_v4().to_string();
    let scope = "openid email profile";
    
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        client_id, redirect_uri, scope, state
    );
    
    Redirect::to(&auth_url)
}

/// GET /auth/apple - Redirect to Apple OAuth
pub async fn apple_auth() -> Redirect {
    let client_id = std::env::var("APPLE_CLIENT_ID")
        .unwrap_or_else(|_| "your-apple-client-id".to_string());
    let redirect_uri = std::env::var("APPLE_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:3000/auth/apple/callback".to_string());
    
    let state = uuid::Uuid::new_v4().to_string();
    let scope = "name email";
    
    let auth_url = format!(
        "https://appleid.apple.com/auth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        client_id, redirect_uri, scope, state
    );
    
    Redirect::to(&auth_url)
}

/// GET /auth/google/callback - Handle Google OAuth callback
pub async fn google_callback(
    Query(params): Query<OAuthCallbackRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    tracing::info!("🔐 Google OAuth callback received");
    
    // Initialize OAuth service
    let config = OAuthConfig::from_env()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let oauth_service = RealOAuthService::new(config);
    
    // Exchange code for user info
    match oauth_service.authenticate("google", &params.code).await {
        Ok(user_info) => {
            // Create or find user in database
            // For now, we'll create a mock response
            let user_id = uuid::Uuid::new_v4();
            let claims = Claims::new(
                user_id,
                user_info.name.clone(),
                user_info.email.clone(),
                "user".to_string(),
                "access".to_string(),
            );
            
            let token = claims.to_jwt()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let response = LoginResponse {
                token,
                refresh_token: "".to_string(), // TODO: Implement refresh tokens
                user: UserInfo {
                    id: user_id.to_string(),
                    username: user_info.name,
                    email: user_info.email,
                    role: "user".to_string(),
                    is_verified: true,
                },
                expires_in: 3600,
            };
            
            tracing::info!("✅ Google OAuth authentication successful");
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("❌ Google OAuth authentication failed: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// GET /auth/apple/callback - Handle Apple OAuth callback
pub async fn apple_callback(
    Query(params): Query<OAuthCallbackRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    tracing::info!("🍎 Apple OAuth callback received");
    
    // Initialize OAuth service
    let config = OAuthConfig::from_env()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let oauth_service = RealOAuthService::new(config);
    
    // Exchange code for user info
    match oauth_service.authenticate("apple", &params.code).await {
        Ok(user_info) => {
            // Create or find user in database
            // For now, we'll create a mock response
            let user_id = uuid::Uuid::new_v4();
            let claims = Claims::new(
                user_id,
                user_info.name.clone(),
                user_info.email.clone(),
                "user".to_string(),
                "access".to_string(),
            );
            
            let token = claims.to_jwt()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let response = LoginResponse {
                token,
                refresh_token: "".to_string(), // TODO: Implement refresh tokens
                user: UserInfo {
                    id: user_id.to_string(),
                    username: user_info.name,
                    email: user_info.email,
                    role: "user".to_string(),
                    is_verified: true,
                },
                expires_in: 3600,
            };
            
            tracing::info!("✅ Apple OAuth authentication successful");
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("❌ Apple OAuth authentication failed: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// POST /auth/oauth/token - Exchange OAuth code for JWT token
pub async fn oauth_token_exchange(
    Json(request): Json<OAuthTokenRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    tracing::info!("🔐 OAuth token exchange for provider: {}", request.provider);
    
    // Initialize OAuth service
    let config = OAuthConfig::from_env()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let oauth_service = RealOAuthService::new(config);
    
    // Authenticate with provider
    match oauth_service.authenticate(&request.provider, &request.code).await {
        Ok(user_info) => {
            // Create or find user in database
            // For now, we'll create a mock response
            let user_id = uuid::Uuid::new_v4();
            let claims = Claims::new(
                user_id,
                user_info.name.clone(),
                user_info.email.clone(),
                "user".to_string(),
                "access".to_string(),
            );
            
            let token = claims.to_jwt()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let response = LoginResponse {
                token,
                refresh_token: "".to_string(), // TODO: Implement refresh tokens
                user: UserInfo {
                    id: user_id.to_string(),
                    username: user_info.name,
                    email: user_info.email,
                    role: "user".to_string(),
                    is_verified: true,
                },
                expires_in: 3600,
            };
            
            tracing::info!("✅ OAuth authentication successful for provider: {}", request.provider);
            Ok(Json(response))
        }
        Err(e) => {
            tracing::error!("❌ OAuth authentication failed for provider {}: {:?}", request.provider, e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// GET /auth/providers - Get available OAuth providers
pub async fn get_oauth_providers() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "providers": [
            {
                "name": "google",
                "display_name": "Google",
                "auth_url": "/auth/google",
                "icon": "https://developers.google.com/identity/images/g-logo.png"
            },
            {
                "name": "apple",
                "display_name": "Apple",
                "auth_url": "/auth/apple",
                "icon": "https://developer.apple.com/assets/elements/icons/sign-in-with-apple/sign-in-with-apple.png"
            }
        ]
    }))
}

// =============================================================================
// OAUTH ROUTES
// =============================================================================

/// Create OAuth routes
pub fn create_oauth_routes() -> Router {
    Router::new()
        // OAuth initiation
        .route("/auth/google", get(google_auth))
        .route("/auth/apple", get(apple_auth))
        
        // OAuth callbacks
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/apple/callback", get(apple_callback))
        
        // OAuth token exchange
        .route("/auth/oauth/token", post(oauth_token_exchange))
        
        // OAuth provider info
        .route("/auth/providers", get(get_oauth_providers))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn test_oauth_providers_endpoint() {
        let response = get_oauth_providers().await;
        let body: serde_json::Value = response.0;
        
        assert!(body["providers"].is_array());
        assert_eq!(body["providers"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_oauth_callback_request_parsing() {
        let query_params = "code=test_code&state=test_state";
        let parsed: OAuthCallbackRequest = serde_urlencoded::from_str(query_params).unwrap();
        
        assert_eq!(parsed.code, "test_code");
        assert_eq!(parsed.state, Some("test_state".to_string()));
    }
}
