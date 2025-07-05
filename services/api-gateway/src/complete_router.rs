use axum::{
    routing::{get, post},
    Router,
    extract::{State, Json},
    response::Json as ResponseJson,
    http::StatusCode,
};
use serde_json::json;
use crate::services::{AppState, DatabasePool, MessageQueue};
use crate::handlers::{
    // Existing handlers
    get_balance, queue_status, database_health_check,
    get_users, get_songs, create_user, create_song,
    login, register, get_profile, get_wallet_balance,
    purchase_song, blockchain_health_check, get_user_transactions,
    oauth_login_register,
};

// Import new bounded context routes
use crate::bounded_contexts::{
    fractional_ownership::presentation::ownership_routes::create_ownership_routes,
    campaign::presentation::campaign_routes::create_campaign_routes,
    listen_reward::presentation::listen_routes::create_listen_routes,
};

/// Create the complete VibeStream API router with all bounded contexts
pub async fn create_complete_router() -> Result<Router, Box<dyn std::error::Error>> {
    // Initialize infrastructure services
    let database_pool = DatabasePool::new("postgres://vibestream:vibestream@localhost:5432/vibestream").await?;
    let message_queue = MessageQueue::new("redis://localhost:6379").await?;
    
    let app_state = AppState {
        database_pool,
        message_queue,
    };

    // Create main router with all APIs
    let router = Router::new()
        // Health and system endpoints
        .route("/health", get(comprehensive_health_check))
        .route("/health/database", get(database_health_check))
        .route("/health/blockchain", get(blockchain_health_check))
        .route("/health/queue", get(queue_status))
        
        // Authentication endpoints
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/oauth", post(oauth_login_register))
        .route("/api/v1/auth/profile", get(get_profile))
        
        // User management endpoints
        .route("/api/v1/users", get(get_users).post(create_user))
        .route("/api/v1/users/:id/transactions", get(get_user_transactions))
        
        // Music management endpoints  
        .route("/api/v1/music/songs", get(get_songs).post(create_song))
        .route("/api/v1/music/songs/:id/purchase", post(purchase_song))
        
        // Wallet and blockchain endpoints
        .route("/api/v1/wallet/balance/:blockchain/:address", get(get_wallet_balance))
        .route("/api/v1/blockchain/:blockchain/:address/balance", get(get_balance))
        
        // Nested routers for bounded contexts
        .nest("/api/v1/ownership", create_ownership_routes())
        .nest("/api/v1/campaigns", create_campaign_routes()) 
        .nest("/api/v1/listen", create_listen_routes())
        
        // Legacy music routes (keeping for compatibility)
        .nest("/api/music", crate::music_simple::create_music_routes())
        
        // Set application state
        .with_state(app_state);

    Ok(router)
}

/// Comprehensive health check that tests all system components
async fn comprehensive_health_check(State(state): State<AppState>) -> ResponseJson<serde_json::Value> {
    let mut health_status = serde_json::Map::new();
    
    // Test database connection
    let db_status = match state.database_pool.health_check().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };
    
    // Test Redis/message queue connection
    let redis_status = match state.message_queue.ping().await {
        Ok(_) => "healthy", 
        Err(_) => "unhealthy",
    };
    
    // Overall system status
    let overall_status = if db_status == "healthy" && redis_status == "healthy" {
        "healthy"
    } else {
        "degraded"
    };
    
    health_status.insert("status".to_string(), json!(overall_status));
    health_status.insert("timestamp".to_string(), json!(chrono::Utc::now().to_rfc3339()));
    health_status.insert("version".to_string(), json!("1.0.0"));
    health_status.insert("name".to_string(), json!("VibeStream API Gateway"));
    
    // Service health details
    let services = json!({
        "database": {
            "status": db_status,
            "type": "PostgreSQL"
        },
        "message_queue": {
            "status": redis_status,
            "type": "Redis"
        },
        "bounded_contexts": {
            "fractional_ownership": "implemented",
            "campaign": "implemented", 
            "listen_reward": "implemented",
            "payment": "implemented",
            "music": "implemented",
            "user": "implemented"
        }
    });
    
    health_status.insert("services".to_string(), services);
    
    // Available API endpoints summary
    let endpoints = json!({
        "authentication": [
            "POST /api/v1/auth/login",
            "POST /api/v1/auth/register", 
            "POST /api/v1/auth/oauth",
            "GET /api/v1/auth/profile"
        ],
        "fractional_ownership": [
            "POST /api/v1/ownership/contracts",
            "GET /api/v1/ownership/contracts",
            "GET /api/v1/ownership/contracts/:id",
            "POST /api/v1/ownership/contracts/:id/purchase",
            "POST /api/v1/ownership/contracts/:id/distribute",
            "GET /api/v1/ownership/users/:id/portfolio"
        ],
        "campaigns": [
            "POST /api/v1/campaigns",
            "GET /api/v1/campaigns",
            "GET /api/v1/campaigns/:id", 
            "POST /api/v1/campaigns/:id/activate",
            "POST /api/v1/campaigns/:id/purchase",
            "GET /api/v1/campaigns/:id/analytics"
        ],
        "listen_rewards": [
            "POST /api/v1/listen/sessions",
            "PUT /api/v1/listen/sessions/:id/complete",
            "GET /api/v1/listen/users/:id/rewards", 
            "POST /api/v1/listen/rewards/distribute",
            "GET /api/v1/listen/analytics"
        ],
        "music": [
            "GET /api/v1/music/songs",
            "POST /api/v1/music/songs",
            "POST /api/v1/music/songs/:id/purchase"
        ],
        "wallet": [
            "GET /api/v1/wallet/balance/:blockchain/:address",
            "GET /api/v1/blockchain/:blockchain/:address/balance"
        ]
    });
    
    health_status.insert("endpoints".to_string(), endpoints);
    
    ResponseJson(serde_json::Value::Object(health_status))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_complete_router_creation() {
        let result = create_complete_router().await;
        assert!(result.is_ok(), "Router creation should succeed");
    }

    #[tokio::test] 
    async fn test_health_endpoint_exists() {
        let router = create_complete_router().await.unwrap();
        
        let response = router
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .method("GET")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
} 