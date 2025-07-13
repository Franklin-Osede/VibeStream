use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::value_objects::Id;
use super::value_objects::*;

/// User Profile Entity - Represents a user's music preferences and behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Id,
    pub user_id: String,
    pub music_preferences: MusicPreferences,
    pub listening_history: Vec<ListeningEvent>,
    pub social_connections: Vec<SocialConnection>,
    pub recommendation_preferences: RecommendationPreferences,
    pub p2p_network: P2PNetworkProfile,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            user_id,
            music_preferences: MusicPreferences::default(),
            listening_history: Vec::new(),
            social_connections: Vec::new(),
            recommendation_preferences: RecommendationPreferences::default(),
            p2p_network: P2PNetworkProfile::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_listening_event(&mut self, event: ListeningEvent) {
        self.listening_history.push(event);
        self.updated_at = Utc::now();
    }

    pub fn update_preferences(&mut self, preferences: MusicPreferences) {
        self.music_preferences = preferences;
        self.updated_at = Utc::now();
    }

    pub fn add_social_connection(&mut self, connection: SocialConnection) {
        if !self.social_connections.iter().any(|c| c.connected_user_id == connection.connected_user_id) {
            self.social_connections.push(connection);
            self.updated_at = Utc::now();
        }
    }

    pub fn get_recent_listening_history(&self, days: u32) -> Vec<&ListeningEvent> {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        self.listening_history
            .iter()
            .filter(|event| event.timestamp > cutoff)
            .collect()
    }

    pub fn get_top_genres(&self, limit: usize) -> Vec<String> {
        let mut genre_counts = std::collections::HashMap::new();
        
        for event in &self.listening_history {
            for genre in &event.genres {
                *genre_counts.entry(genre.clone()).or_insert(0) += 1;
            }
        }
        
        let mut genres: Vec<(String, u32)> = genre_counts.into_iter().collect();
        genres.sort_by(|a, b| b.1.cmp(&a.1));
        
        genres.into_iter().take(limit).map(|(genre, _)| genre).collect()
    }
}

/// Listening Event Entity - Represents a user's listening activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeningEvent {
    pub id: Id,
    pub user_id: String,
    pub content_id: String,
    pub content_type: ContentType,
    pub duration_seconds: u32,
    pub completion_rate: f64, // 0.0 to 1.0
    pub genres: Vec<String>,
    pub artists: Vec<String>,
    pub tags: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub device_info: DeviceInfo,
    pub context: ListeningContext,
}

impl ListeningEvent {
    pub fn new(
        user_id: String,
        content_id: String,
        content_type: ContentType,
        duration_seconds: u32,
        completion_rate: f64,
        genres: Vec<String>,
        artists: Vec<String>,
    ) -> Self {
        Self {
            id: Id::new(),
            user_id,
            content_id,
            content_type,
            duration_seconds,
            completion_rate,
            genres,
            artists,
            tags: Vec::new(),
            timestamp: Utc::now(),
            device_info: DeviceInfo::default(),
            context: ListeningContext::default(),
        }
    }

    pub fn is_complete_listen(&self) -> bool {
        self.completion_rate >= 0.8
    }

    pub fn get_engagement_score(&self) -> f64 {
        self.completion_rate * (self.duration_seconds as f64 / 60.0)
    }
}

