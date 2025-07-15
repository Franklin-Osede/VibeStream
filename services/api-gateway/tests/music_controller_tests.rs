use crate::helpers::{TestClient, TestData, assert_json_contains};
use serde_json::{json, Value};
use uuid::Uuid;
use axum::http::StatusCode;

mod helpers;

// =============================================================================
// SONG ENDPOINTS TESTS
// =============================================================================

#[tokio::test]
async fn test_upload_song_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let song_data = json!({
        "title": "New Test Song",
        "artist_id": artist.id,
        "duration_seconds": 210,
        "genre": "Electronic",
        "royalty_percentage": 80.0,
        "ipfs_hash": "QmTest123456789",
        "audio_quality": "lossless",
        "file_format": "flac",
        "mood": "energetic",
        "tempo": 128,
        "release_type": "single"
    });
    
    let response = client.post_with_auth("/api/v1/songs", song_data, artist.id).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["song_id"].is_string());
    assert_eq!(json_response["data"]["ipfs_hash"], "QmTest123456");
    assert_eq!(json_response["data"]["processing_status"], "uploaded");
}

#[tokio::test]
async fn test_upload_song_validation_error() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let invalid_song_data = json!({
        "title": "", // Empty title should fail validation
        "artist_id": artist.id,
        "duration_seconds": 0, // Zero duration should fail
        "genre": "Electronic",
        "royalty_percentage": 150.0, // > 100% should fail
        "ipfs_hash": "",
        "audio_quality": "lossless",
        "file_format": "flac",
        "release_type": "single"
    });
    
    let response = client.post_with_auth("/api/v1/songs", invalid_song_data, artist.id).await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_song_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let test_song = test_data.get_test_song();
    
    let response = client.get(&format!("/api/v1/songs/{}", test_song.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"]["id"], test_song.id.to_string());
    assert_eq!(json_response["data"]["title"], test_song.title);
    assert_eq!(json_response["data"]["genre"], test_song.genre);
}

#[tokio::test]
async fn test_get_song_not_found() {
    let client = TestClient::new().await.unwrap();
    let non_existent_id = Uuid::new_v4();
    
    let response = client.get(&format!("/api/v1/songs/{}", non_existent_id)).await;
    
    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_search_songs_basic() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/songs?limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["songs"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
    assert!(json_response["data"]["has_more"].is_boolean());
}

#[tokio::test]
async fn test_search_songs_with_filters() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/songs?genre=Electronic&limit=5&is_trending=true").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["songs"].is_array());
}

#[tokio::test]
async fn test_update_song_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_song = test_data.get_test_song();
    
    let update_data = json!({
        "title": "Updated Song Title",
        "genre": "Ambient",
        "mood": "relaxing",
        "tempo": 85
    });
    
    let response = client.put(
        &format!("/api/v1/songs/{}", test_song.id),
        update_data
    ).await;
    
    response.assert_success();
}

#[tokio::test]
async fn test_record_listen_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    let test_song = test_data.get_test_song();
    
    let listen_data = json!({
        "duration_seconds": 120,
        "completion_percentage": 80.0,
        "device_type": "mobile",
        "location": "US"
    });
    
    let response = client.post_with_auth(
        &format!("/api/v1/songs/{}/listen", test_song.id),
        listen_data,
        fan.id
    ).await;
    
    response.assert_success();
}

#[tokio::test]
async fn test_get_trending_songs() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/songs/trending?limit=20").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["songs"].is_array());
}

#[tokio::test]
async fn test_get_popular_songs() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/songs/popular?genre=Electronic&limit=15").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["songs"].is_array());
}

// =============================================================================
// ALBUM ENDPOINTS TESTS
// =============================================================================

#[tokio::test]
async fn test_create_album_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let test_song = test_data.get_test_song();
    
    let album_data = json!({
        "title": "Test Album",
        "artist_id": artist.id,
        "description": "A test album for integration testing",
        "genre": "Electronic",
        "album_type": "LP",
        "song_ids": [test_song.id]
    });
    
    let response = client.post_with_auth("/api/v1/albums", album_data, artist.id).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["album_id"].is_string());
    assert_eq!(json_response["data"]["track_count"], 1);
    assert!(json_response["data"]["total_duration_seconds"].is_number());
}

