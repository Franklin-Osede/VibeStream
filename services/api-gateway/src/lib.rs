// Módulos habilitados
pub mod auth;
pub mod blockchain;
pub mod bounded_contexts;
pub mod handlers;
pub mod services;
pub mod shared;
pub mod openapi;
pub mod complete_router;

// Solo el music context sin dependencias problemáticas
pub mod music_simple;

// Re-export the services AppState
pub use services::AppState;

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
    use crate::music_simple::create_music_routes;
    use crate::services::{AppState, DatabasePool, MessageQueue};
    
    pub async fn create_router() -> Result<Router, Box<dyn std::error::Error>> {
        // Initialize real services
        let database_pool = DatabasePool::new("postgres://vibestream:vibestream@localhost:5432/vibestream").await?;
        let message_queue = MessageQueue::new("redis://localhost:6379").await?;
        
        let app_state = AppState {
            database_pool,
            message_queue,
        };
        
        // Use the complete router with P2P analytics
        let router = crate::complete_router::create_app_router(app_state.database_pool.get_pool().clone()).await?;
            
        Ok(router)
    }
    
    async fn health_check(State(state): State<AppState>) -> ResponseJson<serde_json::Value> {
        // Test database connection
        let db_status = match state.database_pool.health_check().await {
            Ok(_) => "connected",
            Err(_) => "disconnected",
        };
        
        // Test Redis connection  
        let redis_status = match state.message_queue.ping().await {
            Ok(_) => "connected",
            Err(_) => "disconnected",
        };
        
        ResponseJson(json!({
            "status": "ok",
            "version": env!("CARGO_PKG_VERSION"),
            "name": "VibeStream API Gateway",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "services": {
                "database": db_status,
                "message_queue": redis_status,
                "music_context": "enabled"
            }
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