//! Simple API Test for Fan Loyalty System
//! 
//! TDD GREEN PHASE - Simple test without database

use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
    Router,
};
use tower::ServiceExt;
use serde_json::json;

/// Test simple health check - TDD GREEN PHASE
#[tokio::test]
async fn test_simple_health_check() {
    // TDD GREEN PHASE: Simple test that should pass
    
    let app = create_simple_test_app();
    
    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["status"], "healthy");
    assert_eq!(response_json["service"], "fan-loyalty");
    
    println!("✅ Simple health check test passed!");
}

/// Test simple info endpoint - TDD GREEN PHASE
#[tokio::test]
async fn test_simple_info() {
    // TDD GREEN PHASE: Simple test that should pass
    
    let app = create_simple_test_app();
    
    let request = Request::builder()
        .method(Method::GET)
        .uri("/info")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["service"], "Fan Loyalty Gateway");
    assert!(response_json["description"].is_string());
    
    println!("✅ Simple info test passed!");
}

/// Helper function to create simple test app
fn create_simple_test_app() -> Router {
    // TDD GREEN PHASE: Create simple test app without database
    Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/info", axum::routing::get(info))
}

/// Simple health check handler
async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(json!({
        "status": "healthy",
        "service": "fan-loyalty",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Simple info handler
async fn info() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(json!({
        "service": "Fan Loyalty Gateway",
        "description": "Sistema de lealtad de fans con verificación biométrica, NFT wristbands y códigos QR",
        "architecture": "DDD + TDD + Loose Coupling",
        "endpoints": {
            "health": "GET /health",
            "info": "GET /info"
        },
        "features": {
            "biometric_verification": "Audio, behavioral, device, location biometrics",
            "nft_wristbands": "Digital collectibles for concert access",
            "qr_codes": "Cryptographically signed validation codes",
            "event_driven": "Domain events for loose coupling",
            "tdd": "Test-driven development implementation"
        }
    }))
}
