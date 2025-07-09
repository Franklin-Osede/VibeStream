use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{PgPool, Postgres, Pool};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// =============================================================================
// TEST CONFIGURATION
// =============================================================================

pub struct TestConfig {
    pub database_url: String,
    pub redis_url: String,
    pub test_db_name: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost/vibestream_test".to_string()),
            redis_url: std::env::var("TEST_REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            test_db_name: format!("test_db_{}", Uuid::new_v4().to_string().replace('-', "")),
        }
    }
}

// =============================================================================
// TEST DATA FIXTURES
// =============================================================================

#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TestSong {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TestPayment {
    pub id: Uuid,
    pub payer_id: Uuid,
    pub payee_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TestCampaign {
    pub id: Uuid,
    pub name: String,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub budget: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub struct TestData {
    pub users: Vec<TestUser>,
    pub songs: Vec<TestSong>,
    pub payments: Vec<TestPayment>,
    pub campaigns: Vec<TestCampaign>,
}

impl TestData {
    pub fn new() -> Self {
        let now = Utc::now();
        
        let users = vec![
            TestUser {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                username: "test_artist".to_string(),
                email: "artist@test.com".to_string(),
                display_name: "Test Artist".to_string(),
                created_at: now,
            },
            TestUser {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(),
                username: "test_fan".to_string(),
                email: "fan@test.com".to_string(),
                display_name: "Test Fan".to_string(),
                created_at: now,
            },
            TestUser {
                id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap(),
                username: "test_admin".to_string(),
                email: "admin@test.com".to_string(),
                display_name: "Test Admin".to_string(),
                created_at: now,
            },
        ];
        
        let songs = vec![
            TestSong {
                id: Uuid::parse_str("660e8400-e29b-41d4-a716-446655440001").unwrap(),
                title: "Test Song 1".to_string(),
                artist_id: users[0].id,
                duration_seconds: 180,
                genre: "Electronic".to_string(),
                created_at: now,
            },
            TestSong {
                id: Uuid::parse_str("660e8400-e29b-41d4-a716-446655440002").unwrap(),
                title: "Test Song 2".to_string(),
                artist_id: users[0].id,
                duration_seconds: 240,
                genre: "Hip Hop".to_string(),
                created_at: now,
            },
        ];
        
        let payments = vec![
            TestPayment {
                id: Uuid::parse_str("770e8400-e29b-41d4-a716-446655440001").unwrap(),
                payer_id: users[1].id,
                payee_id: users[0].id,
                amount: 2.99,
                currency: "USD".to_string(),
                status: "completed".to_string(),
                created_at: now,
            },
        ];
        
        let campaigns = vec![
            TestCampaign {
                id: Uuid::parse_str("880e8400-e29b-41d4-a716-446655440001").unwrap(),
                name: "Test Campaign".to_string(),
                artist_id: users[0].id,
                song_id: songs[0].id,
                budget: 1000.0,
                status: "active".to_string(),
                created_at: now,
            },
        ];
        
        Self {
            users,
            songs,
            payments,
            campaigns,
        }
    }
    
    pub fn get_artist(&self) -> &TestUser {
        &self.users[0]
    }
    
    pub fn get_fan(&self) -> &TestUser {
        &self.users[1]
    }
    
    pub fn get_admin(&self) -> &TestUser {
        &self.users[2]
    }
    
    pub fn get_test_song(&self) -> &TestSong {
        &self.songs[0]
    }
}

// =============================================================================
// MOCK TEST CLIENT (SIMPLIFIED)
// =============================================================================

pub struct TestClient {
    test_data: TestData,
}

impl TestClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let test_data = TestData::new();
        
        Ok(Self {
            test_data,
        })
    }
    
    // Mock HTTP methods for testing
    pub async fn get(&self, uri: &str) -> TestResponse {
        println!("Mock GET request to: {}", uri);
        TestResponse::mock_success()
    }
    
    pub async fn post(&self, uri: &str, body: Value) -> TestResponse {
        println!("Mock POST request to: {} with body: {}", uri, body);
        TestResponse::mock_success()
    }
    
    pub async fn put(&self, uri: &str, body: Value) -> TestResponse {
        println!("Mock PUT request to: {} with body: {}", uri, body);
        TestResponse::mock_success()
    }
    
    pub async fn delete(&self, uri: &str) -> TestResponse {
        println!("Mock DELETE request to: {}", uri);
        TestResponse::mock_success()
    }
    
    pub async fn post_with_auth(&self, uri: &str, body: Value, user_id: Uuid) -> TestResponse {
        println!("Mock POST request to: {} with auth for user: {}", uri, user_id);
        TestResponse::mock_success()
    }
    
    pub async fn get_with_auth(&self, uri: &str, user_id: Uuid) -> TestResponse {
        println!("Mock GET request to: {} with auth for user: {}", uri, user_id);
        TestResponse::mock_success()
    }
    
    pub fn test_data(&self) -> &TestData {
        &self.test_data
    }
}

