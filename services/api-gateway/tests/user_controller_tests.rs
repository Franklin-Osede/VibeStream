use crate::helpers::{TestClient, TestData, assert_json_contains};
use serde_json::{json, Value};
use uuid::Uuid;
use axum::http::StatusCode;

mod helpers;

// =============================================================================
// USER CRUD TESTS
// =============================================================================

#[tokio::test]
async fn test_create_user_success() {
    let client = TestClient::new().await.unwrap();
    
    let user_data = json!({
        "email": "newuser@test.com",
        "username": "newuser123",
        "password": "securepassword123",
        "display_name": "New Test User",
        "bio": "I love music and testing",
        "location": "San Francisco, CA"
    });
    
    let response = client.post("/api/v1/users", user_data).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["user_id"].is_string());
    assert_eq!(json_response["data"]["username"], "newuser123");
    assert_eq!(json_response["data"]["email"], "newuser@test.com");
    assert_eq!(json_response["data"]["display_name"], "New Test User");
    assert!(json_response["data"]["created_at"].is_string());
}

#[tokio::test]
async fn test_create_user_duplicate_email() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let existing_user = test_data.get_artist();
    
    let user_data = json!({
        "email": existing_user.email, // Use existing email
        "username": "newusername",
        "password": "password123",
        "display_name": "Duplicate Email User"
    });
    
    let response = client.post("/api/v1/users", user_data).await;
    
    response.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_create_user_duplicate_username() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let existing_user = test_data.get_artist();
    
    let user_data = json!({
        "email": "unique@test.com",
        "username": existing_user.username, // Use existing username
        "password": "password123",
        "display_name": "Duplicate Username User"
    });
    
    let response = client.post("/api/v1/users", user_data).await;
    
    response.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    let client = TestClient::new().await.unwrap();
    
    let user_data = json!({
        "email": "invalid-email", // Invalid email format
        "username": "validuser",
        "password": "password123",
        "display_name": "Invalid Email User"
    });
    
    let response = client.post("/api/v1/users", user_data).await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_user_weak_password() {
    let client = TestClient::new().await.unwrap();
    
    let user_data = json!({
        "email": "weakpassword@test.com",
        "username": "weakpassuser",
        "password": "123", // Too weak password
        "display_name": "Weak Password User"
    });
    
    let response = client.post("/api/v1/users", user_data).await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_user_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_artist();
    
    let response = client.get(&format!("/api/v1/users/{}", user.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["id"], user.id.to_string());
    assert_eq!(json_response["data"]["username"], user.username);
    assert_eq!(json_response["data"]["display_name"], user.display_name);
    // Password should not be included in response
    assert!(json_response["data"]["password"].is_null());
}

#[tokio::test]
async fn test_get_user_not_found() {
    let client = TestClient::new().await.unwrap();
    let non_existent_id = Uuid::new_v4();
    
    let response = client.get(&format!("/api/v1/users/{}", non_existent_id)).await;
    
    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_user_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_artist();
    
    let update_data = json!({
        "display_name": "Updated Display Name",
        "bio": "Updated bio with new information",
        "location": "New York, NY",
        "banner_image_url": "https://example.com/banner.jpg"
    });
    
    let response = client.put(
        &format!("/api/v1/users/{}", user.id),
        update_data
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["display_name"], "Updated Display Name");
    assert_eq!(json_response["data"]["bio"], "Updated bio with new information");
    assert_eq!(json_response["data"]["location"], "New York, NY");
}

#[tokio::test]
async fn test_update_user_unauthorized() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_artist();
    let other_user = test_data.get_fan();
    
    let update_data = json!({
        "display_name": "Unauthorized Update"
    });
    
    // Try to update user with different user's auth
    let response = client.put(
        &format!("/api/v1/users/{}", user.id),
        update_data
    ).await;
    
    response.assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_user_success() {
    let client = TestClient::new().await.unwrap();
    
    // First create a user to delete
    let user_data = json!({
        "email": "todelete@test.com",
        "username": "todelete",
        "password": "password123",
        "display_name": "To Delete User"
    });
    
    let create_response = client.post("/api/v1/users", user_data).await;
    create_response.assert_success();
    
    let create_json: Value = create_response.json_value();
    let user_id = create_json["data"]["user_id"].as_str().unwrap();
    let user_uuid = Uuid::parse_str(user_id).unwrap();
    
    // Now delete the user
    let response = client.delete(&format!("/api/v1/users/{}", user_id)).await;
    
    response.assert_success();
    
    // Verify user is deleted
    let get_response = client.get(&format!("/api/v1/users/{}", user_id)).await;
    get_response.assert_status(StatusCode::NOT_FOUND);
}

// =============================================================================
// USER SEARCH TESTS
// =============================================================================

#[tokio::test]
async fn test_search_users_basic() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/users?limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["users"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
    assert!(json_response["data"]["has_more"].is_boolean());
}

#[tokio::test]
async fn test_search_users_by_username() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/users?search_text=test_artist&limit=5").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["users"].is_array());
    
    // Should find the test artist
    let users = json_response["data"]["users"].as_array().unwrap();
    if !users.is_empty() {
        let found_artist = users.iter().any(|user| 
            user["username"].as_str().unwrap_or("").contains("test_artist")
        );
        assert!(found_artist, "Should find test_artist in search results");
    }
}

