use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

use super::{
    aggregates::UserAggregate,
    value_objects::{UserId, Email, Username, PasswordHash, UserTier, UserRole},
    repository::UserRepository,
    specifications::{EmailSpecification, UsernameSpecification, PasswordSpecification},
};
use crate::shared::domain::errors::VibeStreamError;

/// Authentication domain service
#[async_trait]
pub trait AuthenticationDomainService: Send + Sync {
    /// Authenticate user with email/username and password
    async fn authenticate(
        &self,
        credential: String, // email or username
        password: String,
    ) -> Result<UserAggregate, VibeStreamError>;

    /// Generate password hash
    fn hash_password(&self, password: &str) -> Result<PasswordHash, VibeStreamError>;

    /// Verify password against hash
    fn verify_password(&self, password: &str, hash: &PasswordHash) -> Result<bool, VibeStreamError>;

    /// Check if password meets security requirements
    fn validate_password_strength(&self, password: &str) -> Result<(), VibeStreamError>;

    /// Generate secure token for email verification, password reset, etc.
    fn generate_secure_token(&self) -> String;

    /// Validate secure token
    fn validate_secure_token(&self, token: &str, created_at: DateTime<Utc>) -> Result<bool, VibeStreamError>;
}

/// User domain service for complex business logic
#[async_trait]
pub trait UserDomainService: Send + Sync {
    /// Register new user
    async fn register_user(
        &self,
        email: Email,
        username: Username,
        password: String,
    ) -> Result<UserAggregate, VibeStreamError>;

    /// Check if user can upgrade to tier
    async fn can_upgrade_to_tier(
        &self,
        user_id: &UserId,
        target_tier: &UserTier,
    ) -> Result<bool, VibeStreamError>;

    /// Calculate tier upgrade requirements
    async fn get_tier_upgrade_requirements(
        &self,
        user_id: &UserId,
        target_tier: &UserTier,
    ) -> Result<TierUpgradeRequirements, VibeStreamError>;

    /// Check for achievement eligibility
    async fn check_achievements(&self, user_id: &UserId) -> Result<Vec<Achievement>, VibeStreamError>;

    /// Calculate user reputation score
    async fn calculate_reputation_score(&self, user_id: &UserId) -> Result<f64, VibeStreamError>;

    /// Check if user is eligible for special features
    async fn check_feature_eligibility(
        &self,
        user_id: &UserId,
        feature: &str,
    ) -> Result<bool, VibeStreamError>;

    /// Get user activity summary
    async fn get_activity_summary(&self, user_id: &UserId) -> Result<UserActivitySummary, VibeStreamError>;
}

/// Password domain service for password-related operations
pub struct PasswordDomainService;

impl PasswordDomainService {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> Result<PasswordHash, VibeStreamError> {
        let hash_result = hash(password, DEFAULT_COST)
            .map_err(|e| VibeStreamError::Internal {
                message: format!("Error hashing password: {}", e),
            })?;
        Ok(PasswordHash::new(hash_result))
    }

    pub fn verify_password(&self, password: &str, hash: &PasswordHash) -> Result<bool, VibeStreamError> {
        verify(password, hash.value())
            .map_err(|e| VibeStreamError::Internal {
                message: format!("Error verifying password: {}", e),
            })
    }

    pub fn validate_password_strength(&self, password: &str) -> Result<(), VibeStreamError> {
        let spec = PasswordSpecification::new();
        if spec.is_satisfied_by(password) {
            Ok(())
        } else {
            Err(VibeStreamError::ValidationError {
                field: "password".to_string(),
                message: "Password does not meet security requirements".to_string(),
            })
        }
    }
}

/// User validation service
pub struct UserValidationService {
    user_repository: Box<dyn UserRepository>,
}

impl UserValidationService {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Validate user registration data
    pub async fn validate_registration(
        &self,
        email: &Email,
        username: &Username,
        password: &str,
    ) -> Result<(), VibeStreamError> {
        // Check email specification
        let email_spec = EmailSpecification::new();
        if !email_spec.is_satisfied_by(email) {
            return Err(VibeStreamError::ValidationError {
                field: "email".to_string(),
                message: "Email format is invalid".to_string(),
            });
        }

        // Check username specification
        let username_spec = UsernameSpecification::new();
        if !username_spec.is_satisfied_by(username) {
            return Err(VibeStreamError::ValidationError {
                field: "username".to_string(),
                message: "Username format is invalid".to_string(),
            });
        }

        // Check password specification
        let password_spec = PasswordSpecification::new();
        if !password_spec.is_satisfied_by(password) {
            return Err(VibeStreamError::ValidationError {
                field: "password".to_string(),
                message: "Password does not meet requirements".to_string(),
            });
        }

        // Check uniqueness
        if self.user_repository.email_exists(email).await? {
            return Err(VibeStreamError::ValidationError {
                field: "email".to_string(),
                message: "Email already exists".to_string(),
            });
        }

        if self.user_repository.username_exists(username).await? {
            return Err(VibeStreamError::ValidationError {
                field: "username".to_string(),
                message: "Username already exists".to_string(),
            });
        }

        Ok(())
    }

