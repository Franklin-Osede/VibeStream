use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::value_objects::Id;
use super::value_objects::*;

/// Federated Instance Entity - Represents a federated instance (Mastodon, PeerTube, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedInstance {
    pub id: Id,
    pub domain: String,
    pub instance_url: String,
    pub software_name: String,
    pub software_version: String,
    pub federation_protocol: FederationProtocol,
    pub supported_features: Vec<FederationFeature>,
    pub trust_level: TrustLevel,
    pub content_policies: ContentPolicies,
    pub federation_stats: FederationStats,
    pub last_sync: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FederatedInstance {
    pub fn new(
        domain: String,
        instance_url: String,
        software_name: String,
        software_version: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            domain,
            instance_url,
            software_name,
            software_version,
            federation_protocol: FederationProtocol::ActivityPub,
            supported_features: vec![],
            trust_level: TrustLevel::Unknown,
            content_policies: ContentPolicies::default(),
            federation_stats: FederationStats::default(),
            last_sync: now,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_federate_content(&self) -> bool {
        self.trust_level != TrustLevel::Blocked &&
        self.supported_features.contains(&FederationFeature::ContentSharing)
    }

    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
        self.updated_at = Utc::now();
    }

    pub fn add_supported_feature(&mut self, feature: FederationFeature) {
        if !self.supported_features.contains(&feature) {
            self.supported_features.push(feature);
            self.updated_at = Utc::now();
        }
    }

    pub fn increment_federation_count(&mut self) {
        self.federation_stats.total_activities += 1;
        self.federation_stats.last_activity = Utc::now();
    }
}

/// ActivityPub Activity Entity - Represents an ActivityPub activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPubActivity {
    pub id: Id,
    pub activity_id: String, // Full ActivityPub ID
    pub activity_type: ActivityType,
    pub actor: String, // Actor URI
    pub object: ActivityObject,
    pub target: Option<String>, // Target URI
    pub audience: Vec<String>, // Audience URIs
    pub published: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub processed: bool,
    pub federation_distance: u32,
    pub source_instance: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ActivityPubActivity {
    pub fn new(
        activity_id: String,
        activity_type: ActivityType,
        actor: String,
        object: ActivityObject,
        source_instance: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            activity_id,
            activity_type,
            actor,
            object,
            target: None,
            audience: vec![],
            published: now,
            received_at: now,
            processed: false,
            federation_distance: 0,
            source_instance,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_processed(&mut self) {
        self.processed = true;
        self.updated_at = Utc::now();
    }

    pub fn is_follow_activity(&self) -> bool {
        matches!(self.activity_type, ActivityType::Follow)
    }

    pub fn is_content_activity(&self) -> bool {
        matches!(self.activity_type, ActivityType::Create | ActivityType::Announce)
    }
}

/// Federated User Entity - Represents a user from a federated instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedUser {
    pub id: Id,
    pub username: String,
    pub domain: String,
    pub full_uri: String, // Full ActivityPub URI
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub header_url: Option<String>,
    pub public_key: String,
    pub inbox_url: String,
    pub outbox_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub last_status_at: Option<DateTime<Utc>>,
    pub trust_level: TrustLevel,
    pub is_local: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FederatedUser {
    pub fn new(
        username: String,
        domain: String,
        display_name: String,
        public_key: String,
        inbox_url: String,
        outbox_url: String,
        is_local: bool,
    ) -> Self {
        let now = Utc::now();
        let full_uri = format!("https://{}/users/{}", domain, username);
        let followers_url = format!("{}/followers", full_uri);
        let following_url = format!("{}/following", full_uri);
        
        Self {
            id: Id::new(),
            username,
            domain,
            full_uri,
            display_name,
            bio: None,
            avatar_url: None,
            header_url: None,
            public_key,
            inbox_url,
            outbox_url,
            followers_url,
            following_url,
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            last_status_at: None,
            trust_level: TrustLevel::Unknown,
            is_local,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_profile(&mut self, display_name: String, bio: Option<String>) {
        self.display_name = display_name;
        self.bio = bio;
        self.updated_at = Utc::now();
    }

    pub fn increment_followers(&mut self) {
        self.followers_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_followers(&mut self) {
        if self.followers_count > 0 {
            self.followers_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    pub fn add_status(&mut self) {
        self.statuses_count += 1;
        self.last_status_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

/// Federated Content Entity - Represents content shared via federation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedContent {
    pub id: Id,
    pub content_id: String, // Full ActivityPub ID
    pub content_type: ContentType,
    pub title: String,
    pub content: String,
    pub media_attachments: Vec<MediaAttachment>,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub sensitive: bool,
    pub visibility: Visibility,
    pub author: String, // Author URI
    pub source_instance: String,
    pub federation_distance: u32,
    pub local_reactions: Vec<Reaction>,
    pub local_comments: Vec<Comment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FederatedContent {
    pub fn new(
        content_id: String,
        content_type: ContentType,
        title: String,
        content: String,
        author: String,
        source_instance: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            content_id,
            content_type,
            title,
            content,
            media_attachments: vec![],
            tags: vec![],
            language: None,
            sensitive: false,
            visibility: Visibility::Public,
            author,
            source_instance,
            federation_distance: 0,
            local_reactions: vec![],
            local_comments: vec![],
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_media_attachment(&mut self, attachment: MediaAttachment) {
        self.media_attachments.push(attachment);
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn add_reaction(&mut self, reaction: Reaction) {
        self.local_reactions.push(reaction);
        self.updated_at = Utc::now();
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.local_comments.push(comment);
        self.updated_at = Utc::now();
    }

    pub fn is_music_content(&self) -> bool {
        matches!(self.content_type, ContentType::Music | ContentType::Audio)
    }

    pub fn is_video_content(&self) -> bool {
        matches!(self.content_type, ContentType::Video | ContentType::LiveStream)
    }
}

/// Federation Follow Relationship Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationFollow {
    pub id: Id,
    pub follower: String, // Follower URI
    pub followee: String, // Followee URI
    pub follower_domain: String,
    pub followee_domain: String,
    pub status: FollowStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub federation_distance: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FederationFollow {
    pub fn new(
        follower: String,
        followee: String,
        follower_domain: String,
        followee_domain: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            follower,
            followee,
            follower_domain,
            followee_domain,
            status: FollowStatus::Pending,
            approved_at: None,
            rejected_at: None,
            federation_distance: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn approve(&mut self) {
        self.status = FollowStatus::Approved;
        self.approved_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn reject(&mut self) {
        self.status = FollowStatus::Rejected;
        self.rejected_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn is_approved(&self) -> bool {
        matches!(self.status, FollowStatus::Approved)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.status, FollowStatus::Pending)
    }
} 