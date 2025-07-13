use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;
use super::repositories::*;
use crate::bounded_contexts::recommendation::domain::*;

/// Recommendation Application Service
/// Handles all recommendation business logic and coordinates between repositories
pub struct RecommendationApplicationService {
    user_profile_repository: Arc<dyn UserProfileRepository>,
    recommendation_repository: Arc<dyn RecommendationRepository>,
    model_repository: Arc<dyn RecommendationModelRepository>,
    p2p_network_repository: Arc<dyn P2PRecommendationNetworkRepository>,
    collaborative_group_repository: Arc<dyn CollaborativeFilteringGroupRepository>,
}

impl RecommendationApplicationService {
    pub fn new(
        user_profile_repository: Arc<dyn UserProfileRepository>,
        recommendation_repository: Arc<dyn RecommendationRepository>,
        model_repository: Arc<dyn RecommendationModelRepository>,
        p2p_network_repository: Arc<dyn P2PRecommendationNetworkRepository>,
        collaborative_group_repository: Arc<dyn CollaborativeFilteringGroupRepository>,
    ) -> Self {
        Self {
            user_profile_repository,
            recommendation_repository,
            model_repository,
            p2p_network_repository,
            collaborative_group_repository,
        }
    }

    // User Profile Management
    pub async fn create_user_profile(&self, user_id: String) -> Result<UserProfile, AppError> {
        let profile = UserProfile::new(user_id);
        self.user_profile_repository.save(&profile).await?;
        Ok(profile)
    }

    pub async fn get_user_profile(&self, user_id: &str) -> Result<Option<UserProfile>, AppError> {
        self.user_profile_repository.find_by_user_id(user_id).await
    }

    pub async fn update_user_preferences(&self, user_id: &str, preferences: MusicPreferences) -> Result<(), AppError> {
        let mut profile = self.user_profile_repository.find_by_user_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User profile not found".to_string()))?;
        
        profile.update_preferences(preferences);
        self.user_profile_repository.save(&profile).await?;
        Ok(())
    }

    // Listening Event Management
    pub async fn record_listening_event(&self, event: ListeningEvent) -> Result<(), AppError> {
        // Save the listening event
        let mut profile = self.user_profile_repository.find_by_user_id(&event.user_id).await?
            .ok_or_else(|| AppError::NotFound("User profile not found".to_string()))?;
        
        profile.add_listening_event(event);
        self.user_profile_repository.save(&profile).await?;
        
        // Trigger recommendation updates
        self.update_recommendations_for_user(&event.user_id).await?;
        
        Ok(())
    }

    // Recommendation Generation
    pub async fn generate_recommendations(&self, user_id: &str, limit: usize) -> Result<Vec<Recommendation>, AppError> {
        let profile = self.user_profile_repository.find_by_user_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User profile not found".to_string()))?;
        
        let mut recommendations = Vec::new();
        
        // Generate recommendations using different algorithms
        if let Ok(collaborative_recs) = self.generate_collaborative_recommendations(&profile, limit / 3).await {
            recommendations.extend(collaborative_recs);
        }
        
        if let Ok(content_based_recs) = self.generate_content_based_recommendations(&profile, limit / 3).await {
            recommendations.extend(content_based_recs);
        }
        
        if let Ok(p2p_recs) = self.generate_p2p_recommendations(&profile, limit / 3).await {
            recommendations.extend(p2p_recs);
        }
        
        // Sort by score and take top recommendations
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        recommendations.truncate(limit);
        
        // Save recommendations
        for recommendation in &recommendations {
            self.recommendation_repository.save(recommendation).await?;
        }
        
        Ok(recommendations)
    }

    pub async fn get_user_recommendations(&self, user_id: &str, limit: usize) -> Result<Vec<Recommendation>, AppError> {
        let recommendations = self.recommendation_repository.find_by_user_id(user_id).await?;
        
        // Filter out expired recommendations
        let valid_recommendations: Vec<Recommendation> = recommendations
            .into_iter()
            .filter(|r| !r.is_expired())
            .collect();
        
        // Sort by score and return top recommendations
        let mut sorted = valid_recommendations;
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        sorted.truncate(limit);
        
        Ok(sorted)
    }

    // Collaborative Filtering
    async fn generate_collaborative_recommendations(&self, profile: &UserProfile, limit: usize) -> Result<Vec<Recommendation>, AppError> {
        let mut recommendations = Vec::new();
        
        // Find similar users
        let similar_users = self.find_similar_users(profile, 10).await?;
        
        for similar_user in similar_users {
            // Get content that similar user liked but current user hasn't heard
            let user_content = self.get_user_liked_content(&similar_user.user_id).await?;
            let user_heard_content: std::collections::HashSet<String> = profile.listening_history
                .iter()
                .map(|e| e.content_id.clone())
                .collect();
            
            for content in user_content {
                if !user_heard_content.contains(&content.content_id) {
                    let recommendation = Recommendation::new(
                        profile.user_id.clone(),
                        content.content_id,
                        content.content_type,
                        similar_user.similarity * 0.8, // Weight by similarity
                        RecommendationAlgorithm::CollaborativeFiltering,
                        RecommendationSource::Algorithmic,
                    );
                    
                    recommendation.add_reason(RecommendationReason {
                        reason_type: ReasonType::CollaborativeFiltering,
                        description: format!("Recommended by similar user {}", similar_user.user_id),
                        confidence: similar_user.similarity,
                        supporting_evidence: vec![similar_user.user_id.clone()],
                    });
                    
                    recommendations.push(recommendation);
                    
                    if recommendations.len() >= limit {
                        break;
                    }
                }
            }
        }
        
        Ok(recommendations)
    }

