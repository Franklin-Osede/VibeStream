use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{
    aggregates::{FanId, BiometricData, BiometricScore, LoyaltyTier},
    entities::{Fan, NftWristband, WristbandId},
    events::FanLoyaltyEvent,
};

/// Repository trait for Fan entity
#[async_trait]
pub trait FanRepository {
    async fn save(&self, fan: &Fan) -> Result<(), String>;
    async fn find_by_id(&self, fan_id: &FanId) -> Result<Option<Fan>, String>;
    async fn find_by_user_id(&self, user_id: &uuid::Uuid) -> Result<Option<Fan>, String>;
    async fn update(&self, fan: &Fan) -> Result<(), String>;
    async fn delete(&self, fan_id: &FanId) -> Result<(), String>;
}

/// Repository trait for NFT Wristband entity
#[async_trait]
pub trait NftWristbandRepository {
    async fn save(&self, wristband: &NftWristband) -> Result<(), String>;
    async fn find_by_id(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, String>;
    async fn find_by_fan_id(&self, fan_id: &FanId) -> Result<Vec<NftWristband>, String>;
    async fn find_by_concert_id(&self, concert_id: &uuid::Uuid) -> Result<Vec<NftWristband>, String>;
    async fn update(&self, wristband: &NftWristband) -> Result<(), String>;
    async fn delete(&self, wristband_id: &WristbandId) -> Result<(), String>;
}

/// Repository trait for Fan Loyalty Aggregate
#[async_trait]
pub trait FanLoyaltyRepository {
    async fn save(&self, aggregate: &crate::bounded_contexts::fan_loyalty::domain::aggregates::FanLoyaltyAggregate) -> Result<(), String>;
    async fn find_by_fan_id(&self, fan_id: &FanId) -> Result<Option<crate::bounded_contexts::fan_loyalty::domain::aggregates::FanLoyaltyAggregate>, String>;
    async fn update(&self, aggregate: &crate::bounded_contexts::fan_loyalty::domain::aggregates::FanLoyaltyAggregate) -> Result<(), String>;
}

/// Repository trait for Biometric Data
#[async_trait]
pub trait BiometricDataRepository {
    async fn save(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<(), String>;
    async fn find_by_fan_id(&self, fan_id: &FanId) -> Result<Vec<BiometricData>, String>;
    async fn find_latest_by_fan_id(&self, fan_id: &FanId) -> Result<Option<BiometricData>, String>;
    async fn delete_old_data(&self, fan_id: &FanId, older_than: DateTime<Utc>) -> Result<(), String>;
}

/// Repository trait for Domain Events
#[async_trait]
pub trait EventRepository {
    async fn save_event(&self, event: &FanLoyaltyEvent) -> Result<(), String>;
    async fn get_events_for_fan(&self, fan_id: &FanId) -> Result<Vec<FanLoyaltyEvent>, String>;
    async fn get_events_since(&self, since: DateTime<Utc>) -> Result<Vec<FanLoyaltyEvent>, String>;
    async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<FanLoyaltyEvent>, String>;
}

/// Repository trait for Loyalty Points
#[async_trait]
pub trait LoyaltyPointsRepository {
    async fn add_points(&self, fan_id: &FanId, points: u32) -> Result<(), String>;
    async fn subtract_points(&self, fan_id: &FanId, points: u32) -> Result<(), String>;
    async fn get_points(&self, fan_id: &FanId) -> Result<u32, String>;
    async fn get_points_history(&self, fan_id: &FanId) -> Result<Vec<LoyaltyPointsTransaction>, String>;
}

/// Loyalty points transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPointsTransaction {
    pub id: uuid::Uuid,
    pub fan_id: FanId,
    pub points: i32, // Positive for addition, negative for subtraction
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl LoyaltyPointsTransaction {
    pub fn new(fan_id: FanId, points: i32, reason: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            fan_id,
            points,
            reason,
            timestamp: Utc::now(),
        }
    }
}

/// Repository trait for QR Code management
#[async_trait]
pub trait QrCodeRepository {
    async fn generate_qr_code(&self, wristband_id: &WristbandId) -> Result<String, String>;
    async fn validate_qr_code(&self, qr_code: &str) -> Result<Option<WristbandId>, String>;
    async fn revoke_qr_code(&self, qr_code: &str) -> Result<(), String>;
}

/// Repository trait for Concert Access
#[async_trait]
pub trait ConcertAccessRepository {
    async fn grant_access(&self, fan_id: &FanId, concert_id: &uuid::Uuid, access_level: &str) -> Result<(), String>;
    async fn revoke_access(&self, fan_id: &FanId, concert_id: &uuid::Uuid) -> Result<(), String>;
    async fn check_access(&self, fan_id: &FanId, concert_id: &uuid::Uuid) -> Result<Option<String>, String>;
    async fn get_access_list(&self, concert_id: &uuid::Uuid) -> Result<Vec<ConcertAccess>, String>;
}

/// Concert access record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcertAccess {
    pub fan_id: FanId,
    pub concert_id: uuid::Uuid,
    pub access_level: String,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl ConcertAccess {
    pub fn new(fan_id: FanId, concert_id: uuid::Uuid, access_level: String) -> Self {
        Self {
            fan_id,
            concert_id,
            access_level,
            granted_at: Utc::now(),
            expires_at: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.expires_at.map_or(true, |expires| expires > Utc::now())
    }
}