    /// Validate profile update
    pub fn validate_profile_update(
        &self,
        display_name: &Option<String>,
        bio: &Option<String>,
    ) -> Result<(), VibeStreamError> {
        if let Some(name) = display_name {
            if name.trim().is_empty() {
                return Err(VibeStreamError::ValidationError {
                    field: "display_name".to_string(),
                    message: "Display name cannot be empty".to_string(),
                });
            }
            if name.len() > 100 {
                return Err(VibeStreamError::ValidationError {
                    field: "display_name".to_string(),
                    message: "Display name too long (max 100 characters)".to_string(),
                });
            }
        }

        if let Some(bio_text) = bio {
            if bio_text.len() > 500 {
                return Err(VibeStreamError::ValidationError {
                    field: "bio".to_string(),
                    message: "Bio too long (max 500 characters)".to_string(),
                });
            }
        }

        Ok(())
    }
}

/// Tier upgrade requirements
#[derive(Debug, Clone)]
pub struct TierUpgradeRequirements {
    pub target_tier: UserTier,
    pub points_required: u32,
    pub points_current: u32,
    pub points_needed: u32,
    pub additional_requirements: Vec<String>,
    pub estimated_time_to_unlock: Option<Duration>,
}

/// Achievement definition
#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub points_reward: u32,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub requirements: AchievementRequirements,
}

/// Achievement categories
#[derive(Debug, Clone)]
pub enum AchievementCategory {
    Listening,
    Social,
    Investment,
    Streak,
    Special,
}

/// Achievement requirements
#[derive(Debug, Clone)]
pub struct AchievementRequirements {
    pub min_listening_time: Option<u64>,
    pub min_songs_listened: Option<u64>,
    pub min_streak_days: Option<u32>,
    pub min_investments: Option<u32>,
    pub min_investment_amount: Option<f64>,
    pub min_rewards_earned: Option<f64>,
    pub specific_actions: Vec<String>,
}

/// User activity summary
#[derive(Debug, Clone)]
pub struct UserActivitySummary {
    pub user_id: UserId,
    pub days_since_registration: i64,
    pub total_listening_hours: f64,
    pub favorite_genres: Vec<String>,
    pub listening_streak: u32,
    pub total_investments: f64,
    pub total_rewards: f64,
    pub activity_score: f64,
    pub recent_achievements: Vec<Achievement>,
    pub next_tier_progress: Option<TierUpgradeRequirements>,
}

/// Default authentication service implementation
pub struct DefaultAuthenticationService {
    user_repository: Box<dyn UserRepository>,
    password_service: PasswordDomainService,
}

impl DefaultAuthenticationService {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self {
            user_repository,
            password_service: PasswordDomainService::new(),
        }
    }
}

#[async_trait]
impl AuthenticationDomainService for DefaultAuthenticationService {
    async fn authenticate(
        &self,
        credential: String,
        password: String,
    ) -> Result<UserAggregate, VibeStreamError> {
        // Try to find user by email first, then by username
        let user_aggregate = if credential.contains('@') {
            let email = Email::new(credential)
                .map_err(|e| VibeStreamError::ValidationError {
                    field: "email".to_string(),
                    message: e,
                })?;
            self.user_repository.find_by_email(&email).await?
        } else {
            let username = Username::new(credential)
                .map_err(|e| VibeStreamError::ValidationError {
                    field: "username".to_string(),
                    message: e,
                })?;
            self.user_repository.find_by_username(&username).await?
        };

        let mut user_aggregate = user_aggregate.ok_or(VibeStreamError::NotFound {
            entity: "User".to_string(),
            id: "N/A".to_string(),
        })?;

        // Verify password
        let password_hash = self.hash_password(&password)?;
        user_aggregate.authenticate(&password_hash)?;

        Ok(user_aggregate)
    }

    fn hash_password(&self, password: &str) -> Result<PasswordHash, VibeStreamError> {
        self.password_service.hash_password(password)
    }

    fn verify_password(&self, password: &str, hash: &PasswordHash) -> Result<bool, VibeStreamError> {
        self.password_service.verify_password(password, hash)
    }

    fn validate_password_strength(&self, password: &str) -> Result<(), VibeStreamError> {
        self.password_service.validate_password_strength(password)
    }

    fn generate_secure_token(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect()
    }

    fn validate_secure_token(&self, _token: &str, created_at: DateTime<Utc>) -> Result<bool, VibeStreamError> {
        // Token expires after 24 hours
        let now = Utc::now();
        let expiry = created_at + Duration::hours(24);
        Ok(now <= expiry)
    }
}

/// Default user domain service implementation
pub struct DefaultUserDomainService {
    user_repository: Box<dyn UserRepository>,
    auth_service: DefaultAuthenticationService,
    validation_service: UserValidationService,
}

