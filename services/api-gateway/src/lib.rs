// Módulos habilitados
pub mod auth;
// pub mod blockchain;
// pub mod bounded_contexts;
// pub mod handlers;
// pub mod shared;

// Simple module that works
pub mod simple {
    use axum::{
        routing::{get, post},
        Router,
        extract::{State, Json},
        response::Json as ResponseJson,
        http::StatusCode,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    
    #[derive(Clone)]
    pub struct AppState {
        pub version: String,
    }
    
    pub fn create_router() -> Router {
        let app_state = AppState {
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        
        Router::new()
            .route("/health", get(health_check))
            .route("/api/auth/login", post(login))
            .with_state(app_state)
    }
    
    async fn health_check(State(state): State<AppState>) -> ResponseJson<serde_json::Value> {
        ResponseJson(json!({
            "status": "ok",
            "version": state.version,
            "name": "VibeStream API Gateway",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
    
    #[derive(Deserialize)]
    struct LoginRequest {
        username: String,
        password: String,
    }
    
    #[derive(Serialize)]
    struct LoginResponse {
        token: String,
        user_id: String,
        expires_at: String,
    }
    
    async fn login(
        State(_state): State<AppState>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<ResponseJson<LoginResponse>, StatusCode> {
        // Demo implementation - in real app would validate credentials
        if payload.username == "demo" && payload.password == "password" {
            let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
            let response = LoginResponse {
                token: token.to_string(),
                user_id: "user_123".to_string(),
                expires_at: (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
            };
            Ok(ResponseJson(response))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
} 