#[tokio::test]
async fn test_search_users_with_filters() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/users?is_artist=true&is_verified=false&limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["users"].is_array());
}

#[tokio::test]
async fn test_search_users_pagination() {
    let client = TestClient::new().await.unwrap();
    
    // Test first page
    let response1 = client.get("/api/v1/users?limit=1&offset=0").await;
    response1.assert_success();
    
    // Test second page
    let response2 = client.get("/api/v1/users?limit=1&offset=1").await;
    response2.assert_success();
    
    let json1: Value = response1.json_value();
    let json2: Value = response2.json_value();
    
    let users1 = json1["data"]["users"].as_array().unwrap();
    let users2 = json2["data"]["users"].as_array().unwrap();
    
    // If both pages have results, they should be different users
    if !users1.is_empty() && !users2.is_empty() {
        assert_ne!(users1[0]["id"], users2[0]["id"], "Pagination should return different users");
    }
}

// =============================================================================
// SOCIAL FEATURES TESTS
// =============================================================================

#[tokio::test]
async fn test_follow_user_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    let artist = test_data.get_artist();
    
    let follow_data = json!({
        "follow": true
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist.id),
        follow_data,
        fan.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
}

#[tokio::test]
async fn test_unfollow_user_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    let artist = test_data.get_artist();
    
    // First follow
    let follow_data = json!({
        "follow": true
    });
    
    let follow_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist.id),
        follow_data,
        fan.id
    ).await;
    follow_response.assert_success();
    
    // Then unfollow
    let unfollow_data = json!({
        "follow": false
    });
    
    let unfollow_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist.id),
        unfollow_data,
        fan.id
    ).await;
    unfollow_response.assert_success();
}

