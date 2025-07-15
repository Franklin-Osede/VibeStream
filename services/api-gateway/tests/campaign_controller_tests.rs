use crate::helpers::{TestClient, TestData, assert_json_contains};
use serde_json::{json, Value};
use uuid::Uuid;
use axum::http::StatusCode;

mod helpers;

// =============================================================================
// CAMPAIGN CRUD TESTS
// =============================================================================

#[tokio::test]
async fn test_create_campaign_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_song = test_data.get_test_song();
    
    let campaign_data = json!({
        "name": "Test Music Campaign",
        "description": "A campaign to promote our new track",
        "campaign_type": "nft_boost",
        "song_id": test_song.id,
        "artist_id": artist.id,
        "target_audience": {
            "age_range": {
                "min_age": 18,
                "max_age": 35
            },
            "locations": ["US", "CA", "UK"],
            "genres": ["Electronic", "House"],
            "fan_level": "casual",
            "platform_activity": "high"
        },
        "budget": 1000.0,
        "currency": "USD",
        "start_date": "2024-02-01T00:00:00Z",
        "end_date": "2024-02-29T23:59:59Z",
        "campaign_parameters": {
            "boost_multiplier": 2.0,
            "max_participants": 500,
            "reward_per_action": 0.1,
            "required_actions": ["listen", "share"],
            "nft_collection_size": 100,
            "minimum_listen_duration": 30
        },
        "metadata": {
            "promotional_message": "Join our exclusive campaign!",
            "hashtags": ["#VibeStream", "#NewMusic"]
        }
    });
    
    let response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist.id).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaign_id"].is_string());
    assert_eq!(json_response["data"]["name"], "Test Music Campaign");
    assert_eq!(json_response["data"]["campaign_type"], "nft_boost");
    assert_eq!(json_response["data"]["status"], "draft");
    assert_eq!(json_response["data"]["budget"], 1000.0);
    assert!(json_response["data"]["estimated_reach"].is_number());
    assert!(json_response["data"]["created_at"].is_string());
}

#[tokio::test]
async fn test_create_campaign_insufficient_budget() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_song = test_data.get_test_song();
    
    let campaign_data = json!({
        "name": "Underfunded Campaign",
        "description": "Campaign without enough budget",
        "campaign_type": "promotion",
        "song_id": test_song.id,
        "artist_id": artist.id,
        "target_audience": {
            "locations": ["US"],
            "genres": ["Electronic"]
        },
        "budget": 0.50, // Too low budget
        "currency": "USD",
        "start_date": "2024-02-01T00:00:00Z",
        "end_date": "2024-02-29T23:59:59Z",
        "campaign_parameters": {
            "max_participants": 1000
        }
    });
    
    let response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist.id).await;
    
    response.assert_status(StatusCode::PAYMENT_REQUIRED);
}

#[tokio::test]
async fn test_create_campaign_invalid_dates() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_song = test_data.get_test_song();
    
    let campaign_data = json!({
        "name": "Invalid Date Campaign",
        "description": "Campaign with end date before start date",
        "campaign_type": "promotion",
        "song_id": test_song.id,
        "artist_id": artist.id,
        "target_audience": {
            "locations": ["US"],
            "genres": ["Electronic"]
        },
        "budget": 100.0,
        "currency": "USD",
        "start_date": "2024-02-29T00:00:00Z",
        "end_date": "2024-02-01T23:59:59Z", // End before start
        "campaign_parameters": {}
    });
    
    let response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist.id).await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_campaign_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let test_campaign = &test_data.campaigns[0];
    
    let response = client.get(&format!("/api/v1/campaigns/{}", test_campaign.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["id"], test_campaign.id.to_string());
    assert_eq!(json_response["data"]["name"], test_campaign.name);
    assert_eq!(json_response["data"]["status"], test_campaign.status);
    assert_eq!(json_response["data"]["budget"], test_campaign.budget);
}