impl DefaultUserDomainService {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        let auth_service = DefaultAuthenticationService::new(user_repository.clone());
        let validation_service = UserValidationService::new(user_repository.clone());
        
        Self {
            user_repository,
            auth_service,
            validation_service,
        }
    }
}

#[async_trait]
impl UserDomainService for DefaultUserDomainService {
    async fn register_user(
        &self,
        email: Email,
        username: Username,
        password: String,
    ) -> Result<UserAggregate, VibeStreamError> {
        // Validate registration data
        self.validation_service
            .validate_registration(&email, &username, &password)
            .await?;

        // Hash password
        let password_hash = self.auth_service.hash_password(&password)?;

        // Create user aggregate
        let user_aggregate = UserAggregate::create(email, username, password_hash)
            .map_err(|e| VibeStreamError::ValidationError {
                field: "user".to_string(),
                message: e,
            })?;

        Ok(user_aggregate)
    }

    async fn can_upgrade_to_tier(
        &self,
        user_id: &UserId,
        target_tier: &UserTier,
    ) -> Result<bool, VibeStreamError> {
        let user_aggregate = self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(VibeStreamError::NotFound {
                entity: "User".to_string(),
                id: user_id.to_string(),
            })?;

        let required_points = target_tier.points_required();
        let current_points = user_aggregate.stats.tier_points;

        Ok(current_points >= required_points)
    }

    async fn get_tier_upgrade_requirements(
        &self,
        user_id: &UserId,
        target_tier: &UserTier,
    ) -> Result<TierUpgradeRequirements, VibeStreamError> {
        let user_aggregate = self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(VibeStreamError::NotFound {
                entity: "User".to_string(),
                id: user_id.to_string(),
            })?;

        let required_points = target_tier.points_required();
        let current_points = user_aggregate.stats.tier_points;
        let points_needed = required_points.saturating_sub(current_points);

        Ok(TierUpgradeRequirements {
            target_tier: target_tier.clone(),
            points_required: required_points,
            points_current: current_points,
            points_needed,
            additional_requirements: vec![],
            estimated_time_to_unlock: None,
        })
    }

    async fn check_achievements(&self, _user_id: &UserId) -> Result<Vec<Achievement>, VibeStreamError> {
        // TODO: Implement achievement checking logic
        Ok(vec![])
    }

    async fn calculate_reputation_score(&self, user_id: &UserId) -> Result<f64, VibeStreamError> {
        let user_aggregate = self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(VibeStreamError::NotFound {
                entity: "User".to_string(),
                id: user_id.to_string(),
            })?;

        // Simple reputation calculation based on various factors
        let listening_score = (user_aggregate.stats.total_listening_time_minutes as f64 / 60.0) * 0.1;
        let investment_score = user_aggregate.stats.total_investments * 0.5;
        let reward_score = user_aggregate.stats.total_rewards_earned * 0.2;
        let streak_score = user_aggregate.stats.longest_listening_streak as f64 * 0.3;

        let reputation = listening_score + investment_score + reward_score + streak_score;
        Ok(reputation.min(100.0)) // Cap at 100
    }

    async fn check_feature_eligibility(
        &self,
        user_id: &UserId,
        feature: &str,
    ) -> Result<bool, VibeStreamError> {
        let user_aggregate = self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(VibeStreamError::NotFound {
                entity: "User".to_string(),
                id: user_id.to_string(),
            })?;

        Ok(user_aggregate.can_access_feature(feature))
    }

    async fn get_activity_summary(&self, user_id: &UserId) -> Result<UserActivitySummary, VibeStreamError> {
        let user_aggregate = self.user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(VibeStreamError::NotFound {
                entity: "User".to_string(),
                id: user_id.to_string(),
            })?;

        let days_since_registration = (Utc::now() - user_aggregate.user.created_at).num_days();
        let total_listening_hours = user_aggregate.stats.total_listening_time_minutes as f64 / 60.0;

        Ok(UserActivitySummary {
            user_id: user_aggregate.user.id.clone(),
            days_since_registration,
            total_listening_hours,
            favorite_genres: vec![], // TODO: Implement based on listening history
            listening_streak: user_aggregate.stats.current_listening_streak,
            total_investments: user_aggregate.stats.total_investments,
            total_rewards: user_aggregate.stats.total_rewards_earned,
            activity_score: 0.0, // TODO: Calculate based on various factors
            recent_achievements: vec![], // TODO: Get recent achievements
            next_tier_progress: None, // TODO: Calculate next tier progress
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_service() {
        let service = PasswordDomainService::new();

        // Test password hashing and verification
        let password = "secure_password123!";
        let hash = service.hash_password(password).unwrap();
        
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_password_strength_validation() {
        let service = PasswordDomainService::new();

        // Valid password
        assert!(service.validate_password_strength("SecurePass123!").is_ok());

        // Invalid passwords
        assert!(service.validate_password_strength("weak").is_err());
        assert!(service.validate_password_strength("123456").is_err());
        assert!(service.validate_password_strength("password").is_err());
    }
} 