use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// DTO for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub display_name: Option<String>,
    pub terms_accepted: bool,
    pub marketing_emails_consent: Option<bool>,
}

impl CreateUserDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err("Email es requerido".to_string());
        }

        if self.username.trim().is_empty() {
            return Err("Username es requerido".to_string());
        }

        if self.password.is_empty() {
            return Err("Password es requerido".to_string());
        }

        if self.password != self.confirm_password {
            return Err("Las contraseñas no coinciden".to_string());
        }

        if !self.terms_accepted {
            return Err("Debe aceptar los términos y condiciones".to_string());
        }

        Ok(())
    }
}

/// DTO for user login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginDto {
    pub credential: String, // email or username
    pub password: String,
    pub remember_me: Option<bool>,
}

impl LoginDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.credential.trim().is_empty() {
            return Err("Email o username es requerido".to_string());
        }

        if self.password.is_empty() {
            return Err("Password es requerido".to_string());
        }

        Ok(())
    }
}

/// DTO for updating user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub is_active: Option<bool>,
    pub tier: Option<String>,
    pub role: Option<String>,
}

/// DTO for updating user profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileDto {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<HashMap<String, String>>,
    pub is_public: Option<bool>,
}

/// DTO for changing password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordDto {
    pub current_password: String,
    pub new_password: String,
    pub confirm_new_password: String,
}

impl ChangePasswordDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.current_password.is_empty() {
            return Err("Contraseña actual es requerida".to_string());
        }

        if self.new_password.is_empty() {
            return Err("Nueva contraseña es requerida".to_string());
        }

        if self.new_password != self.confirm_new_password {
            return Err("Las nuevas contraseñas no coinciden".to_string());
        }

        if self.current_password == self.new_password {
            return Err("La nueva contraseña debe ser diferente a la actual".to_string());
        }

        Ok(())
    }
}

/// DTO for linking wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkWalletDto {
    pub wallet_address: String,
    pub signature: Option<String>, // For verification
    pub message: Option<String>,   // Message that was signed
}

impl LinkWalletDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.wallet_address.trim().is_empty() {
            return Err("Wallet address es requerida".to_string());
        }

        Ok(())
    }
}

/// DTO for user response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub tier: String,
    pub role: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

/// DTO for user profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileDto {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: HashMap<String, String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO for user statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsDto {
    pub user_id: Uuid,
    pub total_listening_time_minutes: u64,
    pub total_listening_hours: f64,
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

impl UserStatsDto {
    pub fn calculate_listening_hours(&mut self) {
        self.total_listening_hours = self.total_listening_time_minutes as f64 / 60.0;
    }
}

/// DTO for user preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferencesDto {
    pub user_id: Uuid,
    pub language: String,
    pub timezone: String,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub marketing_emails: bool,
    pub privacy_settings: PrivacySettingsDto,
    pub music_preferences: MusicPreferencesDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO for privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettingsDto {
    pub profile_visibility: String,
    pub show_listening_activity: bool,
    pub show_investment_activity: bool,
    pub allow_direct_messages: bool,
    pub show_online_status: bool,
}

/// DTO for music preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicPreferencesDto {
    pub favorite_genres: Vec<String>,
    pub preferred_audio_quality: String,
    pub auto_play: bool,
    pub repeat_mode: String,
    pub explicit_content: bool,
}

/// DTO for user list response (with pagination)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub users: Vec<UserSummaryDto>,
    pub pagination: PaginationDto,
}

/// DTO for user summary (lighter version for lists)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummaryDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub tier: String,
    pub role: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub tier_points: u32,
    pub total_rewards: f64,
    pub total_listening_time: u64,
    pub created_at: DateTime<Utc>,
}

/// DTO for pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationDto {
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

/// DTO for user search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchDto {
    pub username_contains: Option<String>,
    pub email_contains: Option<String>,
    pub display_name_contains: Option<String>,
    pub tier: Option<String>,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub has_wallet: Option<bool>,
    pub min_tier_points: Option<u32>,
    pub max_tier_points: Option<u32>,
    pub min_rewards: Option<f64>,
    pub max_rewards: Option<f64>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Default for UserSearchDto {
    fn default() -> Self {
        Self {
            username_contains: None,
            email_contains: None,
            display_name_contains: None,
            tier: None,
            role: None,
            is_verified: None,
            is_active: None,
            has_wallet: None,
            min_tier_points: None,
            max_tier_points: None,
            min_rewards: None,
            max_rewards: None,
            created_after: None,
            created_before: None,
            page: Some(0),
            page_size: Some(20),
            sort_by: None,
            sort_order: None,
        }
    }
}

/// DTO for authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponseDto {
    pub user: UserResponseDto,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64, // seconds
    pub token_type: String,
}

