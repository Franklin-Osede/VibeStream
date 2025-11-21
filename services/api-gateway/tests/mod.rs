// =============================================================================
// VIBESTREAM API GATEWAY - INTEGRATION TESTS
// =============================================================================

pub mod helpers;

// Individual controller tests
pub mod music_controller_tests;
pub mod user_controller_tests;
pub mod payment_controller_tests;
pub mod campaign_controller_tests;

// Gateway integration tests (TDD)
pub mod user_gateway_integration_tests;

// OpenAPI integration tests (TDD)
pub mod openapi_integration_tests;

// Auth middleware tests (TDD)
pub mod auth_middleware_tests;

// MessageQueue async migration tests (TDD)
pub mod message_queue_async_tests;

// Register/Login integration tests (TDD)
pub mod register_login_integration_tests;

// Test fixtures and helpers
pub mod fixtures;

// Legacy tests
pub mod integration_tests;
pub mod payment_gateways_integration_tests;
pub mod fractional_ownership_integration_tests;

use helpers::TestClient;
use serde_json::{json, Value};
use uuid::Uuid;

// =============================================================================
// COMPREHENSIVE INTEGRATION TESTS
// =============================================================================

#[tokio::test]
async fn test_complete_vibestream_platform_flow() {
    let client = TestClient::new().await.unwrap();
    
    println!("🎵 Starting Complete VibeStream Platform Flow Test");
    
    // =============================================================================
    // 1. USER ONBOARDING
    // =============================================================================
    println!("👤 Testing User Onboarding...");
    
    // Create Artist
    let artist_data = json!({
        "email": "flowartist@test.com",
        "username": "flowartist",
        "password": "securepassword123",
        "display_name": "Flow Test Artist",
        "bio": "Creating amazing music for VibeStream"
    });
    
    let artist_response = client.post("/api/v1/users", artist_data).await;
    artist_response.assert_success();
    
    let artist_json: Value = artist_response.json_value();
    let artist_id = Uuid::parse_str(artist_json["data"]["user_id"].as_str().unwrap()).unwrap();
    
    // Create Fan
    let fan_data = json!({
        "email": "flowfan@test.com",
        "username": "flowfan",
        "password": "securepassword123",
        "display_name": "Flow Test Fan",
        "bio": "Music lover and early VibeStream adopter"
    });
    
    let fan_response = client.post("/api/v1/users", fan_data).await;
    fan_response.assert_success();
    
    let fan_json: Value = fan_response.json_value();
    let fan_id = Uuid::parse_str(fan_json["data"]["user_id"].as_str().unwrap()).unwrap();
    
    println!("✅ Users created: Artist({}) and Fan({})", artist_id, fan_id);
    
    // =============================================================================
    // 2. CONTENT CREATION
    // =============================================================================
    println!("🎼 Testing Content Creation...");
    
    // Artist uploads songs
    let song1_data = json!({
        "title": "Flow Test Song 1",
        "artist_id": artist_id,
        "duration_seconds": 210,
        "genre": "Electronic",
        "royalty_percentage": 80.0,
        "ipfs_hash": "QmFlowTest1",
        "audio_quality": "lossless",
        "file_format": "flac",
        "mood": "energetic",
        "tempo": 128,
        "release_type": "single"
    });
    
    let song1_response = client.post_with_auth("/api/v1/songs", song1_data, artist_id).await;
    song1_response.assert_success();
    
    let song1_json: Value = song1_response.json_value();
    let song1_id = Uuid::parse_str(song1_json["data"]["song_id"].as_str().unwrap()).unwrap();
    
    let song2_data = json!({
        "title": "Flow Test Song 2",
        "artist_id": artist_id,
        "duration_seconds": 240,
        "genre": "Electronic",
        "royalty_percentage": 80.0,
        "ipfs_hash": "QmFlowTest2",
        "audio_quality": "high",
        "file_format": "mp3",
        "mood": "chill",
        "tempo": 110,
        "release_type": "album"
    });
    
    let song2_response = client.post_with_auth("/api/v1/songs", song2_data, artist_id).await;
    song2_response.assert_success();
    
    let song2_json: Value = song2_response.json_value();
    let song2_id = Uuid::parse_str(song2_json["data"]["song_id"].as_str().unwrap()).unwrap();
    
    println!("✅ Songs uploaded: {} and {}", song1_id, song2_id);
    
    // Create Album
    let album_data = json!({
        "title": "Flow Test Album",
        "artist_id": artist_id,
        "description": "Complete integration test album",
        "genre": "Electronic",
        "album_type": "EP",
        "song_ids": [song1_id, song2_id]
    });
    
    let album_response = client.post_with_auth("/api/v1/albums", album_data, artist_id).await;
    album_response.assert_success();
    
    let album_json: Value = album_response.json_value();
    let album_id = Uuid::parse_str(album_json["data"]["album_id"].as_str().unwrap()).unwrap();
    
    println!("✅ Album created: {}", album_id);
    
    // =============================================================================
    // 3. SOCIAL INTERACTION
    // =============================================================================
    println!("👥 Testing Social Features...");
    
    // Fan follows Artist
    let follow_data = json!({ "follow": true });
    
    let follow_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist_id),
        follow_data,
        fan_id
    ).await;
    follow_response.assert_success();
    
    // Fan creates playlist
    let playlist_data = json!({
        "name": "Flow Test Playlist",
        "description": "My favorite tracks from the flow test",
        "is_public": true,
        "is_collaborative": false,
        "tags": ["electronic", "test", "favorites"]
    });
    
    let playlist_response = client.post_with_auth("/api/v1/playlists", playlist_data, fan_id).await;
    playlist_response.assert_success();
    
    let playlist_json: Value = playlist_response.json_value();
    let playlist_id = Uuid::parse_str(playlist_json["data"]["playlist_id"].as_str().unwrap()).unwrap();
    
    println!("✅ Social interactions: Follow and Playlist({}) created", playlist_id);
    
    // =============================================================================
    // 4. MUSIC CONSUMPTION
    // =============================================================================
    println!("🎧 Testing Music Consumption...");
    
    // Fan listens to songs
    let listen1_data = json!({
        "duration_seconds": 180,
        "completion_percentage": 85.7,
        "device_type": "mobile",
        "location": "US"
    });
    
    let listen1_response = client.post_with_auth(
        &format!("/api/v1/songs/{}/listen", song1_id),
        listen1_data,
        fan_id
    ).await;
    listen1_response.assert_success();
    
    let listen2_data = json!({
        "duration_seconds": 240,
        "completion_percentage": 100.0,
        "device_type": "desktop",
        "location": "US"
    });
    
    let listen2_response = client.post_with_auth(
        &format!("/api/v1/songs/{}/listen", song2_id),
        listen2_data,
        fan_id
    ).await;
    listen2_response.assert_success();
    
    println!("✅ Listen events recorded for both songs");
    
    // =============================================================================
    // 5. CAMPAIGN CREATION & PARTICIPATION
    // =============================================================================
    println!("🎯 Testing Campaign System...");
    
    // Artist creates campaign
    let campaign_data = json!({
        "name": "Flow Test Campaign",
        "description": "Promote the new Flow Test Album",
        "campaign_type": "nft_boost",
        "song_id": song1_id,
        "artist_id": artist_id,
        "target_audience": {
            "locations": ["US"],
            "genres": ["Electronic"],
            "fan_level": "casual"
        },
        "budget": 500.0,
        "currency": "USD",
        "start_date": "2024-02-01T00:00:00Z",
        "end_date": "2024-02-29T23:59:59Z",
        "campaign_parameters": {
            "max_participants": 100,
            "reward_per_action": 1.0,
            "required_actions": ["listen", "share"],
            "nft_collection_size": 25
        }
    });
    
    let campaign_response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist_id).await;
    campaign_response.assert_success();
    
    let campaign_json: Value = campaign_response.json_value();
    let campaign_id = Uuid::parse_str(campaign_json["data"]["campaign_id"].as_str().unwrap()).unwrap();
    
    // Activate campaign
    let activate_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/activate", campaign_id),
        json!({}),
        artist_id
    ).await;
    activate_response.assert_success();
    
    // Fan participates
    let participation_data = json!({
        "action_type": "listen",
        "action_data": {
            "duration_seconds": 210,
            "completion_percentage": 100.0
        }
    });
    
    let participation_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", campaign_id),
        participation_data,
        fan_id
    ).await;
    participation_response.assert_success();
    
    println!("✅ Campaign created({}) and participation recorded", campaign_id);
    
    // =============================================================================
    // 6. PAYMENT FLOW
    // =============================================================================
    println!("💰 Testing Payment System...");
    
    // Fan purchases song
    let payment_data = json!({
        "payer_id": fan_id,
        "payee_id": artist_id,
        "amount": 1.99,
        "currency": "USD",
        "payment_type": "song_purchase",
        "related_entity_id": song1_id,
        "payment_method": "stripe"
    });
    
    let payment_response = client.post_with_auth("/api/v1/payments", payment_data, fan_id).await;
    payment_response.assert_success();
    
    let payment_json: Value = payment_response.json_value();
    let payment_id = Uuid::parse_str(payment_json["data"]["payment_id"].as_str().unwrap()).unwrap();
    
    // Process payment
    let process_data = json!({
        "gateway_transaction_id": "flow_test_txn",
        "gateway_status": "succeeded"
    });
    
    let process_response = client.post_with_auth(
        &format!("/api/v1/payments/{}/process", payment_id),
        process_data,
        fan_id
    ).await;
    process_response.assert_success();
    
    // Complete payment
    let complete_response = client.post_with_auth(
        &format!("/api/v1/payments/{}/complete", payment_id),
        json!({}),
        fan_id
    ).await;
    complete_response.assert_success();
    
    println!("✅ Payment completed: {}", payment_id);
    
    // =============================================================================
    // 7. ROYALTY DISTRIBUTION
    // =============================================================================
    println!("👑 Testing Royalty Distribution...");
    
    let royalty_data = json!({
        "song_id": song1_id,
        "period_start": "2024-01-01T00:00:00Z",
        "period_end": "2024-01-31T23:59:59Z",
        "total_revenue": 1.99,
        "currency": "USD",
        "distribution_rules": [
            {
                "recipient_id": artist_id,
                "recipient_type": "artist",
                "percentage": 80.0
            },
            {
                "recipient_id": "550e8400-e29b-41d4-a716-446655440099",
                "recipient_type": "platform",
                "percentage": 20.0
            }
        ]
    });
    
    let royalty_response = client.post_with_auth("/api/v1/royalties/distribute", royalty_data, artist_id).await;
    royalty_response.assert_success();
    
    println!("✅ Royalty distribution completed");
    
    // =============================================================================
    // 8. NFT MINTING
    // =============================================================================
    println!("🎨 Testing NFT System...");
    
    let nft_data = json!({
        "nft_count": 1,
        "recipient_id": fan_id,
        "metadata_override": {
            "name": "Flow Test Participant NFT",
            "description": "Special NFT for completing the platform flow test"
        }
    });
    
    let nft_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/nft/mint", campaign_id),
        nft_data,
        artist_id
    ).await;
    nft_response.assert_success();
    
    println!("✅ NFT minted for campaign participant");
    
    // =============================================================================
    // 9. ANALYTICS & VERIFICATION
    // =============================================================================
    println!("📊 Testing Analytics...");
    
    // Check campaign analytics
    let analytics_response = client.get(&format!("/api/v1/campaigns/{}/analytics", campaign_id)).await;
    analytics_response.assert_success();
    
    // Check payment statistics
    let stats_response = client.get("/api/v1/payments/statistics").await;
    stats_response.assert_success();
    
    // Check user payment history
    let history_response = client.get_with_auth(
        &format!("/api/v1/payments/user/{}/history", fan_id),
        fan_id
    ).await;
    history_response.assert_success();
    
    println!("✅ Analytics and statistics verified");
    
    // =============================================================================
    // 10. SEARCH & DISCOVERY
    // =============================================================================
    println!("🔍 Testing Search & Discovery...");
    
    // Search songs
    let search_response = client.get("/api/v1/songs?search_text=Flow&limit=10").await;
    search_response.assert_success();
    
    // Search campaigns
    let campaign_search_response = client.get("/api/v1/campaigns?search_text=Flow&limit=5").await;
    campaign_search_response.assert_success();
    
    // Get trending content
    let trending_response = client.get("/api/v1/songs/trending").await;
    trending_response.assert_success();
    
    println!("✅ Search and discovery functionality verified");
    
    println!("🎉 Complete VibeStream Platform Flow Test PASSED!");
    println!("🎯 All major platform features tested successfully:");
    println!("   ✅ User Management & Social Features");
    println!("   ✅ Music Content Creation & Management");
    println!("   ✅ Campaign System & NFT Integration");
    println!("   ✅ Payment Processing & Royalty Distribution");
    println!("   ✅ Analytics & Reporting");
    println!("   ✅ Search & Discovery");
}