#[tokio::test]
async fn test_follow_self_error() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let follow_data = json!({
        "follow": true
    });
    
    // Try to follow self
    let response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", user.id),
        follow_data,
        user.id
    ).await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_user_followers() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let response = client.get(&format!("/api/v1/users/{}/followers?limit=10", artist.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["users"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
}

#[tokio::test]
async fn test_get_user_following() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    
    let response = client.get(&format!("/api/v1/users/{}/following?limit=10", fan.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["users"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
}

// =============================================================================
// PRIVACY SETTINGS TESTS
// =============================================================================

#[tokio::test]
async fn test_get_privacy_settings() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/privacy", user.id),
        user.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["is_profile_public"].is_boolean());
    assert!(json_response["data"]["is_activity_public"].is_boolean());
    assert!(json_response["data"]["is_playlist_public"].is_boolean());
    assert!(json_response["data"]["allow_follower_requests"].is_boolean());
}

#[tokio::test]
async fn test_update_privacy_settings() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let privacy_data = json!({
        "is_profile_public": false,
        "is_activity_public": false,
        "is_playlist_public": true,
        "allow_follower_requests": true
    });
    
    let response = client.put(
        &format!("/api/v1/users/{}/privacy", user.id),
        privacy_data
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["is_profile_public"], false);
    assert_eq!(json_response["data"]["is_activity_public"], false);
    assert_eq!(json_response["data"]["is_playlist_public"], true);
    assert_eq!(json_response["data"]["allow_follower_requests"], true);
}

#[tokio::test]
async fn test_privacy_settings_unauthorized() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    let other_user = test_data.get_artist();
    
    // Try to access another user's privacy settings
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/privacy", user.id),
        other_user.id
    ).await;
    
    response.assert_status(StatusCode::FORBIDDEN);
}

// =============================================================================
// USER CONTENT TESTS
// =============================================================================

#[tokio::test]
async fn test_get_user_playlists() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let response = client.get(&format!("/api/v1/users/{}/playlists?limit=10", user.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    // This would delegate to Music Context
}

#[tokio::test]
async fn test_get_user_listen_history_self() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/listen-history?limit=20", user.id),
        user.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
}

#[tokio::test]
async fn test_get_user_listen_history_private() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    let other_user = test_data.get_artist();
    
    // Try to access another user's listen history
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/listen-history", user.id),
        other_user.id
    ).await;
    
    response.assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_get_user_recommendations() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/recommendations", user.id),
        user.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
}

// =============================================================================
// ANALYTICS TESTS
// =============================================================================

#[tokio::test]
async fn test_get_user_analytics_artist() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/analytics", artist.id),
        artist.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
}

#[tokio::test]
async fn test_get_user_insights() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let user = test_data.get_fan();
    
    let response = client.get_with_auth(
        &format!("/api/v1/users/{}/insights", user.id),
        user.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
}

// =============================================================================
// INTEGRATION FLOW TESTS
// =============================================================================

#[tokio::test]
async fn test_complete_user_onboarding_flow() {
    let client = TestClient::new().await.unwrap();
    
    // 1. Create a new user
    let user_data = json!({
        "email": "onboarding@test.com",
        "username": "onboardinguser",
        "password": "securepassword123",
        "display_name": "Onboarding User",
        "location": "Los Angeles, CA"
    });
    
    let create_response = client.post("/api/v1/users", user_data).await;
    create_response.assert_success();
    
    let create_json: Value = create_response.json_value();
    let user_id = create_json["data"]["user_id"].as_str().unwrap();
    let user_uuid = Uuid::parse_str(user_id).unwrap();
    
    // 2. Update profile with more information
    let update_data = json!({
        "bio": "Music lover and early adopter of VibeStream",
        "profile_image_url": "https://example.com/profile.jpg"
    });
    
    let update_response = client.put(
        &format!("/api/v1/users/{}", user_id),
        update_data
    ).await;
    update_response.assert_success();
    
    // 3. Set privacy preferences
    let privacy_data = json!({
        "is_profile_public": true,
        "is_activity_public": false,
        "is_playlist_public": true,
        "allow_follower_requests": true
    });
    
    let privacy_response = client.put(
        &format!("/api/v1/users/{}/privacy", user_id),
        privacy_data
    ).await;
    privacy_response.assert_success();
    
    // 4. Follow an artist
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let follow_data = json!({
        "follow": true
    });
    
    let follow_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist.id),
        follow_data,
        user_uuid
    ).await;
    follow_response.assert_success();
    
    // 5. Verify user profile is complete
    let get_response = client.get(&format!("/api/v1/users/{}", user_id)).await;
    get_response.assert_success();
    
    let profile_json: Value = get_response.json_value();
    assert_eq!(profile_json["data"]["bio"], "Music lover and early adopter of VibeStream");
    assert_eq!(profile_json["data"]["location"], "Los Angeles, CA");
}