#[tokio::test]
async fn test_search_albums() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/albums?limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["albums"].is_array());
    assert!(json_response["data"]["total_count"].is_number());
}

// =============================================================================
// PLAYLIST ENDPOINTS TESTS  
// =============================================================================

#[tokio::test]
async fn test_create_playlist_success() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    
    let playlist_data = json!({
        "name": "My Test Playlist",
        "description": "A playlist for testing",
        "is_public": true,
        "is_collaborative": false,
        "tags": ["electronic", "chill"]
    });
    
    let response = client.post_with_auth("/api/v1/playlists", playlist_data, fan.id).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["playlist_id"].is_string());
    assert_eq!(json_response["data"]["name"], "My Test Playlist");
    assert_eq!(json_response["data"]["is_public"], true);
    assert_eq!(json_response["data"]["is_collaborative"], false);
}

#[tokio::test]
async fn test_create_collaborative_playlist() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let fan = test_data.get_fan();
    
    let playlist_data = json!({
        "name": "Collaborative Playlist",
        "description": "Anyone can add songs",
        "is_public": true,
        "is_collaborative": true,
        "tags": ["collaborative", "community"]
    });
    
    let response = client.post_with_auth("/api/v1/playlists", playlist_data, fan.id).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["data"]["is_collaborative"], true);
}

#[tokio::test]
async fn test_search_playlists() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/playlists?limit=10&tags=electronic").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["playlists"].is_array());
}

// =============================================================================
// ARTIST ENDPOINTS TESTS
// =============================================================================

#[tokio::test]
async fn test_get_artist_profile() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let response = client.get(&format!("/api/v1/artists/{}", artist.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    // Artist profile data would be returned here
}

#[tokio::test]
async fn test_get_artist_songs() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let response = client.get(&format!("/api/v1/artists/{}/songs", artist.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["songs"].is_array());
}

#[tokio::test]
async fn test_get_artist_albums() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    let response = client.get(&format!("/api/v1/artists/{}/albums", artist.id)).await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["albums"].is_array());
}

#[tokio::test]
async fn test_search_artists() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/artists?search_text=test&limit=10").await;
    
    response.assert_success();
    let json_response: Value = response.json_value();
    
    assert_eq!(json_response["success"], true);
    assert!(json_response["data"]["artists"].is_array());
}

// =============================================================================
// ERROR HANDLING TESTS
// =============================================================================

#[tokio::test]
async fn test_unauthorized_access() {
    let client = TestClient::new().await.unwrap();
    
    let song_data = json!({
        "title": "Unauthorized Song",
        "artist_id": Uuid::new_v4(),
        "duration_seconds": 180,
        "genre": "Test",
        "royalty_percentage": 80.0,
        "ipfs_hash": "QmTest",
        "audio_quality": "high",
        "file_format": "mp3",
        "release_type": "single"
    });
    
    // Try to upload without authentication
    let response = client.post("/api/v1/songs", song_data).await;
    
    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_invalid_uuid_format() {
    let client = TestClient::new().await.unwrap();
    
    let response = client.get("/api/v1/songs/invalid-uuid").await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_malformed_json() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    // Send malformed JSON
    let response = client.post_with_auth(
        "/api/v1/songs",
        json!("invalid json structure"),
        artist.id
    ).await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

// =============================================================================
// PERFORMANCE TESTS
// =============================================================================

#[tokio::test]
async fn test_bulk_song_search_performance() {
    let client = TestClient::new().await.unwrap();
    
    let start = std::time::Instant::now();
    
    // Perform multiple search requests
    for i in 0..10 {
        let response = client.get(&format!("/api/v1/songs?limit=20&offset={}", i * 20)).await;
        response.assert_success();
    }
    
    let duration = start.elapsed();
    
    // Assert that 10 searches complete within reasonable time (adjust as needed)
    assert!(duration.as_millis() < 5000, "Bulk searches took too long: {:?}", duration);
}

#[tokio::test]
async fn test_concurrent_requests() {
    let client = TestClient::new().await.unwrap();
    
    let mut handles = vec![];
    
    // Spawn 5 concurrent requests
    for _ in 0..5 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            client.get("/api/v1/songs/trending").await
        });
        handles.push(handle);
    }
    
    // Wait for all requests to complete
    for handle in handles {
        let response = handle.await.unwrap();
        response.assert_success();
    }
}