/// Recommendation Entity - Represents a recommendation for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Id,
    pub user_id: String,
    pub content_id: String,
    pub content_type: ContentType,
    pub score: f64,
    pub algorithm: RecommendationAlgorithm,
    pub reasoning: Vec<RecommendationReason>,
    pub source: RecommendationSource,
    pub metadata: RecommendationMetadata,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl Recommendation {
    pub fn new(
        user_id: String,
        content_id: String,
        content_type: ContentType,
        score: f64,
        algorithm: RecommendationAlgorithm,
        source: RecommendationSource,
    ) -> Self {
        Self {
            id: Id::new(),
            user_id,
            content_id,
            content_type,
            score,
            algorithm,
            reasoning: Vec::new(),
            source,
            metadata: RecommendationMetadata::default(),
            created_at: Utc::now(),
            expires_at: None,
        }
    }

    pub fn add_reason(&mut self, reason: RecommendationReason) {
        self.reasoning.push(reason);
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    pub fn is_high_confidence(&self) -> bool {
        self.score >= 0.8
    }
}

/// Recommendation Model Entity - Represents a machine learning model for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationModel {
    pub id: Id,
    pub model_type: ModelType,
    pub version: String,
    pub parameters: ModelParameters,
    pub performance_metrics: ModelPerformance,
    pub training_data: TrainingDataInfo,
    pub status: ModelStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RecommendationModel {
    pub fn new(
        model_type: ModelType,
        version: String,
        parameters: ModelParameters,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            model_type,
            version,
            parameters,
            performance_metrics: ModelPerformance::default(),
            training_data: TrainingDataInfo::default(),
            status: ModelStatus::Training,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_trained(&mut self, performance: ModelPerformance) {
        self.performance_metrics = performance;
        self.status = ModelStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, ModelStatus::Active)
    }

    pub fn get_accuracy(&self) -> f64 {
        self.performance_metrics.accuracy
    }
}

/// P2P Recommendation Network Entity - Represents the P2P network for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PRecommendationNetwork {
    pub id: Id,
    pub network_id: String,
    pub nodes: Vec<RecommendationNode>,
    pub connections: Vec<RecommendationConnection>,
    pub shared_knowledge: Vec<SharedKnowledge>,
    pub network_metrics: NetworkMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl P2PRecommendationNetwork {
    pub fn new(network_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            network_id,
            nodes: Vec::new(),
            connections: Vec::new(),
            shared_knowledge: Vec::new(),
            network_metrics: NetworkMetrics::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_node(&mut self, node: RecommendationNode) {
        if !self.nodes.iter().any(|n| n.node_id == node.node_id) {
            self.nodes.push(node);
            self.updated_at = Utc::now();
        }
    }

    pub fn add_connection(&mut self, connection: RecommendationConnection) {
        if !self.connections.iter().any(|c| 
            (c.from_node == connection.from_node && c.to_node == connection.to_node) ||
            (c.from_node == connection.to_node && c.to_node == connection.from_node)
        ) {
            self.connections.push(connection);
            self.updated_at = Utc::now();
        }
    }

    pub fn share_knowledge(&mut self, knowledge: SharedKnowledge) {
        self.shared_knowledge.push(knowledge);
        self.updated_at = Utc::now();
    }

    pub fn get_connected_nodes(&self, node_id: &str) -> Vec<&RecommendationNode> {
        let connected_ids: Vec<String> = self.connections
            .iter()
            .filter(|c| c.from_node == node_id || c.to_node == node_id)
            .map(|c| if c.from_node == node_id { c.to_node.clone() } else { c.from_node.clone() })
            .collect();
        
        self.nodes.iter().filter(|n| connected_ids.contains(&n.node_id)).collect()
    }
}

/// Collaborative Filtering Group Entity - Represents a group of similar users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeFilteringGroup {
    pub id: Id,
    pub group_id: String,
    pub members: Vec<String>, // User IDs
    pub centroid: UserCentroid,
    pub similarity_matrix: Vec<SimilarityScore>,
    pub group_preferences: GroupPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CollaborativeFilteringGroup {
    pub fn new(group_id: String, members: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            group_id,
            members,
            centroid: UserCentroid::default(),
            similarity_matrix: Vec::new(),
            group_preferences: GroupPreferences::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_member(&mut self, user_id: String) {
        if !self.members.contains(&user_id) {
            self.members.push(user_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_member(&mut self, user_id: &str) {
        self.members.retain(|id| id != user_id);
        self.updated_at = Utc::now();
    }

    pub fn calculate_centroid(&mut self) {
        // This would calculate the centroid based on member preferences
        // Implementation would be in the application service
        self.updated_at = Utc::now();
    }

    pub fn get_group_size(&self) -> usize {
        self.members.len()
    }
} 