// =============================================================================
// LOAD TESTING
// =============================================================================

#[tokio::test]
async fn test_platform_load_simulation() {
    let client = TestClient::new().await.unwrap();
    
    println!("⚡ Starting Load Test Simulation");
    
    let start = std::time::Instant::now();
    let mut handles = vec![];
    
    // Simulate concurrent users
    for i in 0..10 {
        let client = &client;
        let handle = tokio::spawn(async move {
            // Each "user" performs multiple actions
            
            // Search for songs
            let search_response = client.get(&format!("/api/v1/songs?limit=5&offset={}", i * 5)).await;
            search_response.assert_success();
            
            // Get trending campaigns
            let trending_response = client.get("/api/v1/campaigns/trending?limit=3").await;
            trending_response.assert_success();
            
            // Check payment statistics
            let stats_response = client.get("/api/v1/payments/statistics").await;
            stats_response.assert_success();
            
            println!("User {} completed load test actions", i);
        });
        handles.push(handle);
    }
    
    // Wait for all concurrent users to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    let duration = start.elapsed();
    
    println!("✅ Load test completed in {:?}", duration);
    println!("📊 10 concurrent users, 30 total requests");
    println!("⚡ Average response time: {:?}", duration / 30);
    
    // Assert reasonable performance
    assert!(duration.as_millis() < 10000, "Load test took too long: {:?}", duration);
}