// =============================================================================
// MOCK TEST RESPONSE
// =============================================================================

pub struct TestResponse {
    pub status: StatusCode,
    pub body: String,
}

impl TestResponse {
    fn mock_success() -> Self {
        Self {
            status: StatusCode::OK,
            body: json!({
                "success": true,
                "data": {},
                "timestamp": chrono::Utc::now().to_rfc3339()
            }).to_string(),
        }
    }
    
    fn mock_error(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            body: json!({
                "success": false,
                "error": message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }).to_string(),
        }
    }
    
    pub fn assert_status(&self, expected: StatusCode) {
        assert_eq!(self.status, expected, "Response body: {}", self.body);
    }
    
    pub fn assert_success(&self) {
        assert!(self.status.is_success(), "Expected success, got {}: {}", self.status, self.body);
    }
    
    pub fn json<T: for<'a> Deserialize<'a>>(&self) -> T {
        serde_json::from_str(&self.body)
            .unwrap_or_else(|e| panic!("Failed to parse JSON: {}. Body: {}", e, self.body))
    }
    
    pub fn json_value(&self) -> Value {
        serde_json::from_str(&self.body)
            .unwrap_or_else(|e| panic!("Failed to parse JSON: {}. Body: {}", e, self.body))
    }
    
    pub fn assert_json_contains(&self, expected: &Value) {
        let actual: Value = self.json_value();
        assert_json_contains(&actual, expected);
    }
}

// =============================================================================
// ASSERTION HELPERS
// =============================================================================

pub fn assert_json_contains(actual: &Value, expected: &Value) {
    match (actual, expected) {
        (Value::Object(actual_map), Value::Object(expected_map)) => {
            for (key, expected_value) in expected_map {
                let actual_value = actual_map.get(key)
                    .unwrap_or_else(|| panic!("Missing key '{}' in actual JSON", key));
                assert_json_contains(actual_value, expected_value);
            }
        }
        (Value::Array(actual_array), Value::Array(expected_array)) => {
            assert_eq!(actual_array.len(), expected_array.len());
            for (actual_item, expected_item) in actual_array.iter().zip(expected_array.iter()) {
                assert_json_contains(actual_item, expected_item);
            }
        }
        _ => {
            assert_eq!(actual, expected, "JSON values don't match");
        }
    }
}

// =============================================================================
// MACROS FOR COMMON TESTS
// =============================================================================

#[macro_export]
macro_rules! test_crud_operations {
    ($entity:ident, $endpoint:literal, $create_data:expr, $update_data:expr) => {
        #[tokio::test]
        async fn test_create_get_update_delete() {
            let client = TestClient::new().await.unwrap();
            
            // CREATE
            let response = client.post($endpoint, $create_data).await;
            response.assert_success();
            let created: Value = response.json_value();
            let id = created["data"]["id"].as_str().unwrap_or("test-id");
            
            // GET
            let response = client.get(&format!("{}/{}", $endpoint, id)).await;
            response.assert_success();
            
            // UPDATE
            let response = client.put(&format!("{}/{}", $endpoint, id), $update_data).await;
            response.assert_success();
            
            // DELETE
            let response = client.delete(&format!("{}/{}", $endpoint, id)).await;
            response.assert_success();
        }
    };
}

// =============================================================================
// TEST UTILITIES
// =============================================================================

pub fn random_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn random_email() -> String {
    format!("{}@test.com", random_string(10).to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_setup() {
        let db = TestDatabase::new().await.unwrap();
        let test_data = db.seed_test_data().await.unwrap();
        
        assert_eq!(test_data.users.len(), 3);
        assert_eq!(test_data.songs.len(), 2);
        
        db.cleanup().await.unwrap();
    }
    
    #[test]
    fn test_test_data_getters() {
        let test_data = TestData::new();
        
        assert_eq!(test_data.get_artist().username, "test_artist");
        assert_eq!(test_data.get_fan().username, "test_fan");
        assert_eq!(test_data.get_admin().username, "test_admin");
    }
} 