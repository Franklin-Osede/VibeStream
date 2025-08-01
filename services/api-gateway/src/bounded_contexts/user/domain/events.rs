use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::value_objects::{
    UserId, Email, Username, WalletAddress, UserTier, ProfileUrl
};

// Use the shared DomainEvent trait and EventMetadata
use crate::shared::domain::events::{DomainEvent, EventMetadata};

/// User registered event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub user_id: UserId,
    pub email: Email,
    pub username: Username,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserRegistered {
    pub fn new(
        user_id: UserId,
        email: Email,
        username: Username,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserRegistered",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            email,
            username,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserRegistered {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserRegistered"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User authenticated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthenticated {
    pub user_id: UserId,
    pub login_time: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserAuthenticated {
    pub fn new(user_id: UserId, login_time: DateTime<Utc>) -> Self {
        let occurred_at = Utc::now();
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserAuthenticated",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            login_time,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserAuthenticated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserAuthenticated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User profile updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUpdated {
    pub user_id: UserId,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<ProfileUrl>,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserProfileUpdated {
    pub fn new(
        user_id: UserId,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<ProfileUrl>,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserProfileUpdated",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            display_name,
            bio,
            avatar_url,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserProfileUpdated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserProfileUpdated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User tier upgraded event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTierUpgraded {
    pub user_id: UserId,
    pub old_tier: UserTier,
    pub new_tier: UserTier,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserTierUpgraded {
    pub fn new(
        user_id: UserId,
        old_tier: UserTier,
        new_tier: UserTier,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserTierUpgraded",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            old_tier,
            new_tier,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserTierUpgraded {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserTierUpgraded"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User deactivated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeactivated {
    pub user_id: UserId,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserDeactivated {
    pub fn new(user_id: UserId, reason: String, occurred_at: DateTime<Utc>) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserDeactivated",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            reason,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserDeactivated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserDeactivated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User reactivated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReactivated {
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserReactivated {
    pub fn new(user_id: UserId, occurred_at: DateTime<Utc>) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserReactivated",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserReactivated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserReactivated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User wallet linked event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWalletLinked {
    pub user_id: UserId,
    pub wallet_address: WalletAddress,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserWalletLinked {
    pub fn new(
        user_id: UserId,
        wallet_address: WalletAddress,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserWalletLinked",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            wallet_address,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserWalletLinked {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserWalletLinked"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User wallet unlinked event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWalletUnlinked {
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserWalletUnlinked {
    pub fn new(user_id: UserId, occurred_at: DateTime<Utc>) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserWalletUnlinked",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserWalletUnlinked {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserWalletUnlinked"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User email verified event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmailVerified {
    pub user_id: UserId,
    pub email: Email,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserEmailVerified {
    pub fn new(user_id: UserId, email: Email, occurred_at: DateTime<Utc>) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserEmailVerified",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            email,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserEmailVerified {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserEmailVerified"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User password changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPasswordChanged {
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserPasswordChanged {
    pub fn new(user_id: UserId, occurred_at: DateTime<Utc>) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserPasswordChanged",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserPasswordChanged {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserPasswordChanged"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// User stats updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsUpdated {
    pub user_id: UserId,
    pub stats_type: String,
    pub old_value: f64,
    pub new_value: f64,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl UserStatsUpdated {
    pub fn new(
        user_id: UserId,
        stats_type: String,
        old_value: f64,
        new_value: f64,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "UserStatsUpdated",
            user_id.value(),
            "User"
        );
        
        Self {
            user_id,
            stats_type,
            old_value,
            new_value,
            occurred_at,
            metadata,
        }
    }
}

impl DomainEvent for UserStatsUpdated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &'static str {
        "UserStatsUpdated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id.value()
    }

    fn aggregate_type(&self) -> &str {
        "User"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_registered_event() {
        let user_id = UserId::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let now = Utc::now();

        let event = UserRegistered::new(user_id.clone(), email, username, now);

        assert_eq!(event.event_type(), "UserRegistered");
        assert_eq!(event.aggregate_id(), user_id.value());
        assert_eq!(event.occurred_at(), now);
    }

    #[test]
    fn test_user_tier_upgraded_event() {
        let user_id = UserId::new();
        let now = Utc::now();

        let event = UserTierUpgraded::new(
            user_id.clone(),
            UserTier::Free,
            UserTier::Premium,
            now
        );

        assert_eq!(event.event_type(), "UserTierUpgraded");
        assert_eq!(event.aggregate_id(), user_id.value());
        assert_eq!(event.old_tier, UserTier::Free);
        assert_eq!(event.new_tier, UserTier::Premium);
    }

    #[test]
    fn test_user_wallet_linked_event() {
        let user_id = UserId::new();
        let wallet = WalletAddress::new("0x742d35Cc6345C16fd86b1B1b4b85e73c5c9c8E9b".to_string()).unwrap();
        let now = Utc::now();

        let event = UserWalletLinked::new(user_id.clone(), wallet.clone(), now);

        assert_eq!(event.event_type(), "UserWalletLinked");
        assert_eq!(event.aggregate_id(), user_id.value());
        assert_eq!(event.wallet_address, wallet);
    }
} 