/// DTO for tier upgrade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierUpgradeDto {
    pub target_tier: String,
    pub points_required: u32,
    pub points_current: u32,
    pub points_needed: u32,
    pub additional_requirements: Vec<String>,
    pub can_upgrade: bool,
}

/// DTO for achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub points_reward: u32,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub is_unlocked: bool,
    pub progress_percentage: Option<f64>,
}

/// DTO for user activity summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivitySummaryDto {
    pub user_id: Uuid,
    pub days_since_registration: i64,
    pub total_listening_hours: f64,
    pub favorite_genres: Vec<String>,
    pub listening_streak: u32,
    pub total_investments: f64,
    pub total_rewards: f64,
    pub activity_score: f64,
    pub recent_achievements: Vec<AchievementDto>,
    pub next_tier_progress: Option<TierUpgradeDto>,
}

/// DTO for email verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerificationDto {
    pub token: String,
    pub user_id: Uuid,
}

/// DTO for password reset request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequestDto {
    pub email: String,
}

/// DTO for password reset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetDto {
    pub token: String,
    pub new_password: String,
    pub confirm_new_password: String,
}

impl PasswordResetDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.token.trim().is_empty() {
            return Err("Token es requerido".to_string());
        }

        if self.new_password.is_empty() {
            return Err("Nueva contraseña es requerida".to_string());
        }

        if self.new_password != self.confirm_new_password {
            return Err("Las contraseñas no coinciden".to_string());
        }

        Ok(())
    }
}

/// DTO for user analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnalyticsDto {
    pub total_users: u64,
    pub active_users: u64,
    pub verified_users: u64,
    pub users_with_wallets: u64,
    pub tier_distribution: HashMap<String, u64>,
    pub role_distribution: HashMap<String, u64>,
    pub registration_stats: Vec<RegistrationStatDto>,
    pub activity_stats: Vec<ActivityStatDto>,
}

/// DTO for registration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationStatDto {
    pub period: DateTime<Utc>,
    pub count: u64,
    pub verified_count: u64,
}

/// DTO for activity statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityStatDto {
    pub period: DateTime<Utc>,
    pub active_users: u64,
    pub new_users: u64,
    pub total_listening_time: u64,
    pub total_rewards_earned: f64,
}

/// DTO for bulk user operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUserOperationDto {
    pub user_ids: Vec<Uuid>,
    pub operation: String, // "activate", "deactivate", "upgrade_tier", etc.
    pub parameters: HashMap<String, serde_json::Value>,
}

/// DTO for user export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExportDto {
    pub format: String, // "csv", "json", "xlsx"
    pub filters: UserSearchDto,
    pub fields: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_dto_validation() {
        let mut dto = CreateUserDto {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
            display_name: None,
            terms_accepted: true,
            marketing_emails_consent: None,
        };

        assert!(dto.validate().is_ok());

        // Test password mismatch
        dto.confirm_password = "different".to_string();
        assert!(dto.validate().is_err());

        // Test terms not accepted
        dto.confirm_password = "password123".to_string();
        dto.terms_accepted = false;
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_login_dto_validation() {
        let dto = LoginDto {
            credential: "test@example.com".to_string(),
            password: "password123".to_string(),
            remember_me: Some(true),
        };

        assert!(dto.validate().is_ok());

        let empty_credential_dto = LoginDto {
            credential: "".to_string(),
            password: "password123".to_string(),
            remember_me: None,
        };

        assert!(empty_credential_dto.validate().is_err());
    }

    #[test]
    fn test_change_password_dto_validation() {
        let dto = ChangePasswordDto {
            current_password: "oldpassword".to_string(),
            new_password: "newpassword123".to_string(),
            confirm_new_password: "newpassword123".to_string(),
        };

        assert!(dto.validate().is_ok());

        // Test same password
        let same_password_dto = ChangePasswordDto {
            current_password: "password".to_string(),
            new_password: "password".to_string(),
            confirm_new_password: "password".to_string(),
        };

        assert!(same_password_dto.validate().is_err());
    }

    #[test]
    fn test_user_stats_dto_calculate_hours() {
        let mut stats = UserStatsDto {
            user_id: Uuid::new_v4(),
            total_listening_time_minutes: 120,
            total_listening_hours: 0.0,
            total_songs_listened: 10,
            total_rewards_earned: 50.0,
            current_listening_streak: 5,
            longest_listening_streak: 10,
            total_investments: 100.0,
            investment_count: 2,
            nfts_owned: 3,
            campaigns_participated: 1,
            tier_points: 250,
            achievements_unlocked: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        stats.calculate_listening_hours();
        assert_eq!(stats.total_listening_hours, 2.0);
    }
} 