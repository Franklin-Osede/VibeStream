use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::value_objects::{
    UserId, Email, Username, PasswordHash, WalletAddress, 
    UserTier, UserRole, ProfileUrl
};

/// Core User entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub wallet_address: Option<WalletAddress>,
    pub tier: UserTier,
    pub role: UserRole,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        email: Email,
        username: Username,
        password_hash: PasswordHash,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            username,
            password_hash,
            wallet_address: None,
            tier: UserTier::Free,
            role: UserRole::User,
            is_verified: false,
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login_at: None,
        }
    }

    pub fn with_id(
        id: UserId,
        email: Email,
        username: Username,
        password_hash: PasswordHash,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            email,
            username,
            password_hash,
            wallet_address: None,
            tier: UserTier::Free,
            role: UserRole::User,
            is_verified: false,
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login_at: None,
        }
    }

    /// Update user password
    pub fn update_password(&mut self, new_password_hash: PasswordHash) {
        self.password_hash = new_password_hash;
        self.updated_at = Utc::now();
    }

    /// Link wallet address
    pub fn link_wallet(&mut self, wallet_address: WalletAddress) {
        self.wallet_address = Some(wallet_address);
        self.updated_at = Utc::now();
    }

    /// Remove wallet address
    pub fn unlink_wallet(&mut self) {
        self.wallet_address = None;
        self.updated_at = Utc::now();
    }

    /// Verify user email
    pub fn verify_email(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }

    /// Upgrade user tier
    pub fn upgrade_tier(&mut self, new_tier: UserTier) {
        self.tier = new_tier;
        self.updated_at = Utc::now();
    }

    /// Change user role
    pub fn change_role(&mut self, new_role: UserRole) {
        self.role = new_role;
        self.updated_at = Utc::now();
    }

    /// Deactivate user
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Reactivate user
    pub fn reactivate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Record successful login
    pub fn record_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Check if user has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.is_active && self.role.can(permission)
    }

    /// Check if user can access feature
    pub fn can_access_feature(&self, feature: &str) -> bool {
        self.is_active && self.tier.features().contains(&feature)
    }
}

/// User profile information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: UserId,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<ProfileUrl>,
    pub cover_url: Option<ProfileUrl>,
    pub location: Option<String>,
    pub website: Option<ProfileUrl>,
    pub social_links: HashMap<String, String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            display_name: None,
            bio: None,
            avatar_url: None,
            cover_url: None,
            location: None,
            website: None,
            social_links: HashMap::new(),
            is_public: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update display name
    pub fn update_display_name(&mut self, display_name: Option<String>) {
        self.display_name = display_name;
        self.updated_at = Utc::now();
    }

    /// Update bio
    pub fn update_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
        self.updated_at = Utc::now();
    }

    /// Update avatar
    pub fn update_avatar(&mut self, avatar_url: Option<ProfileUrl>) {
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }

    /// Update cover image
    pub fn update_cover(&mut self, cover_url: Option<ProfileUrl>) {
        self.cover_url = cover_url;
        self.updated_at = Utc::now();
    }

    /// Update location
    pub fn update_location(&mut self, location: Option<String>) {
        self.location = location;
        self.updated_at = Utc::now();
    }

    /// Update website
    pub fn update_website(&mut self, website: Option<ProfileUrl>) {
        self.website = website;
        self.updated_at = Utc::now();
    }

    /// Add or update social link
    pub fn add_social_link(&mut self, platform: String, url: String) {
        self.social_links.insert(platform, url);
        self.updated_at = Utc::now();
    }

    /// Remove social link
    pub fn remove_social_link(&mut self, platform: &str) {
        self.social_links.remove(platform);
        self.updated_at = Utc::now();
    }

    /// Set profile visibility
    pub fn set_visibility(&mut self, is_public: bool) {
        self.is_public = is_public;
        self.updated_at = Utc::now();
    }
}

/// User preferences and settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPreferences {
    pub user_id: UserId,
    pub language: String,
    pub timezone: String,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub marketing_emails: bool,
    pub privacy_settings: PrivacySettings,
    pub music_preferences: MusicPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserPreferences {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            email_notifications: true,
            push_notifications: true,
            marketing_emails: false,
            privacy_settings: PrivacySettings::default(),
            music_preferences: MusicPreferences::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update language preference
    pub fn update_language(&mut self, language: String) {
        self.language = language;
        self.updated_at = Utc::now();
    }

    /// Update timezone
    pub fn update_timezone(&mut self, timezone: String) {
        self.timezone = timezone;
        self.updated_at = Utc::now();
    }

    /// Update notification preferences
    pub fn update_notifications(
        &mut self,
        email: Option<bool>,
        push: Option<bool>,
        marketing: Option<bool>,
    ) {
        if let Some(email) = email {
            self.email_notifications = email;
        }
        if let Some(push) = push {
            self.push_notifications = push;
        }
        if let Some(marketing) = marketing {
            self.marketing_emails = marketing;
        }
        self.updated_at = Utc::now();
    }

    /// Update privacy settings
    pub fn update_privacy(&mut self, privacy_settings: PrivacySettings) {
        self.privacy_settings = privacy_settings;
        self.updated_at = Utc::now();
    }

    /// Update music preferences
    pub fn update_music_preferences(&mut self, music_preferences: MusicPreferences) {
        self.music_preferences = music_preferences;
        self.updated_at = Utc::now();
    }
}

