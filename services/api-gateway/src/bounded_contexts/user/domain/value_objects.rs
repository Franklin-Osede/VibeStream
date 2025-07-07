use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use uuid::Uuid;
use regex::Regex;

/// User ID - Unique identifier for users
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Email address with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        let email = email.trim().to_lowercase();
        
        if email.is_empty() {
            return Err("Email no puede estar vacío".to_string());
        }

        // Regex básica para validación de email
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        
        if !email_regex.is_match(&email) {
            return Err("Formato de email inválido".to_string());
        }

        if email.len() > 254 {
            return Err("Email demasiado largo (máximo 254 caracteres)".to_string());
        }

        Ok(Self(email))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Username with validation rules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, String> {
        let username = username.trim().to_string();

        if username.is_empty() {
            return Err("Username no puede estar vacío".to_string());
        }

        if username.len() < 3 {
            return Err("Username debe tener al menos 3 caracteres".to_string());
        }

        if username.len() > 30 {
            return Err("Username no puede tener más de 30 caracteres".to_string());
        }

        // Solo letras, números, guiones y guiones bajos
        let username_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if !username_regex.is_match(&username) {
            return Err("Username solo puede contener letras, números, _ y -".to_string());
        }

        // No puede empezar o terminar con guión
        if username.starts_with('-') || username.ends_with('-') {
            return Err("Username no puede empezar o terminar con guión".to_string());
        }

        Ok(Self(username))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Hashed password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl PartialEq for PasswordHash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Wallet address for blockchain integration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WalletAddress(String);

impl WalletAddress {
    pub fn new(address: String) -> Result<Self, String> {
        let address = address.trim().to_string();

        if address.is_empty() {
            return Err("Wallet address no puede estar vacía".to_string());
        }

        // Validación básica para direcciones Ethereum (42 caracteres, empieza con 0x)
        if address.starts_with("0x") && address.len() == 42 {
            let hex_part = &address[2..];
            if hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(Self(address));
            }
        }

        // Validación básica para direcciones Solana (32-44 caracteres, base58)
        if address.len() >= 32 && address.len() <= 44 {
            // Caracteres válidos en base58
            let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
            if address.chars().all(|c| base58_chars.contains(c)) {
                return Ok(Self(address));
            }
        }

        Err("Formato de wallet address inválido".to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn is_ethereum(&self) -> bool {
        self.0.starts_with("0x") && self.0.len() == 42
    }

    pub fn is_solana(&self) -> bool {
        !self.0.starts_with("0x") && self.0.len() >= 32 && self.0.len() <= 44
    }
}

/// User tier for gamification and features
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserTier {
    Free,
    Premium,
    Vip,
    Artist,
}

impl UserTier {
    pub fn points_required(&self) -> u32 {
        match self {
            Self::Free => 0,
            Self::Premium => 500,
            Self::Vip => 1000,
            Self::Artist => 0, // Special tier, not points-based
        }
    }

    pub fn features(&self) -> Vec<&'static str> {
        match self {
            Self::Free => vec!["basic_streaming", "basic_rewards"],
            Self::Premium => vec!["basic_streaming", "basic_rewards", "enhanced_rewards", "priority_support"],
            Self::Vip => vec!["basic_streaming", "basic_rewards", "enhanced_rewards", "priority_support", "exclusive_content", "advanced_analytics"],
            Self::Artist => vec!["basic_streaming", "artist_dashboard", "upload_music", "campaign_creation", "advanced_analytics", "priority_support"],
        }
    }
}

impl Display for UserTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Free => write!(f, "free"),
            Self::Premium => write!(f, "premium"),
            Self::Vip => write!(f, "vip"),
            Self::Artist => write!(f, "artist"),
        }
    }
}

/// User role for authorization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Artist,
    Admin,
    Moderator,
}

impl UserRole {
    pub fn permissions(&self) -> Vec<&'static str> {
        match self {
            Self::User => vec!["read_content", "listen_music", "purchase_nfts", "invest_shares"],
            Self::Artist => vec!["read_content", "listen_music", "upload_music", "create_campaigns", "view_analytics"],
            Self::Moderator => vec!["read_content", "moderate_content", "view_reports", "manage_users"],
            Self::Admin => vec!["*"], // All permissions
        }
    }

    pub fn can(&self, permission: &str) -> bool {
        let permissions = self.permissions();
        permissions.contains(&"*") || permissions.contains(&permission)
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Artist => write!(f, "artist"),
            Self::Admin => write!(f, "admin"),
            Self::Moderator => write!(f, "moderator"),
        }
    }
}

/// Profile URL for images and content
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileUrl(String);

impl ProfileUrl {
    pub fn new(url: String) -> Result<Self, String> {
        let url = url.trim().to_string();

        if url.is_empty() {
            return Err("URL no puede estar vacía".to_string());
        }

        // Validación básica de URL
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return Err("URL debe empezar con http:// o https://".to_string());
        }

        if url.len() > 2048 {
            return Err("URL demasiado larga (máximo 2048 caracteres)".to_string());
        }

        Ok(Self(url))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for ProfileUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(Email::new("test@example.com".to_string()).is_ok());
        assert!(Email::new("user+tag@domain.co.uk".to_string()).is_ok());
        assert!(Email::new("invalid-email".to_string()).is_err());
        assert!(Email::new("".to_string()).is_err());
    }

    #[test]
    fn test_username_validation() {
        assert!(Username::new("valid_user123".to_string()).is_ok());
        assert!(Username::new("a-b-c".to_string()).is_ok());
        assert!(Username::new("ab".to_string()).is_err()); // Too short
        assert!(Username::new("-invalid".to_string()).is_err()); // Starts with dash
        assert!(Username::new("invalid@".to_string()).is_err()); // Invalid character
    }

    #[test]
    fn test_wallet_validation() {
        // Ethereum address
        assert!(WalletAddress::new("0x742d35Cc6345C16fd86b1B1b4b85e73c5c9c8E9b".to_string()).is_ok());
        
        // Solana address (example)
        assert!(WalletAddress::new("11111111111111111111111111111112".to_string()).is_ok());
        
        // Invalid
        assert!(WalletAddress::new("invalid".to_string()).is_err());
        assert!(WalletAddress::new("".to_string()).is_err());
    }

    #[test]
    fn test_user_tier_features() {
        let tier = UserTier::Premium;
        let features = tier.features();
        assert!(features.contains(&"enhanced_rewards"));
        assert!(features.contains(&"priority_support"));
    }

    #[test]
    fn test_user_role_permissions() {
        let admin = UserRole::Admin;
        assert!(admin.can("any_permission"));

        let user = UserRole::User;
        assert!(user.can("read_content"));
        assert!(!user.can("upload_music"));
    }
} 