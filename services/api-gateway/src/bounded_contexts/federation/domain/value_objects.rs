use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ActivityPub Activity Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityType {
    Create,
    Update,
    Delete,
    Follow,
    Accept,
    Reject,
    Add,
    Remove,
    Like,
    Announce,
    Block,
    Undo,
    Move,
}

/// ActivityPub Object Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityObject {
    Note {
        content: String,
        attachments: Vec<MediaAttachment>,
        tags: Vec<String>,
        language: Option<String>,
        sensitive: bool,
    },
    Person {
        username: String,
        display_name: String,
        bio: Option<String>,
        avatar_url: Option<String>,
        public_key: String,
    },
    Music {
        title: String,
        artist: String,
        album: Option<String>,
        duration: u32,
        audio_url: String,
        cover_url: Option<String>,
        tags: Vec<String>,
    },
    Video {
        title: String,
        description: String,
        video_url: String,
        thumbnail_url: Option<String>,
        duration: u32,
        tags: Vec<String>,
    },
    LiveStream {
        title: String,
        description: String,
        stream_url: String,
        thumbnail_url: Option<String>,
        is_live: bool,
        viewer_count: u32,
    },
    Custom {
        object_type: String,
        data: serde_json::Value,
    },
}

/// Content Types for Federation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    Music,
    Audio,
    Video,
    LiveStream,
    Image,
    Text,
    Link,
    Poll,
    Event,
}

/// Federation Protocol Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FederationProtocol {
    ActivityPub,
    Matrix,
    Diaspora,
    Custom(String),
}

/// Federation Feature Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FederationFeature {
    ContentSharing,
    UserFollowing,
    Comments,
    Reactions,
    LiveStreaming,
    Analytics,
    Moderation,
    Search,
}

/// Trust Level for Federation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustLevel {
    Trusted,
    Verified,
    Unknown,
    Suspicious,
    Blocked,
}

/// Follow Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FollowStatus {
    Pending,
    Approved,
    Rejected,
    Blocked,
}

/// Content Visibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

/// Media Attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAttachment {
    pub id: String,
    pub media_type: MediaType,
    pub url: String,
    pub preview_url: Option<String>,
    pub description: Option<String>,
    pub file_size: Option<u64>,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Media Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
    Unknown,
}

/// Reaction to Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub id: String,
    pub emoji: String,
    pub count: u32,
    pub users: Vec<String>, // User URIs
    pub created_at: DateTime<Utc>,
}

/// Comment on Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub content: String,
    pub author: String, // Author URI
    pub parent_id: Option<String>,
    pub replies_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Content Policies for Federation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentPolicies {
    pub allow_nsfw: bool,
    pub allow_political: bool,
    pub allow_commercial: bool,
    pub require_moderation: bool,
    pub auto_approve: bool,
    pub blocked_keywords: Vec<String>,
    pub allowed_domains: Vec<String>,
    pub blocked_domains: Vec<String>,
}

/// Federation Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FederationStats {
    pub total_activities: u64,
    pub total_users: u32,
    pub total_content: u32,
    pub total_follows: u32,
    pub last_activity: DateTime<Utc>,
    pub uptime_percentage: f64,
    pub response_time_ms: u32,
    pub error_rate: f64,
}

/// WebFinger Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFingerResource {
    pub subject: String,
    pub links: Vec<WebFingerLink>,
    pub aliases: Vec<String>,
    pub properties: std::collections::HashMap<String, serde_json::Value>,
}

/// WebFinger Link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFingerLink {
    pub rel: String,
    pub href: Option<String>,
    pub template: Option<String>,
    pub title: Option<String>,
    pub media_type: Option<String>,
    pub properties: std::collections::HashMap<String, serde_json::Value>,
}

/// NodeInfo Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub version: String,
    pub software: NodeInfoSoftware,
    pub protocols: Vec<String>,
    pub services: NodeInfoServices,
    pub usage: NodeInfoUsage,
    pub open_registrations: bool,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// NodeInfo Software
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoSoftware {
    pub name: String,
    pub version: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
}

/// NodeInfo Services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoServices {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

/// NodeInfo Usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoUsage {
    pub users: NodeInfoUsers,
    pub local_posts: u32,
    pub local_comments: u32,
}

/// NodeInfo Users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoUsers {
    pub total: u32,
    pub active_month: u32,
    pub active_half_year: u32,
}

/// Federation Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationRequest {
    pub method: String,
    pub url: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_seconds: u64,
}

/// Federation Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationResponse {
    pub status_code: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
    pub response_time_ms: u32,
}

/// Federation Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationError {
    pub error_type: FederationErrorType,
    pub message: String,
    pub details: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Federation Error Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FederationErrorType {
    NetworkError,
    AuthenticationError,
    AuthorizationError,
    RateLimitError,
    ContentError,
    ProtocolError,
    ValidationError,
    Unknown,
}

/// Federation Metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FederationMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub total_data_transferred_mb: f64,
    pub active_connections: u32,
    pub last_request_at: Option<DateTime<Utc>>,
}

impl FederationMetrics {
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.successful_requests as f64 / self.total_requests as f64
    }

    pub fn increment_request(&mut self, success: bool, response_time_ms: u32) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        
        // Update average response time
        let total_time = self.average_response_time_ms * (self.total_requests - 1) as f64 + response_time_ms as f64;
        self.average_response_time_ms = total_time / self.total_requests as f64;
        
        self.last_request_at = Some(Utc::now());
    }
} 