
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Content Types for Recommendations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    Music,
    Audio,
    Video,
    LiveStream,
    Podcast,
    Playlist,
    Album,
    Artist,
}

/// Recommendation Algorithm Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationAlgorithm {
    CollaborativeFiltering,
    ContentBasedFiltering,
    Hybrid,
    MatrixFactorization,
    NeuralNetwork,
    P2PCollaborative,
    FederatedLearning,
    Custom(String),
}

/// Recommendation Source Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationSource {
    Local,
    Federated,
    P2PNetwork,
    PeerRecommendation,
    Trending,
    Editorial,
    Algorithmic,
}

/// Model Types for Machine Learning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelType {
    CollaborativeFiltering,
    ContentBased,
    Hybrid,
    DeepLearning,
    MatrixFactorization,
    NeuralCollaborativeFiltering,
    FederatedLearning,
}

/// Model Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModelStatus {
    Training,
    Active,
    Inactive,
    Retraining,
    Error,
}

/// Music Preferences
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MusicPreferences {
    pub favorite_genres: Vec<String>,
    pub favorite_artists: Vec<String>,
    pub preferred_tempo_range: (u32, u32), // BPM
    pub preferred_duration_range: (u32, u32), // seconds
    pub language_preferences: Vec<String>,
    pub mood_preferences: Vec<String>,
    pub energy_level_preference: f64, // 0.0 to 1.0
    pub acousticness_preference: f64, // 0.0 to 1.0
    pub danceability_preference: f64, // 0.0 to 1.0
}

/// Recommendation Preferences
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationPreferences {
    pub diversity_weight: f64, // 0.0 to 1.0
    pub novelty_weight: f64, // 0.0 to 1.0
    pub familiarity_weight: f64, // 0.0 to 1.0
    pub include_explicit_content: bool,
    pub preferred_content_types: Vec<ContentType>,
    pub max_recommendations_per_request: usize,
    pub enable_cross_genre_recommendations: bool,
}

/// P2P Network Profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PNetworkProfile {
    pub node_id: String,
    pub connected_peers: Vec<String>,
    pub trust_scores: std::collections::HashMap<String, f64>,
    pub shared_content_count: u32,
    pub received_recommendations_count: u32,
    pub sent_recommendations_count: u32,
    pub network_reputation: f64,
}

/// Social Connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialConnection {
    pub connected_user_id: String,
    pub connection_type: ConnectionType,
    pub strength: f64, // 0.0 to 1.0
    pub created_at: DateTime<Utc>,
    pub last_interaction: DateTime<Utc>,
}

/// Connection Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Follow,
    Friend,
    Collaborator,
    SimilarTaste,
    P2PPeer,
}

/// Device Information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub platform: String,
    pub app_version: String,
    pub location: Option<String>,
    pub timezone: String,
}

/// Device Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    Mobile,
    Desktop,
    Tablet,
    SmartTV,
    Car,
    Wearable,
    Unknown,
}

/// Listening Context
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListeningContext {
    pub activity: ActivityType,
    pub mood: MoodType,
    pub time_of_day: TimeOfDay,
    pub day_of_week: DayOfWeek,
    pub location_type: LocationType,
    pub social_context: SocialContext,
}

/// Activity Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityType {
    Working,
    Exercising,
    Commuting,
    Relaxing,
    Socializing,
    Studying,
    Cooking,
    Sleeping,
    Unknown,
}

/// Mood Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoodType {
    Happy,
    Sad,
    Energetic,
    Calm,
    Focused,
    Romantic,
    Nostalgic,
    Unknown,
}

/// Time of Day
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
    Night,
}

/// Day of Week
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Location Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationType {
    Home,
    Work,
    Gym,
    Car,
    Public,
    Outdoors,
    Unknown,
}

/// Social Context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocialContext {
    Alone,
    WithFriends,
    WithFamily,
    WithPartner,
    InCrowd,
    Unknown,
}

/// Recommendation Reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationReason {
    pub reason_type: ReasonType,
    pub description: String,
    pub confidence: f64,
    pub supporting_evidence: Vec<String>,
}

/// Reason Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReasonType {
    SimilarGenre,
    SimilarArtist,
    CollaborativeFiltering,
    Trending,
    SocialConnection,
    P2PRecommendation,
    Editorial,
    Contextual,
}