#[tokio::test]
async fn test_get_campaign_not_found() {
    let client = TestClient::new().await.unwrap();
    let non_existent_id = Uuid::new_v4();
    
    let response = client.get(&format!("/api/v1/campaigns/{}", non_existent_id)).await;
    
    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_campaign_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_campaign = &test_data.campaigns[0];
    
    let update_data = json!({
        "name": "Updated Campaign Name",
        "description": "Updated description with new information",
        "budget": 1500.0,
        "target_audience": {
            "locations": ["US", "CA", "UK", "AU"],
            "genres": ["Electronic", "House", "Techno"]
        }
    });
    
    let response = client.put(
        &format!("/api/v1/campaigns/{}", test_campaign.id),
        update_data
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["name"], "Updated Campaign Name");
    assert_eq!(json_response["data"]["budget"], 1500.0);
}

#[tokio::test]
async fn test_search_campaigns_basic() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/campaigns?limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaigns"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
    assert!(json_response["data"]["has_more"].is_boolean());
}

#[tokio::test]
async fn test_search_campaigns_with_filters() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/campaigns?campaign_type=nft_boost&status=active&min_budget=100&max_budget=2000").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaigns"].is_array());
}

// =============================================================================
// CAMPAIGN OPERATIONS TESTS
// =============================================================================

#[tokio::test]
async fn test_activate_campaign_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_song = test_data.get_test_song();
    
    // First create a campaign
    let campaign_data = json!({
        "name": "Campaign to Activate",
        "description": "A campaign ready for activation",
        "campaign_type": "promotion",
        "song_id": test_song.id,
        "artist_id": artist.id,
        "target_audience": {
            "locations": ["US"],
            "genres": ["Electronic"]
        },
        "budget": 500.0,
        "currency": "USD",
        "start_date": "2024-02-01T00:00:00Z",
        "end_date": "2024-02-29T23:59:59Z",
        "campaign_parameters": {
            "max_participants": 200
        }
    });
    
    let create_response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist.id).await;
    create_response.assert_success();
    
    let create_json: Value = create_response.json_value();
    let campaign_id = create_json["data"]["campaign_id"].as_str().unwrap();
    
    // Activate the campaign
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/activate", campaign_id),
        json!({}),
        artist.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["status"], "active");
}

#[tokio::test]
async fn test_participate_campaign_listen_action() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    let test_campaign = &test_data.campaigns[0];
    
    let participation_data = json!({
        "action_type": "listen",
        "action_data": {
            "duration_seconds": 120,
            "completion_percentage": 85.0
        },
        "proof_of_action": "zk_proof_hash_12345"
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", test_campaign.id),
        participation_data,
        fan.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["participation_id"].is_string());
    assert_eq!(json_response["data"]["campaign_id"], test_campaign.id.to_string());
    assert_eq!(json_response["data"]["user_id"], fan.id.to_string());
    assert_eq!(json_response["data"]["action_type"], "listen");
    assert!(json_response["data"]["reward_earned"].is_number());
    assert!(json_response["data"]["is_eligible_for_nft"].is_boolean());
    assert!(json_response["data"]["total_actions"].is_number());
}

#[tokio::test]
async fn test_participate_campaign_share_action() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    let test_campaign = &test_data.campaigns[0];
    
    let participation_data = json!({
        "action_type": "share",
        "action_data": {
            "platform": "twitter",
            "post_url": "https://twitter.com/user/status/123456",
            "reach_estimate": 500
        }
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", test_campaign.id),
        participation_data,
        fan.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["data"]["action_type"], "share");
}

#[tokio::test]
async fn test_participate_campaign_duplicate_action() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    let test_campaign = &test_data.campaigns[0];
    
    let participation_data = json!({
        "action_type": "follow",
        "action_data": {
            "artist_id": test_data.get_artist().id
        }
    });
    
    // First participation should succeed
    let response1 = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", test_campaign.id),
        participation_data.clone(),
        fan.id
    ).await;
    response1.assert_success();
    
    // Second identical participation might be rejected or have reduced rewards
    let response2 = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", test_campaign.id),
        participation_data,
        fan.id
    ).await;
    
    // Should either succeed with reduced rewards or be rejected
    assert!(
        response2.status.is_success() || response2.status == StatusCode::CONFLICT,
        "Duplicate participation should be handled gracefully"
    );
}

