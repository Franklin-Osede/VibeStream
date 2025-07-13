use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::recommendation::domain::*;

// Repository Traits

#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn save(&self, profile: &UserProfile) -> Result<(), AppError>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<UserProfile>, AppError>;
    async fn find_by_preferences(&self, preferences: &MusicPreferences) -> Result<Vec<UserProfile>, AppError>;
    async fn find_similar_users(&self, user_id: &str, limit: usize) -> Result<Vec<UserProfile>, AppError>;
    async fn delete(&self, user_id: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait RecommendationRepository: Send + Sync {
    async fn save(&self, recommendation: &Recommendation) -> Result<(), AppError>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Recommendation>, AppError>;
    async fn find_by_content_id(&self, content_id: &str) -> Result<Vec<Recommendation>, AppError>;
    async fn find_by_algorithm(&self, algorithm: RecommendationAlgorithm) -> Result<Vec<Recommendation>, AppError>;
    async fn find_expired(&self) -> Result<Vec<Recommendation>, AppError>;
    async fn delete_expired(&self) -> Result<(), AppError>;
}

#[async_trait]
pub trait RecommendationModelRepository: Send + Sync {
    async fn save(&self, model: &RecommendationModel) -> Result<(), AppError>;
    async fn find_by_id(&self, model_id: &str) -> Result<Option<RecommendationModel>, AppError>;
    async fn find_active_models(&self) -> Result<Vec<RecommendationModel>, AppError>;
    async fn find_by_type(&self, model_type: ModelType) -> Result<Vec<RecommendationModel>, AppError>;
    async fn delete(&self, model_id: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait P2PRecommendationNetworkRepository: Send + Sync {
    async fn save(&self, network: &P2PRecommendationNetwork) -> Result<(), AppError>;
    async fn find_by_network_id(&self, network_id: &str) -> Result<Option<P2PRecommendationNetwork>, AppError>;
    async fn find_by_node_id(&self, node_id: &str) -> Result<Vec<P2PRecommendationNetwork>, AppError>;
    async fn find_all(&self) -> Result<Vec<P2PRecommendationNetwork>, AppError>;
    async fn delete(&self, network_id: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait CollaborativeFilteringGroupRepository: Send + Sync {
    async fn save(&self, group: &CollaborativeFilteringGroup) -> Result<(), AppError>;
    async fn find_by_group_id(&self, group_id: &str) -> Result<Option<CollaborativeFilteringGroup>, AppError>;
    async fn find_by_member(&self, user_id: &str) -> Result<Vec<CollaborativeFilteringGroup>, AppError>;
    async fn find_similar_groups(&self, group_id: &str, limit: usize) -> Result<Vec<CollaborativeFilteringGroup>, AppError>;
    async fn delete(&self, group_id: &str) -> Result<(), AppError>;
}

// In-Memory Implementations

pub struct InMemoryUserProfileRepository {
    profiles: Arc<RwLock<HashMap<String, UserProfile>>>,
}

impl InMemoryUserProfileRepository {
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserProfileRepository for InMemoryUserProfileRepository {
    async fn save(&self, profile: &UserProfile) -> Result<(), AppError> {
        let mut profiles = self.profiles.write().await;
        profiles.insert(profile.user_id.clone(), profile.clone());
        Ok(())
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Option<UserProfile>, AppError> {
        let profiles = self.profiles.read().await;
        Ok(profiles.get(user_id).cloned())
    }

    async fn find_by_preferences(&self, preferences: &MusicPreferences) -> Result<Vec<UserProfile>, AppError> {
        let profiles = self.profiles.read().await;
        let filtered: Vec<UserProfile> = profiles
            .values()
            .filter(|profile| {
                // Simple preference matching - could be more sophisticated
                !profile.music_preferences.favorite_genres.is_empty() &&
                !preferences.favorite_genres.is_empty()
            })
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_similar_users(&self, user_id: &str, limit: usize) -> Result<Vec<UserProfile>, AppError> {
        let profiles = self.profiles.read().await;
        let current_profile = profiles.get(user_id)
            .ok_or_else(|| AppError::NotFound("User profile not found".to_string()))?;
        
        let mut similar_profiles: Vec<UserProfile> = profiles
            .values()
            .filter(|p| p.user_id != user_id)
            .cloned()
            .collect();
        
        // Simple similarity based on shared genres
        similar_profiles.sort_by(|a, b| {
            let a_similarity = self.calculate_similarity(current_profile, a);
            let b_similarity = self.calculate_similarity(current_profile, b);
            b_similarity.partial_cmp(&a_similarity).unwrap()
        });
        
        similar_profiles.truncate(limit);
        Ok(similar_profiles)
    }

    async fn delete(&self, user_id: &str) -> Result<(), AppError> {
        let mut profiles = self.profiles.write().await;
        profiles.remove(user_id);
        Ok(())
    }
}

impl InMemoryUserProfileRepository {
    fn calculate_similarity(&self, profile1: &UserProfile, profile2: &UserProfile) -> f64 {
        let genres1: std::collections::HashSet<String> = profile1.music_preferences.favorite_genres
            .iter()
            .cloned()
            .collect();
        let genres2: std::collections::HashSet<String> = profile2.music_preferences.favorite_genres
            .iter()
            .cloned()
            .collect();
        
        let intersection = genres1.intersection(&genres2).count();
        let union = genres1.union(&genres2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

pub struct InMemoryRecommendationRepository {
    recommendations: Arc<RwLock<HashMap<String, Recommendation>>>,
}

impl InMemoryRecommendationRepository {
    pub fn new() -> Self {
        Self {
            recommendations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl RecommendationRepository for InMemoryRecommendationRepository {
    async fn save(&self, recommendation: &Recommendation) -> Result<(), AppError> {
        let mut recommendations = self.recommendations.write().await;
        recommendations.insert(recommendation.id.to_string(), recommendation.clone());
        Ok(())
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Recommendation>, AppError> {
        let recommendations = self.recommendations.read().await;
        let filtered: Vec<Recommendation> = recommendations
            .values()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_content_id(&self, content_id: &str) -> Result<Vec<Recommendation>, AppError> {
        let recommendations = self.recommendations.read().await;
        let filtered: Vec<Recommendation> = recommendations
            .values()
            .filter(|r| r.content_id == content_id)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_algorithm(&self, algorithm: RecommendationAlgorithm) -> Result<Vec<Recommendation>, AppError> {
        let recommendations = self.recommendations.read().await;
        let filtered: Vec<Recommendation> = recommendations
            .values()
            .filter(|r| r.algorithm == algorithm)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_expired(&self) -> Result<Vec<Recommendation>, AppError> {
        let recommendations = self.recommendations.read().await;
        let expired: Vec<Recommendation> = recommendations
            .values()
            .filter(|r| r.is_expired())
            .cloned()
            .collect();
        Ok(expired)
    }

    async fn delete_expired(&self) -> Result<(), AppError> {
        let mut recommendations = self.recommendations.write().await;
        recommendations.retain(|_, r| !r.is_expired());
        Ok(())
    }
}

pub struct InMemoryRecommendationModelRepository {
    models: Arc<RwLock<HashMap<String, RecommendationModel>>>,
}

impl InMemoryRecommendationModelRepository {
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl RecommendationModelRepository for InMemoryRecommendationModelRepository {
    async fn save(&self, model: &RecommendationModel) -> Result<(), AppError> {
        let mut models = self.models.write().await;
        models.insert(model.id.to_string(), model.clone());
        Ok(())
    }

    async fn find_by_id(&self, model_id: &str) -> Result<Option<RecommendationModel>, AppError> {
        let models = self.models.read().await;
        Ok(models.get(model_id).cloned())
    }

    async fn find_active_models(&self) -> Result<Vec<RecommendationModel>, AppError> {
        let models = self.models.read().await;
        let active: Vec<RecommendationModel> = models
            .values()
            .filter(|m| m.is_active())
            .cloned()
            .collect();
        Ok(active)
    }

    async fn find_by_type(&self, model_type: ModelType) -> Result<Vec<RecommendationModel>, AppError> {
        let models = self.models.read().await;
        let filtered: Vec<RecommendationModel> = models
            .values()
            .filter(|m| m.model_type == model_type)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn delete(&self, model_id: &str) -> Result<(), AppError> {
        let mut models = self.models.write().await;
        models.remove(model_id);
        Ok(())
    }
}

pub struct InMemoryP2PRecommendationNetworkRepository {
    networks: Arc<RwLock<HashMap<String, P2PRecommendationNetwork>>>,
}

impl InMemoryP2PRecommendationNetworkRepository {
    pub fn new() -> Self {
        Self {
            networks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl P2PRecommendationNetworkRepository for InMemoryP2PRecommendationNetworkRepository {
    async fn save(&self, network: &P2PRecommendationNetwork) -> Result<(), AppError> {
        let mut networks = self.networks.write().await;
        networks.insert(network.network_id.clone(), network.clone());
        Ok(())
    }

    async fn find_by_network_id(&self, network_id: &str) -> Result<Option<P2PRecommendationNetwork>, AppError> {
        let networks = self.networks.read().await;
        Ok(networks.get(network_id).cloned())
    }

    async fn find_by_node_id(&self, node_id: &str) -> Result<Vec<P2PRecommendationNetwork>, AppError> {
        let networks = self.networks.read().await;
        let filtered: Vec<P2PRecommendationNetwork> = networks
            .values()
            .filter(|n| n.nodes.iter().any(|node| node.node_id == node_id))
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_all(&self) -> Result<Vec<P2PRecommendationNetwork>, AppError> {
        let networks = self.networks.read().await;
        Ok(networks.values().cloned().collect())
    }

    async fn delete(&self, network_id: &str) -> Result<(), AppError> {
        let mut networks = self.networks.write().await;
        networks.remove(network_id);
        Ok(())
    }
}

pub struct InMemoryCollaborativeFilteringGroupRepository {
    groups: Arc<RwLock<HashMap<String, CollaborativeFilteringGroup>>>,
}

impl InMemoryCollaborativeFilteringGroupRepository {
    pub fn new() -> Self {
        Self {
            groups: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CollaborativeFilteringGroupRepository for InMemoryCollaborativeFilteringGroupRepository {
    async fn save(&self, group: &CollaborativeFilteringGroup) -> Result<(), AppError> {
        let mut groups = self.groups.write().await;
        groups.insert(group.group_id.clone(), group.clone());
        Ok(())
    }

    async fn find_by_group_id(&self, group_id: &str) -> Result<Option<CollaborativeFilteringGroup>, AppError> {
        let groups = self.groups.read().await;
        Ok(groups.get(group_id).cloned())
    }

    async fn find_by_member(&self, user_id: &str) -> Result<Vec<CollaborativeFilteringGroup>, AppError> {
        let groups = self.groups.read().await;
        let filtered: Vec<CollaborativeFilteringGroup> = groups
            .values()
            .filter(|g| g.members.contains(&user_id.to_string()))
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_similar_groups(&self, group_id: &str, limit: usize) -> Result<Vec<CollaborativeFilteringGroup>, AppError> {
        let groups = self.groups.read().await;
        let current_group = groups.get(group_id)
            .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;
        
        let mut similar_groups: Vec<CollaborativeFilteringGroup> = groups
            .values()
            .filter(|g| g.group_id != group_id)
            .cloned()
            .collect();
        
        // Simple similarity based on group size and preferences
        similar_groups.sort_by(|a, b| {
            let a_similarity = (a.get_group_size() as i32 - current_group.get_group_size() as i32).abs();
            let b_similarity = (b.get_group_size() as i32 - current_group.get_group_size() as i32).abs();
            a_similarity.cmp(&b_similarity)
        });
        
        similar_groups.truncate(limit);
        Ok(similar_groups)
    }

    async fn delete(&self, group_id: &str) -> Result<(), AppError> {
        let mut groups = self.groups.write().await;
        groups.remove(group_id);
        Ok(())
    }
} 