/// Recommendation Metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationMetadata {
    pub content_metadata: ContentMetadata,
    pub user_metadata: UserMetadata,
    pub algorithm_metadata: AlgorithmMetadata,
    pub p2p_metadata: P2PMetadata,
}

/// Content Metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub genres: Vec<String>,
    pub duration: u32,
    pub popularity_score: f64,
    pub release_date: Option<DateTime<Utc>>,
    pub language: Option<String>,
}

/// User Metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserMetadata {
    pub user_preferences: Vec<String>,
    pub listening_patterns: Vec<String>,
    pub social_connections: u32,
    pub activity_level: f64,
}

/// Algorithm Metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlgorithmMetadata {
    pub model_version: String,
    pub training_data_size: u64,
    pub last_updated: DateTime<Utc>,
    pub confidence_interval: (f64, f64),
}

/// P2P Metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PMetadata {
    pub peer_count: u32,
    pub network_depth: u32,
    pub trust_score: f64,
    pub propagation_time_ms: u32,
}

/// Model Parameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelParameters {
    pub learning_rate: f64,
    pub batch_size: usize,
    pub epochs: u32,
    pub regularization_factor: f64,
    pub embedding_dimension: usize,
    pub hidden_layers: Vec<usize>,
    pub dropout_rate: f64,
}

/// Model Performance
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelPerformance {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub mae: f64, // Mean Absolute Error
    pub rmse: f64, // Root Mean Square Error
    pub training_time_seconds: u64,
    pub inference_time_ms: u32,
}

/// Training Data Info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainingDataInfo {
    pub total_samples: u64,
    pub training_samples: u64,
    pub validation_samples: u64,
    pub test_samples: u64,
    pub data_start_date: DateTime<Utc>,
    pub data_end_date: DateTime<Utc>,
    pub features_count: usize,
}

/// Recommendation Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationNode {
    pub node_id: String,
    pub user_id: String,
    pub node_type: NodeType,
    pub capabilities: Vec<NodeCapability>,
    pub trust_score: f64,
    pub last_seen: DateTime<Utc>,
    pub recommendation_history: Vec<String>, // Recommendation IDs
}

/// Node Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    User,
    ContentProvider,
    Aggregator,
    FederatedInstance,
    MLModel,
}

/// Node Capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeCapability {
    ContentRecommendation,
    UserMatching,
    ModelTraining,
    DataSharing,
    FederatedLearning,
    P2PCommunication,
}

/// Recommendation Connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationConnection {
    pub from_node: String,
    pub to_node: String,
    pub connection_type: ConnectionType,
    pub strength: f64,
    pub last_used: DateTime<Utc>,
    pub recommendation_count: u32,
}

/// Shared Knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedKnowledge {
    pub knowledge_id: String,
    pub knowledge_type: KnowledgeType,
    pub content: serde_json::Value,
    pub source_node: String,
    pub shared_at: DateTime<Utc>,
    pub expiration_at: Option<DateTime<Utc>>,
    pub trust_score: f64,
}

/// Knowledge Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KnowledgeType {
    UserPreference,
    ContentSimilarity,
    ModelParameter,
    RecommendationPattern,
    SocialConnection,
    TrendData,
}

/// Network Metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub total_nodes: u32,
    pub active_nodes: u32,
    pub total_connections: u32,
    pub average_degree: f64,
    pub network_diameter: u32,
    pub clustering_coefficient: f64,
    pub recommendation_accuracy: f64,
}

/// User Centroid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCentroid {
    pub genre_vector: Vec<f64>,
    pub artist_vector: Vec<f64>,
    pub feature_vector: Vec<f64>,
    pub activity_pattern: Vec<f64>,
    pub last_updated: DateTime<Utc>,
}

/// Similarity Score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore {
    pub user_id: String,
    pub target_user_id: String,
    pub similarity: f64,
    pub similarity_type: SimilarityType,
    pub calculated_at: DateTime<Utc>,
}

/// Similarity Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimilarityType {
    Cosine,
    Pearson,
    Jaccard,
    Euclidean,
    Manhattan,
    Custom(String),
}

/// Group Preferences
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupPreferences {
    pub common_genres: Vec<String>,
    pub common_artists: Vec<String>,
    pub average_preferences: MusicPreferences,
    pub diversity_score: f64,
    pub cohesion_score: f64,
} 