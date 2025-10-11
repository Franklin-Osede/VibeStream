use crate::bounded_contexts::fan_loyalty::domain::{WristbandId};

/// Query to get wristband by ID
#[derive(Debug, Clone)]
pub struct GetWristbandQuery {
    pub wristband_id: WristbandId,
}

/// Query to validate QR code
#[derive(Debug, Clone)]
pub struct ValidateQrCodeQuery {
    pub qr_code: String,
}