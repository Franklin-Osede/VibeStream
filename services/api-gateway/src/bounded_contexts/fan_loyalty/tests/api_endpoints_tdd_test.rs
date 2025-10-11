//! API Endpoints TDD Tests for Fan Loyalty System
//! 
//! TDD RED PHASE - Write tests first, then implement

use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
    Router,
};
use tower::ServiceExt;
use serde_json::json;
use crate::bounded_contexts::fan_loyalty::application::real_dependency_injection::{RealFanLoyaltyContainer, RealFanLoyaltyFactory};
use crate::bounded_contexts::fan_loyalty::domain::{FanId, WristbandType, BiometricData, BehavioralPatterns, DeviceCharacteristics};

/// Test API endpoints - TDD RED PHASE
#[tokio::test]
async fn test_verify_fan_endpoint() {
    // TDD RED PHASE: This test should fail initially
    // It will pass once we implement the endpoint
    
    let app = create_test_app().await;
    
    let request_body = json!({
        "fan_id": "fan_123",
        "biometric_data": {
            "audio_sample": "base64_audio_data",
            "behavioral_patterns": {
                "listening_duration": 300,
                "skip_frequency": 0.1,
                "volume_preferences": [0.7, 0.8, 0.9],
                "time_of_day_patterns": ["evening", "night"]
            },
            "device_characteristics": {
                "device_type": "mobile",
                "os_version": "iOS 17.0",
                "app_version": "1.0.0",
                "hardware_fingerprint": "device_fingerprint_123"
            }
        },
        "device_id": "test_device"
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/verify")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["is_verified"].as_bool().unwrap());
    assert!(response_json["wristband_eligible"].as_bool().unwrap());
    assert!(response_json["confidence_score"].as_f64().unwrap() > 0.9);
    
    println!("✅ POST /api/fan-loyalty/verify endpoint working");
}

#[tokio::test]
async fn test_create_wristband_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    let request_body = json!({
        "fan_id": "fan_123",
        "concert_id": "concert_456",
        "artist_id": "artist_789",
        "wristband_type": "VIP",
        "fan_wallet_address": "0xfan_wallet_address"
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/wristbands")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["wristband_id"].is_string());
    assert_eq!(response_json["fan_id"], "fan_123");
    assert_eq!(response_json["concert_id"], "concert_456");
    assert_eq!(response_json["artist_id"], "artist_789");
    assert_eq!(response_json["wristband_type"], "VIP");
    assert!(!response_json["is_active"].as_bool().unwrap());
    
    println!("✅ POST /api/fan-loyalty/wristbands endpoint working");
}

#[tokio::test]
async fn test_activate_wristband_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    // First create a wristband
    let create_body = json!({
        "fan_id": "fan_123",
        "concert_id": "concert_456",
        "artist_id": "artist_789",
        "wristband_type": "VIP",
        "fan_wallet_address": "0xfan_wallet_address"
    });
    
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/wristbands")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&create_body).unwrap()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let wristband_id = create_json["wristband_id"].as_str().unwrap();
    
    // Now activate the wristband
    let activate_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/fan-loyalty/wristbands/{}/activate", wristband_id))
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();
    
    let activate_response = app.oneshot(activate_request).await.unwrap();
    
    assert_eq!(activate_response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(activate_response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["success"].as_bool().unwrap());
    assert_eq!(response_json["wristband_id"], wristband_id);
    
    println!("✅ POST /api/fan-loyalty/wristbands/{}/activate endpoint working", wristband_id);
}

#[tokio::test]
async fn test_get_wristband_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    // First create a wristband
    let create_body = json!({
        "fan_id": "fan_123",
        "concert_id": "concert_456",
        "artist_id": "artist_789",
        "wristband_type": "VIP",
        "fan_wallet_address": "0xfan_wallet_address"
    });
    
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/wristbands")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&create_body).unwrap()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let wristband_id = create_json["wristband_id"].as_str().unwrap();
    
    // Now get the wristband
    let get_request = Request::builder()
        .method(Method::GET)
        .uri(&format!("/api/fan-loyalty/wristbands/{}", wristband_id))
        .body(Body::empty())
        .unwrap();
    
    let get_response = app.oneshot(get_request).await.unwrap();
    
    assert_eq!(get_response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["wristband_id"], wristband_id);
    assert_eq!(response_json["fan_id"], "fan_123");
    assert_eq!(response_json["concert_id"], "concert_456");
    assert_eq!(response_json["artist_id"], "artist_789");
    assert_eq!(response_json["wristband_type"], "VIP");
    
    println!("✅ GET /api/fan-loyalty/wristbands/{} endpoint working", wristband_id);
}

#[tokio::test]
async fn test_generate_qr_code_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    // First create a wristband
    let create_body = json!({
        "fan_id": "fan_123",
        "concert_id": "concert_456",
        "artist_id": "artist_789",
        "wristband_type": "VIP",
        "fan_wallet_address": "0xfan_wallet_address"
    });
    
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/wristbands")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&create_body).unwrap()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let wristband_id = create_json["wristband_id"].as_str().unwrap();
    
    // Now generate QR code
    let qr_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/fan-loyalty/wristbands/{}/qr-code", wristband_id))
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();
    
    let qr_response = app.oneshot(qr_request).await.unwrap();
    
    assert_eq!(qr_response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(qr_response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["qr_code"].is_string());
    assert!(response_json["qr_code"].as_str().unwrap().starts_with("QR_"));
    assert_eq!(response_json["wristband_id"], wristband_id);
    
    println!("✅ POST /api/fan-loyalty/wristbands/{}/qr-code endpoint working", wristband_id);
}

#[tokio::test]
async fn test_validate_qr_code_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    // First create a wristband and generate QR code
    let create_body = json!({
        "fan_id": "fan_123",
        "concert_id": "concert_456",
        "artist_id": "artist_789",
        "wristband_type": "VIP",
        "fan_wallet_address": "0xfan_wallet_address"
    });
    
    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/wristbands")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&create_body).unwrap()))
        .unwrap();
    
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let create_body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let wristband_id = create_json["wristband_id"].as_str().unwrap();
    
    // Generate QR code
    let qr_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/fan-loyalty/wristbands/{}/qr-code", wristband_id))
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();
    
    let qr_response = app.clone().oneshot(qr_request).await.unwrap();
    let qr_body = hyper::body::to_bytes(qr_response.into_body()).await.unwrap();
    let qr_json: serde_json::Value = serde_json::from_slice(&qr_body).unwrap();
    let qr_code = qr_json["qr_code"].as_str().unwrap();
    
    // Now validate the QR code
    let validate_body = json!({
        "qr_code": qr_code
    });
    
    let validate_request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/validate-qr")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&validate_body).unwrap()))
        .unwrap();
    
    let validate_response = app.oneshot(validate_request).await.unwrap();
    
    assert_eq!(validate_response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(validate_response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["is_valid"].as_bool().unwrap());
    assert_eq!(response_json["wristband_id"], wristband_id);
    assert_eq!(response_json["fan_id"], "fan_123");
    assert_eq!(response_json["concert_id"], "concert_456");
    assert_eq!(response_json["artist_id"], "artist_789");
    assert_eq!(response_json["wristband_type"], "VIP");
    
    println!("✅ POST /api/fan-loyalty/validate-qr endpoint working");
}

#[tokio::test]
async fn test_get_fan_verification_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    // First verify a fan
    let verify_body = json!({
        "fan_id": "fan_123",
        "biometric_data": {
            "audio_sample": "base64_audio_data",
            "behavioral_patterns": {
                "listening_duration": 300,
                "skip_frequency": 0.1,
                "volume_preferences": [0.7, 0.8, 0.9],
                "time_of_day_patterns": ["evening", "night"]
            },
            "device_characteristics": {
                "device_type": "mobile",
                "os_version": "iOS 17.0",
                "app_version": "1.0.0",
                "hardware_fingerprint": "device_fingerprint_123"
            }
        },
        "device_id": "test_device"
    });
    
    let verify_request = Request::builder()
        .method(Method::POST)
        .uri("/api/fan-loyalty/verify")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&verify_body).unwrap()))
        .unwrap();
    
    app.clone().oneshot(verify_request).await.unwrap();
    
    // Now get the verification
    let get_request = Request::builder()
        .method(Method::GET)
        .uri("/api/fan-loyalty/verify/fan_123")
        .body(Body::empty())
        .unwrap();
    
    let get_response = app.oneshot(get_request).await.unwrap();
    
    assert_eq!(get_response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(response_json["is_verified"].as_bool().unwrap());
    assert!(response_json["wristband_eligible"].as_bool().unwrap());
    assert!(response_json["confidence_score"].as_f64().unwrap() > 0.9);
    
    println!("✅ GET /api/fan-loyalty/verify/fan_123 endpoint working");
}

#[tokio::test]
async fn test_health_check_endpoint() {
    // TDD RED PHASE: This test should fail initially
    
    let app = create_test_app().await;
    
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/fan-loyalty/health")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["status"], "healthy");
    assert_eq!(response_json["service"], "fan-loyalty");
    
    println!("✅ GET /api/fan-loyalty/health endpoint working");
}

/// Helper function to create test app
async fn create_test_app() -> Router {
    // TDD GREEN PHASE: Create test app with real database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/vibestream_test".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    let redis_client = redis::Client::open("redis://localhost:6379/")
        .expect("Failed to create Redis client");
    
    let container = crate::bounded_contexts::fan_loyalty::application::real_dependency_injection::RealFanLoyaltyFactory::create_container(pool, redis_client);
    
    // Create router with API prefix
    Router::new()
        .nest("/api/fan-loyalty", crate::bounded_contexts::fan_loyalty::infrastructure::api_handlers::create_fan_loyalty_router(container))
}