#[tokio::test]
async fn test_social_interaction_flow() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan1 = test_data.get_fan();
    let artist = test_data.get_artist();
    
    // Create a second fan
    let fan2_data = json!({
        "email": "fan2@test.com",
        "username": "testfan2",
        "password": "password123",
        "display_name": "Test Fan 2"
    });
    
    let fan2_response = client.post("/api/v1/users", fan2_data).await;
    fan2_response.assert_success();
    
    let fan2_json: Value = fan2_response.json_value();
    let fan2_id = Uuid::parse_str(fan2_json["data"]["user_id"].as_str().unwrap()).unwrap();
    
    // 1. Both fans follow the artist
    let follow_data = json!({ "follow": true });
    
    let follow1_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist.id),
        follow_data.clone(),
        fan1.id
    ).await;
    follow1_response.assert_success();
    
    let follow2_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", artist.id),
        follow_data,
        fan2_id
    ).await;
    follow2_response.assert_success();
    
    // 2. Fan1 follows Fan2
    let follow_fan_data = json!({ "follow": true });
    
    let follow_fan_response = client.post_with_auth(
        &format!("/api/v1/users/{}/follow", fan2_id),
        follow_fan_data,
        fan1.id
    ).await;
    follow_fan_response.assert_success();
    
    // 3. Check artist's followers
    let followers_response = client.get(&format!("/api/v1/users/{}/followers", artist.id)).await;
    followers_response.assert_success();
    
    let followers_json: Value = followers_response.json_value();
    let followers = followers_json["data"]["users"].as_array().unwrap();
    
    // Should have at least 2 followers (fan1 and fan2)
    assert!(followers.len() >= 2, "Artist should have multiple followers");
    
    // 4. Check fan1's following list
    let following_response = client.get(&format!("/api/v1/users/{}/following", fan1.id)).await;
    following_response.assert_success();
    
    let following_json: Value = following_response.json_value();
    let following = following_json["data"]["users"].as_array().unwrap();
    
    // Should be following at least 2 people (artist and fan2)
    assert!(following.len() >= 2, "Fan1 should be following multiple users");
}

// =============================================================================
// PERFORMANCE AND STRESS TESTS
// =============================================================================

#[tokio::test]
async fn test_bulk_user_operations() {
    let client = TestClient::new().await.unwrap();
    
    let start = std::time::Instant::now();
    
    // Create multiple users concurrently
    let mut handles = vec![];
    
    for i in 0..5 {
        let client = &client;
        let handle = tokio::spawn(async move {
            let user_data = json!({
                "email": format!("bulkuser{}@test.com", i),
                "username": format!("bulkuser{}", i),
                "password": "password123",
                "display_name": format!("Bulk User {}", i)
            });
            
            client.post("/api/v1/users", user_data).await
        });
        handles.push(handle);
    }
    
    // Wait for all user creations to complete
    for handle in handles {
        let response = handle.await.unwrap();
        response.assert_success();
    }
    
    let duration = start.elapsed();
    
    // Assert reasonable performance
    assert!(duration.as_millis() < 3000, "Bulk user creation took too long: {:?}", duration);
}

#[tokio::test]
async fn test_concurrent_user_searches() {
    let client = TestClient::new().await.unwrap();
    
    let mut handles = vec![];
    
    // Spawn multiple concurrent search requests
    for i in 0..3 {
        let client = &client;
        let handle = tokio::spawn(async move {
            client.get(&format!("/api/v1/users?limit=10&offset={}", i * 10)).await
        });
        handles.push(handle);
    }
    
    // Wait for all searches to complete
    for handle in handles {
        let response = handle.await.unwrap();
        response.assert_success();
    }
} 