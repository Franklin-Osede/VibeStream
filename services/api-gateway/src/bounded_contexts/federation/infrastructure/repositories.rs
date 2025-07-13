use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::federation::domain::*;

// Repository Traits

#[async_trait]
pub trait FederatedInstanceRepository: Send + Sync {
    async fn save(&self, instance: &FederatedInstance) -> Result<(), AppError>;
    async fn find_by_domain(&self, domain: &str) -> Result<Option<FederatedInstance>, AppError>;
    async fn find_by_trust_level(&self, trust_level: TrustLevel) -> Result<Vec<FederatedInstance>, AppError>;
    async fn find_all(&self) -> Result<Vec<FederatedInstance>, AppError>;
    async fn delete(&self, domain: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait ActivityPubActivityRepository: Send + Sync {
    async fn save(&self, activity: &ActivityPubActivity) -> Result<(), AppError>;
    async fn find_by_activity_id(&self, activity_id: &str) -> Result<Option<ActivityPubActivity>, AppError>;
    async fn find_pending(&self) -> Result<Vec<ActivityPubActivity>, AppError>;
    async fn find_by_actor(&self, actor: &str) -> Result<Vec<ActivityPubActivity>, AppError>;
    async fn find_by_type(&self, activity_type: ActivityType) -> Result<Vec<ActivityPubActivity>, AppError>;
    async fn delete_old_activities(&self, older_than_days: u32) -> Result<(), AppError>;
}

#[async_trait]
pub trait FederatedUserRepository: Send + Sync {
    async fn save(&self, user: &FederatedUser) -> Result<(), AppError>;
    async fn find_by_uri(&self, uri: &str) -> Result<Option<FederatedUser>, AppError>;
    async fn find_by_username_domain(&self, username: &str, domain: &str) -> Result<Option<FederatedUser>, AppError>;
    async fn find_local_users(&self) -> Result<Vec<FederatedUser>, AppError>;
    async fn find_by_trust_level(&self, trust_level: TrustLevel) -> Result<Vec<FederatedUser>, AppError>;
    async fn delete(&self, uri: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait FederatedContentRepository: Send + Sync {
    async fn save(&self, content: &FederatedContent) -> Result<(), AppError>;
    async fn find_by_content_id(&self, content_id: &str) -> Result<Option<FederatedContent>, AppError>;
    async fn find_by_author(&self, author: &str) -> Result<Vec<FederatedContent>, AppError>;
    async fn find_by_type(&self, content_type: ContentType) -> Result<Vec<FederatedContent>, AppError>;
    async fn find_music_content(&self) -> Result<Vec<FederatedContent>, AppError>;
    async fn find_video_content(&self) -> Result<Vec<FederatedContent>, AppError>;
    async fn delete(&self, content_id: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait FederationFollowRepository: Send + Sync {
    async fn save(&self, follow: &FederationFollow) -> Result<(), AppError>;
    async fn find_by_follower_followee(&self, follower: &str, followee: &str) -> Result<Option<FederationFollow>, AppError>;
    async fn find_pending(&self) -> Result<Vec<FederationFollow>, AppError>;
    async fn find_by_follower(&self, follower: &str) -> Result<Vec<FederationFollow>, AppError>;
    async fn find_by_followee(&self, followee: &str) -> Result<Vec<FederationFollow>, AppError>;
    async fn delete(&self, follower: &str, followee: &str) -> Result<(), AppError>;
}

// In-Memory Implementations

pub struct InMemoryFederatedInstanceRepository {
    instances: Arc<RwLock<HashMap<String, FederatedInstance>>>,
}

impl InMemoryFederatedInstanceRepository {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl FederatedInstanceRepository for InMemoryFederatedInstanceRepository {
    async fn save(&self, instance: &FederatedInstance) -> Result<(), AppError> {
        let mut instances = self.instances.write().await;
        instances.insert(instance.domain.clone(), instance.clone());
        Ok(())
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Option<FederatedInstance>, AppError> {
        let instances = self.instances.read().await;
        Ok(instances.get(domain).cloned())
    }

    async fn find_by_trust_level(&self, trust_level: TrustLevel) -> Result<Vec<FederatedInstance>, AppError> {
        let instances = self.instances.read().await;
        let filtered: Vec<FederatedInstance> = instances
            .values()
            .filter(|instance| instance.trust_level == trust_level)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_all(&self) -> Result<Vec<FederatedInstance>, AppError> {
        let instances = self.instances.read().await;
        Ok(instances.values().cloned().collect())
    }

    async fn delete(&self, domain: &str) -> Result<(), AppError> {
        let mut instances = self.instances.write().await;
        instances.remove(domain);
        Ok(())
    }
}

pub struct InMemoryActivityPubActivityRepository {
    activities: Arc<RwLock<HashMap<String, ActivityPubActivity>>>,
}

impl InMemoryActivityPubActivityRepository {
    pub fn new() -> Self {
        Self {
            activities: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ActivityPubActivityRepository for InMemoryActivityPubActivityRepository {
    async fn save(&self, activity: &ActivityPubActivity) -> Result<(), AppError> {
        let mut activities = self.activities.write().await;
        activities.insert(activity.activity_id.clone(), activity.clone());
        Ok(())
    }

    async fn find_by_activity_id(&self, activity_id: &str) -> Result<Option<ActivityPubActivity>, AppError> {
        let activities = self.activities.read().await;
        Ok(activities.get(activity_id).cloned())
    }

    async fn find_pending(&self) -> Result<Vec<ActivityPubActivity>, AppError> {
        let activities = self.activities.read().await;
        let pending: Vec<ActivityPubActivity> = activities
            .values()
            .filter(|activity| !activity.processed)
            .cloned()
            .collect();
        Ok(pending)
    }

    async fn find_by_actor(&self, actor: &str) -> Result<Vec<ActivityPubActivity>, AppError> {
        let activities = self.activities.read().await;
        let filtered: Vec<ActivityPubActivity> = activities
            .values()
            .filter(|activity| activity.actor == actor)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_type(&self, activity_type: ActivityType) -> Result<Vec<ActivityPubActivity>, AppError> {
        let activities = self.activities.read().await;
        let filtered: Vec<ActivityPubActivity> = activities
            .values()
            .filter(|activity| activity.activity_type == activity_type)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn delete_old_activities(&self, older_than_days: u32) -> Result<(), AppError> {
        let mut activities = self.activities.write().await;
        let cutoff = chrono::Utc::now() - chrono::Duration::days(older_than_days as i64);
        
        activities.retain(|_, activity| activity.created_at > cutoff);
        Ok(())
    }
}

pub struct InMemoryFederatedUserRepository {
    users: Arc<RwLock<HashMap<String, FederatedUser>>>,
}

impl InMemoryFederatedUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl FederatedUserRepository for InMemoryFederatedUserRepository {
    async fn save(&self, user: &FederatedUser) -> Result<(), AppError> {
        let mut users = self.users.write().await;
        users.insert(user.full_uri.clone(), user.clone());
        Ok(())
    }

    async fn find_by_uri(&self, uri: &str) -> Result<Option<FederatedUser>, AppError> {
        let users = self.users.read().await;
        Ok(users.get(uri).cloned())
    }

    async fn find_by_username_domain(&self, username: &str, domain: &str) -> Result<Option<FederatedUser>, AppError> {
        let users = self.users.read().await;
        let uri = format!("https://{}/users/{}", domain, username);
        Ok(users.get(&uri).cloned())
    }

    async fn find_local_users(&self) -> Result<Vec<FederatedUser>, AppError> {
        let users = self.users.read().await;
        let local: Vec<FederatedUser> = users
            .values()
            .filter(|user| user.is_local)
            .cloned()
            .collect();
        Ok(local)
    }

    async fn find_by_trust_level(&self, trust_level: TrustLevel) -> Result<Vec<FederatedUser>, AppError> {
        let users = self.users.read().await;
        let filtered: Vec<FederatedUser> = users
            .values()
            .filter(|user| user.trust_level == trust_level)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn delete(&self, uri: &str) -> Result<(), AppError> {
        let mut users = self.users.write().await;
        users.remove(uri);
        Ok(())
    }
}

pub struct InMemoryFederatedContentRepository {
    contents: Arc<RwLock<HashMap<String, FederatedContent>>>,
}

impl InMemoryFederatedContentRepository {
    pub fn new() -> Self {
        Self {
            contents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl FederatedContentRepository for InMemoryFederatedContentRepository {
    async fn save(&self, content: &FederatedContent) -> Result<(), AppError> {
        let mut contents = self.contents.write().await;
        contents.insert(content.content_id.clone(), content.clone());
        Ok(())
    }

    async fn find_by_content_id(&self, content_id: &str) -> Result<Option<FederatedContent>, AppError> {
        let contents = self.contents.read().await;
        Ok(contents.get(content_id).cloned())
    }

    async fn find_by_author(&self, author: &str) -> Result<Vec<FederatedContent>, AppError> {
        let contents = self.contents.read().await;
        let filtered: Vec<FederatedContent> = contents
            .values()
            .filter(|content| content.author == author)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_type(&self, content_type: ContentType) -> Result<Vec<FederatedContent>, AppError> {
        let contents = self.contents.read().await;
        let filtered: Vec<FederatedContent> = contents
            .values()
            .filter(|content| content.content_type == content_type)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_music_content(&self) -> Result<Vec<FederatedContent>, AppError> {
        let contents = self.contents.read().await;
        let music: Vec<FederatedContent> = contents
            .values()
            .filter(|content| content.is_music_content())
            .cloned()
            .collect();
        Ok(music)
    }

    async fn find_video_content(&self) -> Result<Vec<FederatedContent>, AppError> {
        let contents = self.contents.read().await;
        let video: Vec<FederatedContent> = contents
            .values()
            .filter(|content| content.is_video_content())
            .cloned()
            .collect();
        Ok(video)
    }

    async fn delete(&self, content_id: &str) -> Result<(), AppError> {
        let mut contents = self.contents.write().await;
        contents.remove(content_id);
        Ok(())
    }
}

pub struct InMemoryFederationFollowRepository {
    follows: Arc<RwLock<HashMap<String, FederationFollow>>>,
}

impl InMemoryFederationFollowRepository {
    pub fn new() -> Self {
        Self {
            follows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn generate_key(follower: &str, followee: &str) -> String {
        format!("{}:{}", follower, followee)
    }
}

#[async_trait]
impl FederationFollowRepository for InMemoryFederationFollowRepository {
    async fn save(&self, follow: &FederationFollow) -> Result<(), AppError> {
        let mut follows = self.follows.write().await;
        let key = Self::generate_key(&follow.follower, &follow.followee);
        follows.insert(key, follow.clone());
        Ok(())
    }

    async fn find_by_follower_followee(&self, follower: &str, followee: &str) -> Result<Option<FederationFollow>, AppError> {
        let follows = self.follows.read().await;
        let key = Self::generate_key(follower, followee);
        Ok(follows.get(&key).cloned())
    }

    async fn find_pending(&self) -> Result<Vec<FederationFollow>, AppError> {
        let follows = self.follows.read().await;
        let pending: Vec<FederationFollow> = follows
            .values()
            .filter(|follow| follow.is_pending())
            .cloned()
            .collect();
        Ok(pending)
    }

    async fn find_by_follower(&self, follower: &str) -> Result<Vec<FederationFollow>, AppError> {
        let follows = self.follows.read().await;
        let filtered: Vec<FederationFollow> = follows
            .values()
            .filter(|follow| follow.follower == follower)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn find_by_followee(&self, followee: &str) -> Result<Vec<FederationFollow>, AppError> {
        let follows = self.follows.read().await;
        let filtered: Vec<FederationFollow> = follows
            .values()
            .filter(|follow| follow.followee == followee)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn delete(&self, follower: &str, followee: &str) -> Result<(), AppError> {
        let mut follows = self.follows.write().await;
        let key = Self::generate_key(follower, followee);
        follows.remove(&key);
        Ok(())
    }
} 