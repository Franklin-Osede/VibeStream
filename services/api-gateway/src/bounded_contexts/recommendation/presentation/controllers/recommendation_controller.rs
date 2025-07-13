use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::recommendation::application::RecommendationApplicationService;
use crate::bounded_contexts::recommendation::domain::*;

/// Recommendation Controller for P2P Music Discovery
pub struct RecommendationController {
    recommendation_service: Arc<RecommendationApplicationService>,
}

impl RecommendationController {
    pub fn new(recommendation_service: Arc<RecommendationApplicationService>) -> Self {
        Self { recommendation_service }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            // User profile endpoints
            .route("/profiles", post(Self::create_user_profile))
            .route("/profiles/:user_id", get(Self::get_user_profile))
            .route("/profiles/:user_id/preferences", post(Self::update_user_preferences))
            
            // Listening events
            .route("/events", post(Self::record_listening_event))
            
            // Recommendations
            .route("/recommendations/:user_id", get(Self::get_recommendations))
            .route("/recommendations/:user_id/generate", post(Self::generate_recommendations))
            
            // Models
            .route("/models", get(Self::get_active_models))
            .route("/models", post(Self::train_model))
            
            // P2P Network
            .route("/p2p/join", post(Self::join_p2p_network))
            .route("/p2p/networks", get(Self::get_p2p_networks))
            
            // Analytics
            .route("/analytics/:user_id", get(Self::get_user_analytics))
            .route("/analytics/trending", get(Self::get_trending_content))
        }
}

// User Profile Endpoints

#[derive(Deserialize)]
struct CreateProfileRequest {
    user_id: String,
}

async fn create_user_profile(
    State(controller): State<Arc<RecommendationController>>,
    Json(request): Json<CreateProfileRequest>,
) -> Result<Json<UserProfile>, AppError> {
    let profile = controller.recommendation_service.create_user_profile(request.user_id).await?;
    Ok(Json(profile))
}

async fn get_user_profile(
    State(controller): State<Arc<RecommendationController>>,
    Path(user_id): Path<String>,
) -> Result<Json<Option<UserProfile>>, AppError> {
    let profile = controller.recommendation_service.get_user_profile(&user_id).await?;
    Ok(Json(profile))
}

#[derive(Deserialize)]
struct UpdatePreferencesRequest {
    preferences: MusicPreferences,
}

async fn update_user_preferences(
    State(controller): State<Arc<RecommendationController>>,
    Path(user_id): Path<String>,
    Json(request): Json<UpdatePreferencesRequest>,
) -> Result<StatusCode, AppError> {
    controller.recommendation_service.update_user_preferences(&user_id, request.preferences).await?;
    Ok(StatusCode::OK)
}

// Listening Events

#[derive(Deserialize)]
struct RecordListeningEventRequest {
    user_id: String,
    content_id: String,
    content_type: ContentType,
    duration_seconds: u32,
    completion_rate: f64,
    genres: Vec<String>,
    artists: Vec<String>,
}

async fn record_listening_event(
    State(controller): State<Arc<RecommendationController>>,
    Json(request): Json<RecordListeningEventRequest>,
) -> Result<StatusCode, AppError> {
    let event = ListeningEvent::new(
        request.user_id,
        request.content_id,
        request.content_type,
        request.duration_seconds,
        request.completion_rate,
        request.genres,
        request.artists,
    );
    
    controller.recommendation_service.record_listening_event(event).await?;
    Ok(StatusCode::CREATED)
}

// Recommendations

#[derive(Deserialize)]
struct GetRecommendationsQuery {
    limit: Option<usize>,
}

async fn get_recommendations(
    State(controller): State<Arc<RecommendationController>>,
    Path(user_id): Path<String>,
    Query(query): Query<GetRecommendationsQuery>,
) -> Result<Json<Vec<Recommendation>>, AppError> {
    let limit = query.limit.unwrap_or(20);
    let recommendations = controller.recommendation_service.get_user_recommendations(&user_id, limit).await?;
    Ok(Json(recommendations))
}

#[derive(Deserialize)]
struct GenerateRecommendationsRequest {
    limit: Option<usize>,
}

#[derive(Serialize)]
struct GenerateRecommendationsResponse {
    recommendations: Vec<Recommendation>,
    generated_at: chrono::DateTime<chrono::Utc>,
    algorithm_used: String,
}

async fn generate_recommendations(
    State(controller): State<Arc<RecommendationController>>,
    Path(user_id): Path<String>,
    Json(request): Json<GenerateRecommendationsRequest>,
) -> Result<Json<GenerateRecommendationsResponse>, AppError> {
    let limit = request.limit.unwrap_or(20);
    let recommendations = controller.recommendation_service.generate_recommendations(&user_id, limit).await?;
    
    let response = GenerateRecommendationsResponse {
        recommendations,
        generated_at: Utc::now(),
        algorithm_used: "Hybrid P2P".to_string(),
    };
    
    Ok(Json(response))
}

