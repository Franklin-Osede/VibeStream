use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

use crate::shared::application::commands::Command;

/// Create user command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub marketing_emails_consent: bool,
}

impl Command for CreateUserCommand {
    fn command_type(&self) -> &'static str {
        "CreateUser"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        None // Will be generated during handling
    }
}

/// Update user command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub username: Option<String>,
    pub is_active: Option<bool>,
    pub tier: Option<String>,
    pub role: Option<String>,
}

impl Command for UpdateUserCommand {
    fn command_type(&self) -> &'static str {
        "UpdateUser"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Delete user command (soft delete)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserCommand {
    pub user_id: Uuid,
    pub reason: Option<String>,
}

impl Command for DeleteUserCommand {
    fn command_type(&self) -> &'static str {
        "DeleteUser"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Change password command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordCommand {
    pub user_id: Uuid,
    pub current_password: String,
    pub new_password: String,
}

impl Command for ChangePasswordCommand {
    fn command_type(&self) -> &'static str {
        "ChangePassword"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Link wallet command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkWalletCommand {
    pub user_id: Uuid,
    pub wallet_address: String,
    pub signature: Option<String>,
    pub message: Option<String>,
}

impl Command for LinkWalletCommand {
    fn command_type(&self) -> &'static str {
        "LinkWallet"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Unlink wallet command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlinkWalletCommand {
    pub user_id: Uuid,
}

impl Command for UnlinkWalletCommand {
    fn command_type(&self) -> &'static str {
        "UnlinkWallet"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Verify email command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailCommand {
    pub user_id: Uuid,
    pub verification_token: String,
}

impl Command for VerifyEmailCommand {
    fn command_type(&self) -> &'static str {
        "VerifyEmail"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Upgrade tier command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeTierCommand {
    pub user_id: Uuid,
    pub target_tier: String,
}

impl Command for UpgradeTierCommand {
    fn command_type(&self) -> &'static str {
        "UpgradeTier"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Update profile command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileCommand {
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<HashMap<String, String>>,
    pub is_public: Option<bool>,
}

impl Command for UpdateProfileCommand {
    fn command_type(&self) -> &'static str {
        "UpdateProfile"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Update preferences command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePreferencesCommand {
    pub user_id: Uuid,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub marketing_emails: Option<bool>,
    pub privacy_settings: Option<UpdatePrivacySettingsDto>,
    pub music_preferences: Option<UpdateMusicPreferencesDto>,
}

impl Command for UpdatePreferencesCommand {
    fn command_type(&self) -> &'static str {
        "UpdatePreferences"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Deactivate user command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeactivateUserCommand {
    pub user_id: Uuid,
    pub reason: String,
    pub deactivated_by: Uuid, // Admin user ID
}

impl Command for DeactivateUserCommand {
    fn command_type(&self) -> &'static str {
        "DeactivateUser"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Reactivate user command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactivateUserCommand {
    pub user_id: Uuid,
    pub reactivated_by: Uuid, // Admin user ID
}

impl Command for ReactivateUserCommand {
    fn command_type(&self) -> &'static str {
        "ReactivateUser"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Add listening session command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddListeningSessionCommand {
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub duration_minutes: u64,
    pub completed: bool,
}

impl Command for AddListeningSessionCommand {
    fn command_type(&self) -> &'static str {
        "AddListeningSession"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Add rewards command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRewardsCommand {
    pub user_id: Uuid,
    pub amount: f64,
    pub reason: String,
    pub transaction_id: Option<String>,
}

impl Command for AddRewardsCommand {
    fn command_type(&self) -> &'static str {
        "AddRewards"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Add investment command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddInvestmentCommand {
    pub user_id: Uuid,
    pub amount: f64,
    pub investment_type: String,
    pub target_id: Uuid, // Artist, song, or campaign ID
}

impl Command for AddInvestmentCommand {
    fn command_type(&self) -> &'static str {
        "AddInvestment"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Add NFT command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNftCommand {
    pub user_id: Uuid,
    pub nft_id: Uuid,
    pub contract_address: String,
    pub token_id: String,
}

impl Command for AddNftCommand {
    fn command_type(&self) -> &'static str {
        "AddNft"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Add tier points command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTierPointsCommand {
    pub user_id: Uuid,
    pub points: u32,
    pub reason: String,
}

impl Command for AddTierPointsCommand {
    fn command_type(&self) -> &'static str {
        "AddTierPoints"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Unlock achievement command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockAchievementCommand {
    pub user_id: Uuid,
    pub achievement_id: String,
}

impl Command for UnlockAchievementCommand {
    fn command_type(&self) -> &'static str {
        "UnlockAchievement"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        Some(self.user_id)
    }
}

/// Reset password command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordCommand {
    pub email: String,
    pub reset_token: String,
    pub new_password: String,
}

impl Command for ResetPasswordCommand {
    fn command_type(&self) -> &'static str {
        "ResetPassword"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        None // Will find user by email
    }
}

/// Request password reset command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPasswordResetCommand {
    pub email: String,
}

impl Command for RequestPasswordResetCommand {
    fn command_type(&self) -> &'static str {
        "RequestPasswordReset"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        None // Will find user by email
    }
}

/// Bulk update users command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateUsersCommand {
    pub user_ids: Vec<Uuid>,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub updated_by: Uuid, // Admin user ID
}

impl Command for BulkUpdateUsersCommand {
    fn command_type(&self) -> &'static str {
        "BulkUpdateUsers"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        None // Multiple aggregates
    }
}

/// Import users command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportUsersCommand {
    pub data: Vec<ImportUserData>,
    pub imported_by: Uuid, // Admin user ID
    pub dry_run: bool,
}

impl Command for ImportUsersCommand {
    fn command_type(&self) -> &'static str {
        "ImportUsers"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        None // Multiple new aggregates
    }
}

/// Export users command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportUsersCommand {
    pub format: String, // "csv", "json", "xlsx"
    pub filters: UserExportFilters,
    pub fields: Vec<String>,
    pub exported_by: Uuid, // Admin user ID
}

impl Command for ExportUsersCommand {
    fn command_type(&self) -> &'static str {
        "ExportUsers"
    }