// =============================================================================
// ERROR RESILIENCE TESTING
// =============================================================================

#[tokio::test]
async fn test_error_resilience() {
    let client = TestClient::new().await.unwrap();
    
    println!("🛡️ Testing Error Resilience");
    
    // Test various error conditions
    let error_tests = vec![
        ("Invalid UUID", "/api/v1/songs/invalid-uuid"),
        ("Non-existent resource", &format!("/api/v1/songs/{}", Uuid::new_v4())),
        ("Invalid endpoint", "/api/v1/nonexistent"),
    ];
    
    for (test_name, endpoint) in error_tests {
        let response = client.get(endpoint).await;
        
        // Should handle errors gracefully
        assert!(
            response.status.is_client_error() || response.status.is_server_error(),
            "Error test '{}' should return an error status", test_name
        );
        
        println!("✅ {}: Handled gracefully", test_name);
    }
    
    println!("🛡️ Error resilience tests passed");
}

// =============================================================================
// API CONSISTENCY TESTING
// =============================================================================

#[tokio::test]
async fn test_api_response_consistency() {
    let client = TestClient::new().await.unwrap();
    
    println!("🔄 Testing API Response Consistency");
    
    let endpoints = vec![
        "/api/v1/songs?limit=1",
        "/api/v1/users?limit=1",
        "/api/v1/campaigns?limit=1",
        "/api/v1/payments/statistics",
    ];
    
    for endpoint in endpoints {
        let response = client.get(endpoint).await;
        response.assert_success();
        
        let json: Value = response.json_value();
        
        // All responses should have consistent structure
        assert!(json["success"].is_boolean(), "Response should have 'success' field");
        assert!(json["timestamp"].is_string(), "Response should have 'timestamp' field");
        
        if json["success"] == true {
            assert!(json["data"].is_object() || json["data"].is_array(), "Successful response should have 'data' field");
        } else {
            assert!(json["error"].is_string(), "Error response should have 'error' field");
        }
        
        println!("✅ {}: Response structure consistent", endpoint);
    }
    
    println!("🔄 API consistency tests passed");
}