#[tokio::test]
async fn test_boost_campaign_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_campaign = &test_data.campaigns[0];
    
    let boost_data = json!({
        "boost_amount": 200.0,
        "boost_duration_hours": 24,
        "target_metrics": ["reach", "engagement"]
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/boost", test_campaign.id),
        boost_data,
        artist.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["boost_id"].is_string());
    assert_eq!(json_response["data"]["campaign_id"], test_campaign.id.to_string());
    assert_eq!(json_response["data"]["boost_amount"], 200.0);
    assert!(json_response["data"]["boost_multiplier"].is_number());
    assert!(json_response["data"]["estimated_additional_reach"].is_number());
    assert!(json_response["data"]["boost_start"].is_string());
    assert!(json_response["data"]["boost_end"].is_string());
}

#[tokio::test]
async fn test_boost_campaign_insufficient_funds() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_campaign = &test_data.campaigns[0];
    
    let boost_data = json!({
        "boost_amount": 50000.0, // Unrealistically high amount
        "boost_duration_hours": 168,
        "target_metrics": ["reach"]
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/boost", test_campaign.id),
        boost_data,
        artist.id
    ).await;
    
    response.assert_status(StatusCode::PAYMENT_REQUIRED);
}

// =============================================================================
// NFT OPERATIONS TESTS
// =============================================================================

#[tokio::test]
async fn test_mint_campaign_nft_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_campaign = &test_data.campaigns[0];
    
    let mint_data = json!({
        "nft_count": 10,
        "metadata_override": {
            "name": "Special Campaign NFT",
            "description": "Exclusive NFT for campaign participants",
            "attributes": [
                {
                    "trait_type": "Campaign",
                    "value": test_campaign.name
                },
                {
                    "trait_type": "Rarity",
                    "value": "Rare"
                }
            ]
        }
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/nft/mint", test_campaign.id),
        mint_data,
        artist.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["mint_batch_id"].is_string());
    assert_eq!(json_response["data"]["campaign_id"], test_campaign.id.to_string());
    assert_eq!(json_response["data"]["nft_count"], 10);
    assert!(json_response["data"]["recipients"].is_array());
    assert!(json_response["data"]["blockchain"].is_string());
    assert!(json_response["data"]["created_at"].is_string());
}

#[tokio::test]
async fn test_mint_campaign_nft_specific_recipient() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let fan = test_data.get_fan();
    let test_campaign = &test_data.campaigns[0];
    
    let mint_data = json!({
        "recipient_id": fan.id,
        "nft_count": 1,
        "metadata_override": {
            "name": "Personal Campaign NFT",
            "description": "Special NFT for top participant"
        }
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/nft/mint", test_campaign.id),
        mint_data,
        artist.id
    ).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["data"]["nft_count"], 1);
    let recipients = json_response["data"]["recipients"].as_array().unwrap();
    assert_eq!(recipients.len(), 1);
    assert_eq!(recipients[0]["user_id"], fan.id.to_string());
}

// =============================================================================
// CAMPAIGN ANALYTICS TESTS
// =============================================================================

#[tokio::test]
async fn test_get_campaign_analytics() {
    let client = TestClient::new().await.unwrap();
    let test_campaign = &client.test_data().campaigns[0];
    
    let response = client.get(&format!("/api/v1/campaigns/{}/analytics?time_range=7d", test_campaign.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["campaign_id"], test_campaign.id.to_string());
    assert!(json_response["data"]["performance_metrics"].is_object());
    assert!(json_response["data"]["audience_insights"].is_object());
    assert!(json_response["data"]["engagement_data"].is_object());
    assert!(json_response["data"]["conversion_funnel"].is_object());
    assert!(json_response["data"]["roi_analysis"].is_object());
    assert!(json_response["data"]["time_series_data"].is_array());
}

#[tokio::test]
async fn test_get_campaign_participants() {
    let client = TestClient::new().await.unwrap();
    let test_campaign = &client.test_data().campaigns[0];
    
    let response = client.get(&format!("/api/v1/campaigns/{}/participants?limit=20", test_campaign.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"].is_array());
}

#[tokio::test]
async fn test_get_campaign_leaderboard() {
    let client = TestClient::new().await.unwrap();
    let test_campaign = &client.test_data().campaigns[0];
    
    let response = client.get(&format!("/api/v1/campaigns/{}/leaderboard?limit=10", test_campaign.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"].is_array());
}

// =============================================================================
// DISCOVERY TESTS
// =============================================================================

#[tokio::test]
async fn test_get_trending_campaigns() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/campaigns/trending?limit=15").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaigns"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
}

#[tokio::test]
async fn test_get_trending_campaigns_by_type() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/campaigns/trending?campaign_type=nft_boost&limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaigns"].is_array());
}

#[tokio::test]
async fn test_get_featured_campaigns() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/campaigns/featured?limit=5").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaigns"].is_array());
}

