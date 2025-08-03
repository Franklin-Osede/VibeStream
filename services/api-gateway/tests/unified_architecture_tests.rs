use axum::{
    body::Body,
    http::{Request, StatusCode},
    test::TestServer,
};
use serde_json::json;
use uuid::Uuid;
use api_gateway::shared::infrastructure::app_state::AppState;

/// Integration tests for unified architecture
/// Following TDD principles: Write tests first, then implement
#[tokio::test]
async fn test_unified_app_state_creation() {
    // Given: We want to create a unified AppState
    // When: We call AppState::default()
    let app_state = AppState::default().await;
    
    // Then: It should be created successfully
    assert!(app_state.is_ok());
    
    let app_state = app_state.unwrap();
    
    // And: All repositories should be initialized
    assert!(app_state.music_repository.find_by_id(&Uuid::new_v4()).await.is_ok());
    assert!(app_state.user_repository.find_by_id(&Uuid::new_v4()).await.is_ok());
    assert!(app_state.campaign_repository.find_by_id(&Uuid::new_v4()).await.is_ok());
    assert!(app_state.artist_venture_repository.find_by_id(&Uuid::new_v4()).await.is_ok());
    assert!(app_state.notification_repository.find_by_id(&Uuid::new_v4()).await.is_ok());
}

#[tokio::test]
async fn test_health_check_endpoint() {
    // Given: A unified AppState
    let app_state = AppState::default().await.unwrap();
    
    // When: We call health_check()
    let health_status = app_state.health_check().await;
    
    // Then: It should return healthy status
    assert!(health_status.is_ok());
    
    let health_status = health_status.unwrap();
    assert_eq!(health_status.overall, "healthy");
    assert_eq!(health_status.database, "healthy");
    assert_eq!(health_status.redis, "healthy");
}

#[tokio::test]
async fn test_fan_ventures_integration_flow() {
    // Given: A unified AppState with all services
    let app_state = AppState::default().await.unwrap();
    
    // When: We create a venture through the application service
    let venture_request = json!({
        "artist_id": Uuid::new_v4(),
        "title": "Test Venture",
        "description": "Test Description",
        "funding_goal": 1000.0,
        "benefits": [
            {
                "title": "Digital Content",
                "description": "Exclusive songs",
                "benefit_type": "DigitalContent",
                "min_investment": 25.0
            }
        ]
    });
    
    // Then: It should be created successfully
    let result = app_state.fan_ventures_service.create_venture(venture_request).await;
    assert!(result.is_ok());
    
    // And: We should be able to retrieve it
    let venture_id = result.unwrap().venture_id;
    let retrieved_venture = app_state.artist_venture_repository.find_by_id(&venture_id).await;
    assert!(retrieved_venture.is_ok());
    assert!(retrieved_venture.unwrap().is_some());
}

#[tokio::test]
async fn test_music_context_integration() {
    // Given: A unified AppState
    let app_state = AppState::default().await.unwrap();
    
    // When: We create a song through the music service
    let song_request = json!({
        "title": "Test Song",
        "artist_id": Uuid::new_v4(),
        "duration": 180,
        "genre": "Rock"
    });
    
    // Then: It should be created successfully
    let result = app_state.music_service.create_song(song_request).await;
    assert!(result.is_ok());
    
    // And: We should be able to discover it
    let discovery_result = app_state.music_service.discover_songs(None, None).await;
    assert!(discovery_result.is_ok());
    assert!(!discovery_result.unwrap().is_empty());
}

#[tokio::test]
async fn test_cross_context_integration() {
    // Given: A unified AppState
    let app_state = AppState::default().await.unwrap();
    
    // When: We create a user, then a venture, then a notification
    let user_id = Uuid::new_v4();
    
    // Create user
    let user_result = app_state.user_service.create_user(json!({
        "user_id": user_id,
        "username": "testuser",
        "email": "test@example.com"
    })).await;
    assert!(user_result.is_ok());
    
    // Create venture
    let venture_result = app_state.fan_ventures_service.create_venture(json!({
        "artist_id": user_id,
        "title": "Cross Context Venture",
        "description": "Testing cross-context integration",
        "funding_goal": 500.0
    })).await;
    assert!(venture_result.is_ok());
    
    // Create notification
    let notification_result = app_state.notification_service.create_notification(json!({
        "user_id": user_id,
        "title": "Venture Created",
        "message": "Your venture has been created successfully",
        "notification_type": "VentureCreated"
    })).await;
    assert!(notification_result.is_ok());
    
    // Then: All operations should be successful and linked
    let user = app_state.user_repository.find_by_id(&user_id).await.unwrap();
    assert!(user.is_some());
    
    let ventures = app_state.artist_venture_repository.find_by_artist(&user_id).await.unwrap();
    assert!(!ventures.is_empty());
    
    let notifications = app_state.notification_repository.find_by_user_id(&user_id).await.unwrap();
    assert!(!notifications.is_empty());
}

#[tokio::test]
async fn test_error_handling_unified() {
    // Given: A unified AppState
    let app_state = AppState::default().await.unwrap();
    
    // When: We try to create invalid data
    let invalid_request = json!({
        "invalid_field": "invalid_value"
    });
    
    // Then: It should handle errors gracefully
    let result = app_state.fan_ventures_service.create_venture(invalid_request).await;
    assert!(result.is_err());
    
    // And: Error should be properly formatted
    let error = result.unwrap_err();
    assert!(error.to_string().contains("validation"));
}

#[tokio::test]
async fn test_performance_unified_architecture() {
    // Given: A unified AppState
    let app_state = AppState::default().await.unwrap();
    
    // When: We perform multiple operations concurrently
    let start = std::time::Instant::now();
    
    let futures = vec![
        app_state.music_service.discover_songs(None, None),
        app_state.fan_ventures_service.get_all_ventures(),
        app_state.user_service.get_all_users(),
        app_state.notification_service.get_all_notifications(),
    ];
    
    let results = futures::future::join_all(futures).await;
    
    let duration = start.elapsed();
    
    // Then: All operations should complete successfully
    for result in results {
        assert!(result.is_ok());
    }
    
    // And: Performance should be acceptable (< 1 second for all operations)
    assert!(duration.as_millis() < 1000);
}

/// Test the complete API flow with HTTP requests
#[tokio::test]
async fn test_complete_api_flow() {
    // Given: A complete router with unified AppState
    let app = api_gateway::create_app().await.unwrap();
    let server = TestServer::new(app).unwrap();
    
    // When: We make requests to different endpoints
    let health_response = server.get("/health").await;
    assert_eq!(health_response.status_code(), StatusCode::OK);
    
    let music_response = server.get("/api/v1/music/songs/discover").await;
    assert_eq!(music_response.status_code(), StatusCode::OK);
    
    let ventures_response = server.get("/api/v1/fan-ventures/ventures").await;
    assert_eq!(ventures_response.status_code(), StatusCode::OK);
    
    // Then: All endpoints should work with unified architecture
    let health_body = health_response.text();
    assert!(health_body.contains("healthy"));
    
    let music_body = music_response.text();
    assert!(music_body.contains("songs"));
    
    let ventures_body = ventures_response.text();
    assert!(ventures_body.contains("ventures"));
} 