    // Content-Based Filtering
    async fn generate_content_based_recommendations(&self, profile: &UserProfile, limit: usize) -> Result<Vec<Recommendation>, AppError> {
        let mut recommendations = Vec::new();
        
        // Get user's favorite genres and artists
        let top_genres = profile.get_top_genres(5);
        let favorite_artists: std::collections::HashSet<String> = profile.music_preferences.favorite_artists
            .iter()
            .cloned()
            .collect();
        
        // Find content with similar characteristics
        for genre in top_genres {
            let similar_content = self.find_content_by_genre(&genre, limit / top_genres.len()).await?;
            
            for content in similar_content {
                let score = self.calculate_content_similarity(profile, &content).await?;
                
                if score > 0.6 {
                    let recommendation = Recommendation::new(
                        profile.user_id.clone(),
                        content.content_id,
                        content.content_type,
                        score,
                        RecommendationAlgorithm::ContentBasedFiltering,
                        RecommendationSource::Algorithmic,
                    );
                    
                    recommendation.add_reason(RecommendationReason {
                        reason_type: ReasonType::SimilarGenre,
                        description: format!("Similar to your favorite genre: {}", genre),
                        confidence: score,
                        supporting_evidence: vec![genre.clone()],
                    });
                    
                    recommendations.push(recommendation);
                }
            }
        }
        
        Ok(recommendations)
    }

    // P2P Recommendations
    async fn generate_p2p_recommendations(&self, profile: &UserProfile, limit: usize) -> Result<Vec<Recommendation>, AppError> {
        let mut recommendations = Vec::new();
        
        // Get P2P network recommendations
        let p2p_recommendations = self.get_p2p_network_recommendations(&profile.user_id, limit).await?;
        
        for p2p_rec in p2p_recommendations {
            let recommendation = Recommendation::new(
                profile.user_id.clone(),
                p2p_rec.content_id,
                p2p_rec.content_type,
                p2p_rec.score,
                RecommendationAlgorithm::P2PCollaborative,
                RecommendationSource::P2PNetwork,
            );
            
            recommendation.add_reason(RecommendationReason {
                reason_type: ReasonType::P2PRecommendation,
                description: "Recommended by P2P network peers".to_string(),
                confidence: p2p_rec.score,
                supporting_evidence: vec!["P2P Network".to_string()],
            });
            
            recommendations.push(recommendation);
        }
        
        Ok(recommendations)
    }

    // Model Management
    pub async fn train_recommendation_model(&self, model_type: ModelType) -> Result<RecommendationModel, AppError> {
        let model = RecommendationModel::new(
            model_type,
            "1.0.0".to_string(),
            ModelParameters::default(),
        );
        
        // Start training process
        self.start_model_training(&model).await?;
        
        self.model_repository.save(&model).await?;
        Ok(model)
    }

    pub async fn get_active_models(&self) -> Result<Vec<RecommendationModel>, AppError> {
        self.model_repository.find_active_models().await
    }

    // P2P Network Management
    pub async fn join_p2p_network(&self, user_id: &str, network_id: &str) -> Result<(), AppError> {
        let node = RecommendationNode {
            node_id: format!("node_{}", uuid::Uuid::new_v4()),
            user_id: user_id.to_string(),
            node_type: NodeType::User,
            capabilities: vec![NodeCapability::ContentRecommendation, NodeCapability::P2PCommunication],
            trust_score: 1.0,
            last_seen: Utc::now(),
            recommendation_history: Vec::new(),
        };
        
        let mut network = self.p2p_network_repository.find_by_network_id(network_id).await?
            .ok_or_else(|| AppError::NotFound("P2P network not found".to_string()))?;
        
        network.add_node(node);
        self.p2p_network_repository.save(&network).await?;
        
        Ok(())
    }

    // Private helper methods
    async fn update_recommendations_for_user(&self, user_id: &str) -> Result<(), AppError> {
        // This would trigger background recommendation updates
        // Implementation would be in a background task
        Ok(())
    }

    async fn find_similar_users(&self, profile: &UserProfile, limit: usize) -> Result<Vec<SimilarityScore>, AppError> {
        // This would implement user similarity calculation
        // For now, return empty vector
        Ok(Vec::new())
    }

    async fn get_user_liked_content(&self, user_id: &str) -> Result<Vec<ContentMetadata>, AppError> {
        // This would get content that user has liked
        // For now, return empty vector
        Ok(Vec::new())
    }

    async fn find_content_by_genre(&self, genre: &str, limit: usize) -> Result<Vec<ContentMetadata>, AppError> {
        // This would find content by genre
        // For now, return empty vector
        Ok(Vec::new())
    }

    async fn calculate_content_similarity(&self, profile: &UserProfile, content: &ContentMetadata) -> Result<f64, AppError> {
        // This would calculate content similarity
        // For now, return a random score
        Ok(0.7)
    }

    async fn get_p2p_network_recommendations(&self, user_id: &str, limit: usize) -> Result<Vec<Recommendation>, AppError> {
        // This would get recommendations from P2P network
        // For now, return empty vector
        Ok(Vec::new())
    }

    async fn start_model_training(&self, model: &RecommendationModel) -> Result<(), AppError> {
        // This would start the model training process
        // Implementation would be in a background task
        Ok(())
    }
} 