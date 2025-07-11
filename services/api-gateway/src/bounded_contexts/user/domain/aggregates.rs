use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

use super::{
    entities::{User, UserProfile, UserPreferences, UserStats},
    value_objects::{
        UserId, Email, Username, PasswordHash, WalletAddress, 
        UserTier, UserRole, ProfileUrl
    },
    events::{
        UserRegistered, UserAuthenticated, UserProfileUpdated,
        UserTierUpgraded, UserDeactivated, UserReactivated, 
        UserWalletLinked, UserWalletUnlinked, UserEmailVerified
    },
};

use crate::shared::domain::events::DomainEvent;

/// User Aggregate - Main aggregate root for user domain
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAggregate {
    pub user: User,
    pub profile: UserProfile,
    pub preferences: UserPreferences,
    pub stats: UserStats,
    
    // Domain events
    #[serde(skip)]
    pending_events: VecDeque<Box<dyn DomainEvent>>,
    version: u64,
}

impl UserAggregate {
    /// Create new user aggregate
    pub fn create(
        email: Email,
        username: Username,
        password_hash: PasswordHash,
    ) -> Result<Self, String> {
        let user = User::new(email.clone(), username.clone(), password_hash);
        let user_id = user.id.clone();
        
        let profile = UserProfile::new(user_id.clone());
        let preferences = UserPreferences::new(user_id.clone());
        let stats = UserStats::new(user_id.clone());

        let mut aggregate = Self {
            user,
            profile,
            preferences,
            stats,
            pending_events: VecDeque::new(),
            version: 1,
        };

        // Record domain event
        let event = UserRegistered::new(
            user_id,
            email,
            username,
            Utc::now(),
        );
        aggregate.add_event(Box::new(event));

        Ok(aggregate)
    }

    /// Load existing user aggregate
    pub fn load(
        user: User,
        profile: UserProfile,
        preferences: UserPreferences,
        stats: UserStats,
        version: u64,
    ) -> Self {
        Self {
            user,
            profile,
            preferences,
            stats,
            pending_events: VecDeque::new(),
            version,
        }
    }

    /// Authenticate user with password
    pub fn authenticate(&mut self, provided_password_hash: &PasswordHash) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        if self.user.password_hash != *provided_password_hash {
            return Err("Contraseña incorrecta".to_string());
        }

        // Record login
        self.user.record_login();
        self.increment_version();