// =============================================================================
// INTEGRATION FLOW TESTS
// =============================================================================

#[tokio::test]
async fn test_complete_music_workflow() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    let fan = test_data.get_fan();
    
    // 1. Artist uploads a song
    let song_data = json!({
        "title": "Workflow Test Song",
        "artist_id": artist.id,
        "duration_seconds": 180,
        "genre": "Electronic",
        "royalty_percentage": 80.0,
        "ipfs_hash": "QmWorkflowTest",
        "audio_quality": "high",
        "file_format": "mp3",
        "release_type": "single"
    });
    
    let upload_response = client.post_with_auth("/api/v1/songs", song_data, artist.id).await;
    upload_response.assert_success();
    
    let upload_json: Value = upload_response.json_value();
    let song_id = upload_json["data"]["song_id"].as_str().unwrap();
    
    // 2. Fan discovers the song
    let search_response = client.get("/api/v1/songs?search_text=Workflow").await;
    search_response.assert_success();
    
    // 3. Fan listens to the song
    let listen_data = json!({
        "duration_seconds": 150,
        "completion_percentage": 83.3,
        "device_type": "mobile"
    });
    
    let listen_response = client.post_with_auth(
        &format!("/api/v1/songs/{}/listen", song_id),
        listen_data,
        fan.id
    ).await;
    listen_response.assert_success();
    
    // 4. Fan creates a playlist and adds the song
    let playlist_data = json!({
        "name": "Workflow Test Playlist",
        "is_public": true,
        "is_collaborative": false
    });
    
    let playlist_response = client.post_with_auth("/api/v1/playlists", playlist_data, fan.id).await;
    playlist_response.assert_success();
    
    let playlist_json: Value = playlist_response.json_value();
    let playlist_id = playlist_json["data"]["playlist_id"].as_str().unwrap();
    
    // 5. Add song to playlist
    let add_song_data = json!({
        "song_id": song_id
    });
    
    let add_response = client.post_with_auth(
        &format!("/api/v1/playlists/{}/songs", playlist_id),
        add_song_data,
        fan.id
    ).await;
    add_response.assert_success();
}

#[tokio::test]
async fn test_artist_content_management_flow() {
    let client = TestClient::new().await.unwrap();
    let test_data = client.test_data();
    let artist = test_data.get_artist();
    
    // 1. Upload multiple songs
    let mut song_ids = vec![];
    
    for i in 1..=3 {
        let song_data = json!({
            "title": format!("Album Song {}", i),
            "artist_id": artist.id,
            "duration_seconds": 180 + (i * 20),
            "genre": "Electronic",
            "royalty_percentage": 80.0,
            "ipfs_hash": format!("QmAlbumSong{}", i),
            "audio_quality": "high",
            "file_format": "mp3",
            "release_type": "album"
        });
        
        let response = client.post_with_auth("/api/v1/songs", song_data, artist.id).await;
        response.assert_success();
        
        let json: Value = response.json_value();
        song_ids.push(json["data"]["song_id"].as_str().unwrap().to_string());
    }
    
    // 2. Create an album with those songs
    let album_data = json!({
        "title": "Test Album Collection",
        "artist_id": artist.id,
        "description": "An album created in integration test",
        "genre": "Electronic",
        "album_type": "LP",
        "song_ids": song_ids
    });
    
    let album_response = client.post_with_auth("/api/v1/albums", album_data, artist.id).await;
    album_response.assert_success();
    
    let album_json: Value = album_response.json_value();
    assert_eq!(album_json["data"]["track_count"], 3);
    
    // 3. Verify artist's songs are listed
    let artist_songs_response = client.get(&format!("/api/v1/artists/{}/songs", artist.id)).await;
    artist_songs_response.assert_success();
    
    // 4. Verify artist's albums are listed
    let artist_albums_response = client.get(&format!("/api/v1/artists/{}/albums", artist.id)).await;
    artist_albums_response.assert_success();
} 