#[tokio::test]
async fn test_get_user_campaigns() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let response = client.get(&format!("/api/v1/campaigns/user/{}?status=active", artist.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["campaigns"].is_array());
}

// =============================================================================
// NFT COLLECTIONS TESTS
// =============================================================================

#[tokio::test]
async fn test_list_nft_collections() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/campaigns/nft-collections?limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"].is_array());
    
    let collections = json_response["data"].as_array().unwrap();
    assert!(collections.len() >= 3, "Should have multiple NFT collections");
}

#[tokio::test]
async fn test_get_nft_collection() {
    let client = TestClient::new().await.unwrap();
    let collection_id = "vibestream_genesis";
    
    let response = client.get(&format!("/api/v1/campaigns/nft-collections/{}", collection_id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
}

// =============================================================================
// INTEGRATION FLOW TESTS
// =============================================================================

#[tokio::test]
async fn test_complete_campaign_lifecycle() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let fan = test_data.get_fan();
    let test_song = test_data.get_test_song();
    
    // 1. Artist creates a campaign
    let campaign_data = json!({
        "name": "Lifecycle Test Campaign",
        "description": "Complete campaign lifecycle test",
        "campaign_type": "nft_boost",
        "song_id": test_song.id,
        "artist_id": artist.id,
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
            "reward_per_action": 0.5,
            "required_actions": ["listen", "share"],
            "nft_collection_size": 50
        }
    });
    
    let create_response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist.id).await;
    create_response.assert_success();
    
    let create_json: Value = create_response.json_value();
    let campaign_id = create_json["data"]["campaign_id"].as_str().unwrap();
    
    // 2. Artist activates the campaign
    let activate_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/activate", campaign_id),
        json!({}),
        artist.id
    ).await;
    activate_response.assert_success();
    
    // 3. Fan participates with listen action
    let listen_participation = json!({
        "action_type": "listen",
        "action_data": {
            "duration_seconds": 180,
            "completion_percentage": 100.0
        }
    });
    
    let listen_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", campaign_id),
        listen_participation,
        fan.id
    ).await;
    listen_response.assert_success();
    
    // 4. Fan participates with share action
    let share_participation = json!({
        "action_type": "share",
        "action_data": {
            "platform": "twitter",
            "reach_estimate": 200
        }
    });
    
    let share_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", campaign_id),
        share_participation,
        fan.id
    ).await;
    share_response.assert_success();
    
    // 5. Artist boosts the campaign
    let boost_data = json!({
        "boost_amount": 100.0,
        "boost_duration_hours": 12,
        "target_metrics": ["reach"]
    });
    
    let boost_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/boost", campaign_id),
        boost_data,
        artist.id
    ).await;
    boost_response.assert_success();
    
    // 6. Check campaign analytics
    let analytics_response = client.get(&format!("/api/v1/campaigns/{}/analytics", campaign_id)).await;
    analytics_response.assert_success();
    
    let analytics_json: Value = analytics_response.json_value();
    assert!(analytics_json["data"]["performance_metrics"]["total_participants"].is_number());
    
    // 7. Check leaderboard
    let leaderboard_response = client.get(&format!("/api/v1/campaigns/{}/leaderboard", campaign_id)).await;
    leaderboard_response.assert_success();
    
    // 8. Mint NFTs for top participants
    let mint_data = json!({
        "nft_count": 5,
        "metadata_override": {
            "name": "Lifecycle Test NFT",
            "description": "NFT from complete lifecycle test"
        }
    });
    
    let mint_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/nft/mint", campaign_id),
        mint_data,
        artist.id
    ).await;
    mint_response.assert_success();
    
    let mint_json: Value = mint_response.json_value();
    assert_eq!(mint_json["data"]["nft_count"], 5);
}