    fn aggregate_id(&self) -> Option<Uuid> {
        None // Read-only operation
    }
}

// Supporting DTOs

/// Privacy settings update DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePrivacySettingsDto {
    pub profile_visibility: Option<String>,
    pub show_listening_activity: Option<bool>,
    pub show_investment_activity: Option<bool>,
    pub allow_direct_messages: Option<bool>,
    pub show_online_status: Option<bool>,
}

/// Music preferences update DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMusicPreferencesDto {
    pub favorite_genres: Option<Vec<String>>,
    pub preferred_audio_quality: Option<String>,
    pub auto_play: Option<bool>,
    pub repeat_mode: Option<String>,
    pub explicit_content: Option<bool>,
}

/// Import user data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportUserData {
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub tier: Option<String>,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub external_id: Option<String>,
}

/// User export filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExportFilters {
    pub tier: Option<String>,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub has_wallet: Option<bool>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub min_tier_points: Option<u32>,
    pub max_tier_points: Option<u32>,
}

/// Command result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub errors: Vec<String>,
}

impl CommandResult {
    pub fn success(message: String) -> Self {
        Self {
            success: true,
            message,
            data: None,
            errors: vec![],
        }
    }

    pub fn success_with_data(message: String, data: serde_json::Value) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
            errors: vec![],
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            data: None,
            errors: vec![],
        }
    }

    pub fn failure_with_errors(message: String, errors: Vec<String>) -> Self {
        Self {
            success: false,
            message,
            data: None,
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_command() {
        let command = CreateUserCommand {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            display_name: Some("Test User".to_string()),
            marketing_emails_consent: false,
        };

        assert_eq!(command.command_type(), "CreateUser");
        assert!(command.aggregate_id().is_none());
    }

    #[test]
    fn test_update_user_command() {
        let user_id = Uuid::new_v4();
        let command = UpdateUserCommand {
            user_id,
            email: Some("newemail@example.com".to_string()),
            username: None,
            is_active: Some(true),
            tier: None,
            role: None,
        };

        assert_eq!(command.command_type(), "UpdateUser");
        assert_eq!(command.aggregate_id(), Some(user_id));
    }

    #[test]
    fn test_command_result() {
        let success_result = CommandResult::success("Operation completed".to_string());
        assert!(success_result.success);
        assert_eq!(success_result.message, "Operation completed");
        assert!(success_result.errors.is_empty());

        let failure_result = CommandResult::failure("Operation failed".to_string());
        assert!(!failure_result.success);
        assert_eq!(failure_result.message, "Operation failed");
    }
} 