        // Record domain event
        let event = UserAuthenticated::new(
            self.user.id.clone(),
            self.user.last_login_at.unwrap(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Update user password
    pub fn update_password(&mut self, new_password_hash: PasswordHash) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        self.user.update_password(new_password_hash);
        self.increment_version();

        Ok(())
    }

    /// Link wallet address
    pub fn link_wallet(&mut self, wallet_address: WalletAddress) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        self.user.link_wallet(wallet_address.clone());
        self.increment_version();

        // Record domain event
        let event = UserWalletLinked::new(
            self.user.id.clone(),
            wallet_address,
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Unlink wallet address
    pub fn unlink_wallet(&mut self) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        if self.user.wallet_address.is_none() {
            return Err("No hay wallet vinculada".to_string());
        }

        self.user.unlink_wallet();
        self.increment_version();

        // Record domain event
        let event = UserWalletUnlinked::new(
            self.user.id.clone(),
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Verify user email
    pub fn verify_email(&mut self) -> Result<(), String> {
        if self.user.is_verified {
            return Err("Email ya está verificado".to_string());
        }

        self.user.verify_email();
        self.increment_version();

        // Record domain event
        let event = UserEmailVerified::new(
            self.user.id.clone(),
            self.user.email.clone(),
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Upgrade user tier
    pub fn upgrade_tier(&mut self, new_tier: UserTier) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        let old_tier = self.user.tier.clone();
        
        // Check if upgrade is valid
        let required_points = new_tier.points_required();
        if required_points > 0 && self.stats.tier_points < required_points {
            return Err(format!(
                "Puntos insuficientes. Requeridos: {}, Actuales: {}",
                required_points, self.stats.tier_points
            ));
        }

        self.user.upgrade_tier(new_tier.clone());
        self.increment_version();

        // Record domain event
        let event = UserTierUpgraded::new(
            self.user.id.clone(),
            old_tier,
            new_tier,
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Change user role (admin operation)
    pub fn change_role(&mut self, new_role: UserRole) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        self.user.change_role(new_role);
        self.increment_version();

        Ok(())
    }

    /// Update profile information
    pub fn update_profile(
        &mut self,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<ProfileUrl>,
        location: Option<String>,
        website: Option<ProfileUrl>,
    ) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        // Update profile fields
        self.profile.update_display_name(display_name.clone());
        self.profile.update_bio(bio.clone());
        self.profile.update_avatar(avatar_url.clone());
        self.profile.update_location(location.clone());
        self.profile.update_website(website.clone());
        
        self.increment_version();

        // Record domain event
        let event = UserProfileUpdated::new(
            self.user.id.clone(),
            display_name,
            bio,
            avatar_url,
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Add listening session to stats
    pub fn add_listening_session(&mut self, duration_minutes: u64) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        self.stats.add_listening_time(duration_minutes);
        self.increment_version();

        Ok(())
    }

    /// Add rewards earned
    pub fn add_rewards(&mut self, amount: f64) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        if amount <= 0.0 {
            return Err("Cantidad de rewards debe ser positiva".to_string());
        }

        self.stats.add_rewards(amount);
        self.increment_version();

        Ok(())
    }

    /// Add investment
    pub fn add_investment(&mut self, amount: f64) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        if amount <= 0.0 {
            return Err("Cantidad de inversión debe ser positiva".to_string());
        }

        self.stats.add_investment(amount);
        self.increment_version();

        Ok(())
    }

    /// Add NFT to collection
    pub fn add_nft(&mut self) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        self.stats.add_nft();
        self.increment_version();

        Ok(())
    }

    /// Add tier points
    pub fn add_tier_points(&mut self, points: u32) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario desactivado".to_string());
        }

        self.stats.add_tier_points(points);
        self.increment_version();

        // Check for automatic tier upgrades
        self.check_automatic_tier_upgrade()?;

        Ok(())
    }

    /// Deactivate user
    pub fn deactivate(&mut self, reason: String) -> Result<(), String> {
        if !self.user.is_active {
            return Err("Usuario ya está desactivado".to_string());
        }

        self.user.deactivate();
        self.increment_version();

        // Record domain event
        let event = UserDeactivated::new(
            self.user.id.clone(),
            reason,
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Reactivate user
    pub fn reactivate(&mut self) -> Result<(), String> {
        if self.user.is_active {
            return Err("Usuario ya está activo".to_string());
        }

        self.user.reactivate();
        self.increment_version();

        // Record domain event
        let event = UserReactivated::new(
            self.user.id.clone(),
            Utc::now(),
        );
        self.add_event(Box::new(event));

        Ok(())
    }

    /// Check for automatic tier upgrades based on points
    fn check_automatic_tier_upgrade(&mut self) -> Result<(), String> {
        let current_tier = &self.user.tier;
        let points = self.stats.tier_points;

        let new_tier = match current_tier {
            UserTier::Free if points >= UserTier::Premium.points_required() => {
                Some(UserTier::Premium)
            }
            UserTier::Premium if points >= UserTier::Vip.points_required() => {
                Some(UserTier::Vip)
            }
            _ => None,
        };

        if let Some(tier) = new_tier {
            self.upgrade_tier(tier)?;
        }

        Ok(())
    }

    /// Get aggregate ID
    pub fn id(&self) -> &UserId {
        &self.user.id
    }

    /// Get aggregate version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Increment version
    fn increment_version(&mut self) {
        self.version += 1;
    }

    /// Add domain event
    fn add_event(&mut self, event: Box<dyn DomainEvent>) {
        self.pending_events.push_back(event);
    }

    /// Get and clear pending events
    pub fn take_events(&mut self) -> VecDeque<Box<dyn DomainEvent>> {
        std::mem::take(&mut self.pending_events)
    }

    /// Check if user has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.user.has_permission(permission)
    }

    /// Check if user can access feature
    pub fn can_access_feature(&self, feature: &str) -> bool {
        self.user.can_access_feature(feature)
    }

    /// Get user summary for external consumption
    pub fn get_summary(&self) -> UserSummary {
        UserSummary {
            id: self.user.id.clone(),
            username: self.user.username.clone(),
            email: self.user.email.clone(),
            tier: self.user.tier.to_string(),
            role: self.user.role.to_string(),
            is_verified: self.user.is_verified,
            is_active: self.user.is_active,
            display_name: self.profile.display_name.clone(),
            avatar_url: self.profile.avatar_url.as_ref().map(|u| u.to_string()),
            total_listening_time: self.stats.total_listening_time_minutes as i64,
            total_rewards: self.stats.total_rewards_earned,
            tier_points: self.stats.tier_points as i32,
            created_at: self.user.created_at,
        }
    }
}

impl Clone for UserAggregate {
    fn clone(&self) -> Self {
        Self {
            user: self.user.clone(),
            profile: self.profile.clone(),
            preferences: self.preferences.clone(),
            stats: self.stats.clone(),
            pending_events: VecDeque::new(), // Don't clone events
            version: self.version,
        }
    }
}

/// User summary for external consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub tier: String,
    pub role: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub total_listening_time: i64,
    pub total_rewards: f64,
    pub tier_points: i32,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::user::domain::value_objects::*;
    use chrono::Utc;

