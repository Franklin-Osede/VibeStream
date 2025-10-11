//! Fan Loyalty Commands
//! 
//! TDD GREEN PHASE - Command structures for Fan Loyalty System

use serde::{Deserialize, Serialize};
use crate::bounded_contexts::fan_loyalty::domain::entities::{FanId, WristbandType, BiometricData, LocationData};

/// Verify Fan Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyFanCommand {
    pub fan_id: FanId,
    pub biometric_data: BiometricData,
    pub device_fingerprint: String,
    pub location: Option<LocationData>,
}

impl VerifyFanCommand {
    pub fn new(
        fan_id: FanId,
        biometric_data: BiometricData,
        device_fingerprint: String,
        location: Option<LocationData>,
    ) -> Self {
        Self {
            fan_id,
            biometric_data,
            device_fingerprint,
            location,
        }
    }
}

/// Create Wristband Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWristbandCommand {
    pub fan_id: FanId,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: WristbandType,
    pub fan_wallet_address: String,
}

impl CreateWristbandCommand {
    pub fn new(
        fan_id: FanId,
        concert_id: String,
        artist_id: String,
        wristband_type: WristbandType,
        fan_wallet_address: String,
    ) -> Self {
        Self {
            fan_id,
            concert_id,
            artist_id,
            wristband_type,
            fan_wallet_address,
        }
    }
}

/// Activate Wristband Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateWristbandCommand {
    pub wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
    pub activation_code: Option<String>,
}

impl ActivateWristbandCommand {
    pub fn new(
        wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
        activation_code: Option<String>,
    ) -> Self {
        Self {
            wristband_id,
            activation_code,
        }
    }
}

/// Generate QR Code Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateQrCodeCommand {
    pub wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
    pub expiration_hours: Option<u32>,
}

impl GenerateQrCodeCommand {
    pub fn new(
        wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
        expiration_hours: Option<u32>,
    ) -> Self {
        Self {
            wristband_id,
            expiration_hours,
        }
    }
}

/// Validate QR Code Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateQrCodeCommand {
    pub qr_code: String,
    pub validation_context: Option<QrValidationContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrValidationContext {
    pub location: Option<LocationData>,
    pub device_fingerprint: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ValidateQrCodeCommand {
    pub fn new(qr_code: String, validation_context: Option<QrValidationContext>) -> Self {
        Self {
            qr_code,
            validation_context,
        }
    }
}

/// Get Wristband Details Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWristbandDetailsCommand {
    pub wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
    pub include_benefits: bool,
    pub include_nft_info: bool,
}

impl GetWristbandDetailsCommand {
    pub fn new(
        wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
        include_benefits: bool,
        include_nft_info: bool,
    ) -> Self {
        Self {
            wristband_id,
            include_benefits,
            include_nft_info,
        }
    }
}

/// Update Wristband Status Command - TDD GREEN PHASE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWristbandStatusCommand {
    pub wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
    pub new_status: WristbandStatus,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WristbandStatus {
    Active,
    Inactive,
    Suspended,
    Expired,
    Revoked,
}

impl UpdateWristbandStatusCommand {
    pub fn new(
        wristband_id: crate::bounded_contexts::fan_loyalty::domain::WristbandId,
        new_status: WristbandStatus,
        reason: Option<String>,
    ) -> Self {
        Self {
            wristband_id,
            new_status,
            reason,
        }
    }
}