#[tokio::test]
async fn test_multi_user_campaign_participation() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let fan1 = test_data.get_fan();
    let test_song = test_data.get_test_song();
    
    // Create additional test users
    let fan2_data = json!({
        "email": "campaignfan2@test.com",
        "username": "campaignfan2",
        "password": "password123",
        "display_name": "Campaign Fan 2"
    });
    
    let fan2_response = client.post("/api/v1/users", fan2_data).await;
    fan2_response.assert_success();
    
    let fan2_json: Value = fan2_response.json_value();
    let fan2_id = Uuid::parse_str(fan2_json["data"]["user_id"].as_str().unwrap()).unwrap();
    
    // Create a campaign
    let campaign_data = json!({
        "name": "Multi-User Test Campaign",
        "description": "Campaign for testing multiple user participation",
        "campaign_type": "contest",
        "song_id": test_song.id,
        "artist_id": artist.id,
        "target_audience": {
            "locations": ["US"],
            "genres": ["Electronic"]
        },
        "budget": 300.0,
        "currency": "USD",
        "start_date": "2024-02-01T00:00:00Z",
        "end_date": "2024-02-29T23:59:59Z",
        "campaign_parameters": {
            "max_participants": 50,
            "reward_per_action": 1.0,
            "required_actions": ["listen", "follow"]
        }
    });
    
    let create_response = client.post_with_auth("/api/v1/campaigns", campaign_data, artist.id).await;
    create_response.assert_success();
    
    let create_json: Value = create_response.json_value();
    let campaign_id = create_json["data"]["campaign_id"].as_str().unwrap();
    
    // Activate campaign
    let activate_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/activate", campaign_id),
        json!({}),
        artist.id
    ).await;
    activate_response.assert_success();
    
    // Both fans participate
    let participation_data = json!({
        "action_type": "listen",
        "action_data": {
            "duration_seconds": 150,
            "completion_percentage": 90.0
        }
    });
    
    let fan1_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", campaign_id),
        participation_data.clone(),
        fan1.id
    ).await;
    fan1_response.assert_success();
    
    let fan2_response = client.post_with_auth(
        &format!("/api/v1/campaigns/{}/participate", campaign_id),
        participation_data,
        fan2_id
    ).await;
    fan2_response.assert_success();
    
    // Check participants list
    let participants_response = client.get(&format!("/api/v1/campaigns/{}/participants", campaign_id)).await;
    participants_response.assert_success();
    
    let participants_json: Value = participants_response.json_value();
    let participants = participants_json["data"].as_array().unwrap();
    
    assert!(participants.len() >= 2, "Should have at least 2 participants");
    
    // Check leaderboard
    let leaderboard_response = client.get(&format!("/api/v1/campaigns/{}/leaderboard", campaign_id)).await;
    leaderboard_response.assert_success();
}

// =============================================================================
// PERFORMANCE TESTS
// =============================================================================

#[tokio::test]
async fn test_campaign_search_performance() {
    let client = TestClient::new().await.unwrap();
    
    let start = std::time::Instant::now();
    
    // Perform multiple campaign searches
    for i in 0..5 {
        let response = client.get(&format!("/api/v1/campaigns?limit=20&offset={}", i * 20)).await;
        response.assert_success();
    }
    
    let duration = start.elapsed();
    
    // Assert reasonable performance
    assert!(duration.as_millis() < 3000, "Campaign searches took too long: {:?}", duration);
}

#[tokio::test]
async fn test_concurrent_campaign_participation() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let test_campaign = &test_data.campaigns[0];
    
    let mut handles = vec![];
    
    // Simulate concurrent participation from multiple users
    for i in 0..3 {
        let client = client.clone();
        let campaign_id = test_campaign.id;
        let user_id = if i == 0 { test_data.get_fan().id } else { test_data.get_artist().id };
        
        let handle = tokio::spawn(async move {
            let participation_data = json!({
                "action_type": "listen",
                "action_data": {
                    "duration_seconds": 120 + (i * 10),
                    "completion_percentage": 80.0 + (i as f64 * 5.0)
                }
            });
            
            client.post_with_auth(
                &format!("/api/v1/campaigns/{}/participate", campaign_id),
                participation_data,
                user_id
            ).await
        });
        handles.push(handle);
    }
    
    // Wait for all participations to complete
    for handle in handles {
        let response = handle.await.unwrap();
        // Some might succeed, some might fail due to business rules
        assert!(
            response.status.is_success() || response.status == StatusCode::CONFLICT,
            "Concurrent participation should be handled gracefully"
        );
    }
} 