    #[test]
    fn test_create_user_aggregate_success() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());

        // Act
        let result = UserAggregate::create(email.clone(), username.clone(), password_hash);

        // Assert
        assert!(result.is_ok());
        let user_aggregate = result.unwrap();
        assert_eq!(user_aggregate.user.email, email);
        assert_eq!(user_aggregate.user.username, username);
        assert_eq!(user_aggregate.user.tier, UserTier::Free);
        assert_eq!(user_aggregate.user.role, UserRole::User);
        assert!(!user_aggregate.user.is_verified);
        assert!(user_aggregate.user.is_active);
        assert_eq!(user_aggregate.version, 1);
    }

    #[test]
    fn test_authenticate_user_success() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash.clone()).unwrap();

        // Act
        let result = user_aggregate.authenticate(&password_hash);

        // Assert
        assert!(result.is_ok());
        assert!(user_aggregate.user.last_login_at.is_some());
        assert_eq!(user_aggregate.version, 2);
    }

    #[test]
    fn test_authenticate_user_wrong_password() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash).unwrap();
        let wrong_password = PasswordHash::new("wrong_password".to_string());

        // Act
        let result = user_aggregate.authenticate(&wrong_password);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Contraseña incorrecta");
    }

    #[test]
    fn test_authenticate_inactive_user() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash.clone()).unwrap();
        user_aggregate.deactivate("Test reason".to_string()).unwrap();

        // Act
        let result = user_aggregate.authenticate(&password_hash);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Usuario desactivado");
    }

    #[test]
    fn test_upgrade_tier_success() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash).unwrap();

        // Act
        let result = user_aggregate.upgrade_tier(UserTier::Premium);

        // Assert
        assert!(result.is_ok());
        assert_eq!(user_aggregate.user.tier, UserTier::Premium);
    }

    #[test]
    fn test_link_wallet_success() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash).unwrap();
        let wallet_address = WalletAddress::new("0x123456789abcdef".to_string()).unwrap();

        // Act
        let result = user_aggregate.link_wallet(wallet_address.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(user_aggregate.user.wallet_address, Some(wallet_address));
    }

    #[test]
    fn test_verify_email_success() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash).unwrap();

        // Act
        let result = user_aggregate.verify_email();

        // Assert
        assert!(result.is_ok());
        assert!(user_aggregate.user.is_verified);
    }

    #[test]
    fn test_deactivate_user_success() {
        // Arrange
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let mut user_aggregate = UserAggregate::create(email, username, password_hash).unwrap();

        // Act
        let result = user_aggregate.deactivate("Test reason".to_string());

        // Assert
        assert!(result.is_ok());
        assert!(!user_aggregate.user.is_active);
    }
} 