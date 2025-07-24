use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::bounded_contexts::payment::presentation::controllers::{PaymentController, RoyaltyController};

#[tokio::test]
async fn test_initiate_payment() {
    let payment_controller = PaymentController::new();
    
    let payment_data = json!({
        "payer_id": Uuid::new_v4().to_string(),
        "payee_id": Uuid::new_v4().to_string(),
        "amount": 1000,
        "currency": "USD",
        "payment_method": "stripe",
        "purpose": "nft_purchase"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/payments")
        .header("content-type", "application/json")
        .body(Body::from(payment_data.to_string()))
        .unwrap();

    let response = payment_controller
        .clone()
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_process_payment() {
    let payment_controller = PaymentController::new();
    let payment_id = Uuid::new_v4();
    
    let request = Request::builder()
        .method("POST")
        .uri(format!("/payments/{}/process", payment_id))
        .body(Body::empty())
        .unwrap();

    let response = payment_controller
        .clone()
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_payment() {
    let payment_controller = PaymentController::new();
    let payment_id = Uuid::new_v4();
    
    let request = Request::builder()
        .method("GET")
        .uri(format!("/payments/{}", payment_id))
        .body(Body::empty())
        .unwrap();

    let response = payment_controller
        .clone()
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_payment_statistics() {
    let payment_controller = PaymentController::new();
    
    let request = Request::builder()
        .method("GET")
        .uri("/payments/statistics")
        .body(Body::empty())
        .unwrap();

    let response = payment_controller
        .clone()
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_royalty_distribution() {
    let royalty_controller = RoyaltyController::new();
    
    let distribution_data = json!({
        "artist_id": Uuid::new_v4().to_string(),
        "song_id": Uuid::new_v4().to_string(),
        "total_revenue": 5000,
        "period_start": "2024-01-01T00:00:00Z",
        "period_end": "2024-01-31T23:59:59Z"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/royalties/distribute")
        .header("content-type", "application/json")
        .body(Body::from(distribution_data.to_string()))
        .unwrap();

    let response = royalty_controller
        .clone()
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_artist_revenue_summary() {
    let royalty_controller = RoyaltyController::new();
    let artist_id = Uuid::new_v4();
    
    let request = Request::builder()
        .method("GET")
        .uri(format!("/royalties/artist/{}/summary", artist_id))
        .body(Body::empty())
        .unwrap();

    let response = royalty_controller
        .clone()
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
} 