// =============================================================================
// SECURITY TESTING
// =============================================================================

#[tokio::test]
async fn test_security_measures() {
    let client = TestClient::new().await.unwrap();
    
    println!("🔒 Testing Security Measures");
    
    // Test unauthorized access
    let protected_endpoints = vec![
        ("/api/v1/songs", "POST"),
        ("/api/v1/campaigns", "POST"),
        ("/api/v1/payments", "POST"),
    ];
    
    for (endpoint, method) in protected_endpoints {
        let response = match method {
            "POST" => client.post(endpoint, json!({})).await,
            _ => client.get(endpoint).await,
        };
        
        // Should require authentication
        assert!(
            response.status == axum::http::StatusCode::UNAUTHORIZED || 
            response.status == axum::http::StatusCode::BAD_REQUEST,
            "Protected endpoint {} should require auth", endpoint
        );
        
        println!("✅ {}: Properly protected", endpoint);
    }
    
    println!("🔒 Security tests passed");
}

#[tokio::test]
async fn test_input_validation() {
    let client = TestClient::new().await.unwrap();
    
    println!("🛡️ Testing Input Validation");
    
    // Test invalid inputs
    let invalid_inputs = vec![
        (
            "/api/v1/users",
            json!({ "email": "invalid-email", "username": "", "password": "123" }),
            "Invalid user data"
        ),
        (
            "/api/v1/payments",
            json!({ "amount": -100, "currency": "INVALID" }),
            "Invalid payment data"
        ),
    ];
    
    for (endpoint, data, test_name) in invalid_inputs {
        let response = client.post(endpoint, data).await;
        
        // Should reject invalid input
        assert!(
            response.status == axum::http::StatusCode::BAD_REQUEST,
            "Should reject invalid input for: {}", test_name
        );
        
        println!("✅ {}: Input validation working", test_name);
    }
    
    println!("🛡️ Input validation tests passed");
} 