/// Privacy settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub profile_visibility: ProfileVisibility,
    pub show_listening_activity: bool,
    pub show_investment_activity: bool,
    pub allow_direct_messages: bool,
    pub show_online_status: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            profile_visibility: ProfileVisibility::Public,
            show_listening_activity: true,
            show_investment_activity: false,
            allow_direct_messages: true,
            show_online_status: true,
        }
    }
}

/// Profile visibility options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProfileVisibility {
    Public,
    FriendsOnly,
    Private,
}

/// Music preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicPreferences {
    pub favorite_genres: Vec<String>,
    pub preferred_audio_quality: AudioQuality,
    pub auto_play: bool,
    pub repeat_mode: RepeatMode,
    pub explicit_content: bool,
}

impl Default for MusicPreferences {
    fn default() -> Self {
        Self {
            favorite_genres: Vec::new(),
            preferred_audio_quality: AudioQuality::High,
            auto_play: true,
            repeat_mode: RepeatMode::Off,
            explicit_content: false,
        }
    }
}

/// Audio quality preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioQuality {
    Low,
    Medium,
    High,
    Lossless,
}

/// Repeat mode options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RepeatMode {
    Off,
    RepeatOne,
    RepeatAll,
}

/// User statistics and metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: UserId,
    pub total_listening_time_minutes: u64,
    pub total_songs_listened: u64,
    pub total_rewards_earned: f64,
    pub current_listening_streak: u32,
    pub longest_listening_streak: u32,
    pub total_investments: f64,
    pub investment_count: u32,
    pub nfts_owned: u32,
    pub campaigns_participated: u32,
    pub tier_points: u32,
    pub achievements_unlocked: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserStats {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            total_listening_time_minutes: 0,
            total_songs_listened: 0,
            total_rewards_earned: 0.0,
            current_listening_streak: 0,
            longest_listening_streak: 0,
            total_investments: 0.0,
            investment_count: 0,
            nfts_owned: 0,
            campaigns_participated: 0,
            tier_points: 0,
            achievements_unlocked: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add listening time
    pub fn add_listening_time(&mut self, minutes: u64) {
        self.total_listening_time_minutes += minutes;
        self.total_songs_listened += 1;
        self.current_listening_streak += 1;
        
        if self.current_listening_streak > self.longest_listening_streak {
            self.longest_listening_streak = self.current_listening_streak;
        }
        
        self.updated_at = Utc::now();
    }

    /// Break listening streak
    pub fn break_listening_streak(&mut self) {
        self.current_listening_streak = 0;
        self.updated_at = Utc::now();
    }

    /// Add rewards earned
    pub fn add_rewards(&mut self, amount: f64) {
        self.total_rewards_earned += amount;
        self.updated_at = Utc::now();
    }

    /// Add investment
    pub fn add_investment(&mut self, amount: f64) {
        self.total_investments += amount;
        self.investment_count += 1;
        self.updated_at = Utc::now();
    }

    /// Add NFT
    pub fn add_nft(&mut self) {
        self.nfts_owned += 1;
        self.updated_at = Utc::now();
    }

    /// Add campaign participation
    pub fn add_campaign_participation(&mut self) {
        self.campaigns_participated += 1;
        self.updated_at = Utc::now();
    }

    /// Add tier points
    pub fn add_tier_points(&mut self, points: u32) {
        self.tier_points += points;
        self.updated_at = Utc::now();
    }

    /// Unlock achievement
    pub fn unlock_achievement(&mut self, achievement: String) {
        if !self.achievements_unlocked.contains(&achievement) {
            self.achievements_unlocked.push(achievement);
            self.updated_at = Utc::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::user::domain::value_objects::{Email, Username};

    #[test]
    fn test_user_creation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());

        let user = User::new(email.clone(), username.clone(), password_hash);

        assert_eq!(user.email, email);
        assert_eq!(user.username, username);
        assert_eq!(user.tier, UserTier::Free);
        assert_eq!(user.role, UserRole::User);
        assert!(!user.is_verified);
        assert!(user.is_active);
    }

    #[test]
    fn test_user_permissions() {
        let email = Email::new("artist@example.com".to_string()).unwrap();
        let username = Username::new("artist".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());

        let mut user = User::new(email, username, password_hash);
        user.change_role(UserRole::Artist);

        assert!(user.has_permission("upload_music"));
        assert!(!user.has_permission("moderate_content"));
    }

    #[test]
    fn test_user_profile_updates() {
        let user_id = UserId::new();
        let mut profile = UserProfile::new(user_id);

        assert!(profile.display_name.is_none());
        
        profile.update_display_name(Some("Test User".to_string()));
        assert_eq!(profile.display_name, Some("Test User".to_string()));

        profile.add_social_link("twitter".to_string(), "@testuser".to_string());
        assert_eq!(profile.social_links.get("twitter"), Some(&"@testuser".to_string()));
    }

    #[test]
    fn test_user_stats_updates() {
        let user_id = UserId::new();
        let mut stats = UserStats::new(user_id);

        assert_eq!(stats.total_listening_time_minutes, 0);
        assert_eq!(stats.current_listening_streak, 0);

        stats.add_listening_time(30);
        assert_eq!(stats.total_listening_time_minutes, 30);
        assert_eq!(stats.current_listening_streak, 1);
        assert_eq!(stats.total_songs_listened, 1);

        stats.add_rewards(10.5);
        assert_eq!(stats.total_rewards_earned, 10.5);
    }
} 