// Models

async fn get_active_models(
    State(controller): State<Arc<RecommendationController>>,
) -> Result<Json<Vec<RecommendationModel>>, AppError> {
    let models = controller.recommendation_service.get_active_models().await?;
    Ok(Json(models))
}

#[derive(Deserialize)]
struct TrainModelRequest {
    model_type: ModelType,
}

async fn train_model(
    State(controller): State<Arc<RecommendationController>>,
    Json(request): Json<TrainModelRequest>,
) -> Result<Json<RecommendationModel>, AppError> {
    let model = controller.recommendation_service.train_recommendation_model(request.model_type).await?;
    Ok(Json(model))
}

// P2P Network

#[derive(Deserialize)]
struct JoinP2PNetworkRequest {
    user_id: String,
    network_id: String,
}

async fn join_p2p_network(
    State(controller): State<Arc<RecommendationController>>,
    Json(request): Json<JoinP2PNetworkRequest>,
) -> Result<StatusCode, AppError> {
    controller.recommendation_service.join_p2p_network(&request.user_id, &request.network_id).await?;
    Ok(StatusCode::OK)
}

#[derive(Serialize)]
struct P2PNetworkInfo {
    network_id: String,
    total_nodes: u32,
    active_nodes: u32,
    recommendation_accuracy: f64,
}

async fn get_p2p_networks(
    State(controller): State<Arc<RecommendationController>>,
) -> Result<Json<Vec<P2PNetworkInfo>>, AppError> {
    // This would get P2P network information
    // For now, return mock data
    let networks = vec![
        P2PNetworkInfo {
            network_id: "main_network".to_string(),
            total_nodes: 1000,
            active_nodes: 850,
            recommendation_accuracy: 0.87,
        },
        P2PNetworkInfo {
            network_id: "federated_network".to_string(),
            total_nodes: 500,
            active_nodes: 420,
            recommendation_accuracy: 0.82,
        },
    ];
    
    Ok(Json(networks))
}

// Analytics

#[derive(Serialize)]
struct UserAnalytics {
    user_id: String,
    total_listening_time: u64,
    favorite_genres: Vec<String>,
    top_artists: Vec<String>,
    recommendation_accuracy: f64,
    p2p_contributions: u32,
    social_connections: u32,
}

async fn get_user_analytics(
    State(controller): State<Arc<RecommendationController>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserAnalytics>, AppError> {
    let profile = controller.recommendation_service.get_user_profile(&user_id).await?;
    
    let analytics = if let Some(profile) = profile {
        let total_listening_time: u64 = profile.listening_history
            .iter()
            .map(|e| e.duration_seconds as u64)
            .sum();
        
        let favorite_genres = profile.get_top_genres(5);
        let top_artists = profile.music_preferences.favorite_artists;
        
        UserAnalytics {
            user_id,
            total_listening_time,
            favorite_genres,
            top_artists,
            recommendation_accuracy: 0.85, // Mock data
            p2p_contributions: profile.p2p_network.shared_content_count,
            social_connections: profile.social_connections.len() as u32,
        }
    } else {
        UserAnalytics {
            user_id,
            total_listening_time: 0,
            favorite_genres: vec![],
            top_artists: vec![],
            recommendation_accuracy: 0.0,
            p2p_contributions: 0,
            social_connections: 0,
        }
    };
    
    Ok(Json(analytics))
}

#[derive(Serialize)]
struct TrendingContent {
    content_id: String,
    title: String,
    artist: String,
    genre: String,
    trending_score: f64,
    recommendation_count: u32,
    p2p_propagation_rate: f64,
}

async fn get_trending_content(
    State(controller): State<Arc<RecommendationController>>,
) -> Result<Json<Vec<TrendingContent>>, AppError> {
    // This would get trending content based on recommendations and P2P propagation
    // For now, return mock data
    let trending = vec![
        TrendingContent {
            content_id: "song_1".to_string(),
            title: "Revolutionary Beat".to_string(),
            artist: "P2P Artist".to_string(),
            genre: "Electronic".to_string(),
            trending_score: 0.95,
            recommendation_count: 150,
            p2p_propagation_rate: 0.87,
        },
        TrendingContent {
            content_id: "song_2".to_string(),
            title: "Decentralized Harmony".to_string(),
            artist: "Federated Musician".to_string(),
            genre: "Hip-Hop".to_string(),
            trending_score: 0.88,
            recommendation_count: 120,
            p2p_propagation_rate: 0.82,
        },
    ];
    
    Ok(Json(trending))
} 