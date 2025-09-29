use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{
    FanId, LoyaltyTier, WristbandType, WristbandId, BiometricScore,
};

/// Query to get fan loyalty information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFanLoyaltyQuery {
    pub fan_id: FanId,
}

impl GetFanLoyaltyQuery {
    pub fn new(fan_id: FanId) -> Self {
        Self { fan_id }
    }
}

/// Query to get NFT wristbands for fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFanWristbandsQuery {
    pub fan_id: FanId,
}

impl GetFanWristbandsQuery {
    pub fn new(fan_id: FanId) -> Self {
        Self { fan_id }
    }
}

/// Query to get NFT wristbands for concert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConcertWristbandsQuery {
    pub concert_id: Uuid,
}

impl GetConcertWristbandsQuery {
    pub fn new(concert_id: Uuid) -> Self {
        Self { concert_id }
    }
}

/// Query to get fan biometric score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFanBiometricScoreQuery {
    pub fan_id: FanId,
}

impl GetFanBiometricScoreQuery {
    pub fn new(fan_id: FanId) -> Self {
        Self { fan_id }
    }
}

/// Query to get loyalty points for fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFanLoyaltyPointsQuery {
    pub fan_id: FanId,
}

impl GetFanLoyaltyPointsQuery {
    pub fn new(fan_id: FanId) -> Self {
        Self { fan_id }
    }
}

/// Query to get fan verification status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFanVerificationStatusQuery {
    pub fan_id: FanId,
}

impl GetFanVerificationStatusQuery {
    pub fn new(fan_id: FanId) -> Self {
        Self { fan_id }
    }
}

/// Query to get wristband by QR code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWristbandByQrCodeQuery {
    pub qr_code: String,
}

impl GetWristbandByQrCodeQuery {
    pub fn new(qr_code: String) -> Self {
        Self { qr_code }
    }
}

/// Query to get fan engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFanEngagementMetricsQuery {
    pub fan_id: FanId,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl GetFanEngagementMetricsQuery {
    pub fn new(fan_id: FanId) -> Self {
        Self {
            fan_id,
            start_date: None,
            end_date: None,
        }
    }

    pub fn with_date_range(mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self.end_date = Some(end_date);
        self
    }
}

/// Query to get artist fan statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistFanStatisticsQuery {
    pub artist_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl GetArtistFanStatisticsQuery {
    pub fn new(artist_id: Uuid) -> Self {
        Self {
            artist_id,
            start_date: None,
            end_date: None,
        }
    }

    pub fn with_date_range(mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self.end_date = Some(end_date);
        self
    }
}

/// Query to get concert access list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConcertAccessListQuery {
    pub concert_id: Uuid,
}

impl GetConcertAccessListQuery {
    pub fn new(concert_id: Uuid) -> Self {
        Self { concert_id }
    }
}

/// Query to check fan eligibility for wristband
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckFanEligibilityQuery {
    pub fan_id: FanId,
    pub wristband_type: WristbandType,
}

impl CheckFanEligibilityQuery {
    pub fn new(fan_id: FanId, wristband_type: WristbandType) -> Self {
        Self { fan_id, wristband_type }
    }
}
