use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{WristbandId, FanId};

/// QR Code service for generating and validating wristband QR codes
#[derive(Debug, Clone)]
pub struct QrCodeService {
    base_url: String,
    secret_key: String,
}

impl QrCodeService {
    pub fn new(base_url: String, secret_key: String) -> Self {
        Self { base_url, secret_key }
    }

    /// Generate QR code for wristband
    pub fn generate_qr_code(&self, wristband_id: &WristbandId, fan_id: &FanId) -> QrCodeData {
        let qr_data = QrCodeData {
            wristband_id: wristband_id.clone(),
            fan_id: fan_id.clone(),
            generated_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(30), // 30 days validity
            signature: self.generate_signature(wristband_id, fan_id),
        };

        qr_data
    }

    /// Generate QR code URL for wristband
    pub fn generate_qr_url(&self, wristband_id: &WristbandId, fan_id: &FanId) -> String {
        let qr_data = self.generate_qr_code(wristband_id, fan_id);
        let encoded_data = self.encode_qr_data(&qr_data);
        format!("{}/wristband/{}", self.base_url, encoded_data)
    }

    /// Validate QR code
    pub fn validate_qr_code(&self, qr_data: &QrCodeData) -> Result<ValidationResult, String> {
        // Check expiration
        if qr_data.expires_at < Utc::now() {
            return Ok(ValidationResult::Expired {
                reason: "QR code has expired".to_string(),
            });
        }

        // Verify signature
        let expected_signature = self.generate_signature(&qr_data.wristband_id, &qr_data.fan_id);
        if qr_data.signature != expected_signature {
            return Ok(ValidationResult::Invalid {
                reason: "Invalid QR code signature".to_string(),
            });
        }

        Ok(ValidationResult::Valid {
            wristband_id: qr_data.wristband_id.clone(),
            fan_id: qr_data.fan_id.clone(),
            generated_at: qr_data.generated_at,
        })
    }

    /// Decode QR code from URL
    pub fn decode_qr_from_url(&self, url: &str) -> Result<QrCodeData, String> {
        // Extract encoded data from URL
        let encoded_data = url.split('/').last()
            .ok_or("Invalid QR code URL")?;
        
        self.decode_qr_data(encoded_data)
    }

    /// Encode QR data to string
    fn encode_qr_data(&self, qr_data: &QrCodeData) -> String {
        // Simple base64 encoding for now
        let json_data = serde_json::to_string(qr_data)
            .expect("Failed to serialize QR data");
        base64::encode(json_data)
    }

    /// Decode QR data from string
    fn decode_qr_data(&self, encoded_data: &str) -> Result<QrCodeData, String> {
        let decoded_bytes = base64::decode(encoded_data)
            .map_err(|e| format!("Failed to decode QR data: {}", e))?;
        
        let json_str = String::from_utf8(decoded_bytes)
            .map_err(|e| format!("Invalid UTF-8 in QR data: {}", e))?;
        
        serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse QR data: {}", e))
    }

    /// Generate signature for QR code
    fn generate_signature(&self, wristband_id: &WristbandId, fan_id: &FanId) -> String {
        let data = format!("{}{}{}", wristband_id.0, fan_id.0, self.secret_key);
        // Simple hash for now - in production, use proper cryptographic signature
        format!("{:x}", md5::compute(data.as_bytes()))
    }
}

/// QR Code data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeData {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub signature: String,
}

/// QR Code validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    Valid {
        wristband_id: WristbandId,
        fan_id: FanId,
        generated_at: DateTime<Utc>,
    },
    Invalid {
        reason: String,
    },
    Expired {
        reason: String,
    },
}

/// QR Code service implementation for wristbands
#[derive(Debug, Clone)]
pub struct WristbandQrService {
    qr_service: QrCodeService,
}

impl WristbandQrService {
    pub fn new(base_url: String, secret_key: String) -> Self {
        Self {
            qr_service: QrCodeService::new(base_url, secret_key),
        }
    }

    /// Generate QR code for wristband
    pub fn generate_wristband_qr(&self, wristband_id: &WristbandId, fan_id: &FanId) -> WristbandQrCode {
        let qr_data = self.qr_service.generate_qr_code(wristband_id, fan_id);
        let qr_url = self.qr_service.generate_qr_url(wristband_id, fan_id);
        
        WristbandQrCode {
            qr_data,
            qr_url,
            qr_image_url: self.generate_qr_image_url(&qr_url),
        }
    }

    /// Validate wristband QR code
    pub fn validate_wristband_qr(&self, qr_data: &QrCodeData) -> Result<WristbandValidationResult, String> {
        match self.qr_service.validate_qr_code(qr_data)? {
            ValidationResult::Valid { wristband_id, fan_id, generated_at } => {
                Ok(WristbandValidationResult::Valid {
                    wristband_id,
                    fan_id,
                    generated_at,
                    benefits: self.get_wristband_benefits(&wristband_id),
                })
            }
            ValidationResult::Invalid { reason } => {
                Ok(WristbandValidationResult::Invalid { reason })
            }
            ValidationResult::Expired { reason } => {
                Ok(WristbandValidationResult::Expired { reason })
            }
        }
    }

    /// Generate QR image URL
    fn generate_qr_image_url(&self, qr_url: &str) -> String {
        // Use a QR code generation service
        format!("https://api.qrserver.com/v1/create-qr-code/?size=200x200&data={}", qr_url)
    }

    /// Get wristband benefits (would typically fetch from database)
    fn get_wristband_benefits(&self, _wristband_id: &WristbandId) -> Vec<String> {
        // This would typically fetch from database based on wristband type
        vec![
            "Concert access".to_string(),
            "VIP seating".to_string(),
            "Premium merchandise discount".to_string(),
        ]
    }
}

/// Wristband QR Code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WristbandQrCode {
    pub qr_data: QrCodeData,
    pub qr_url: String,
    pub qr_image_url: String,
}

/// Wristband validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WristbandValidationResult {
    Valid {
        wristband_id: WristbandId,
        fan_id: FanId,
        generated_at: DateTime<Utc>,
        benefits: Vec<String>,
    },
    Invalid {
        reason: String,
    },
    Expired {
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_code_generation() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        
        // When
        let qr_data = service.generate_qr_code(&wristband_id, &fan_id);
        
        // Then
        assert_eq!(qr_data.wristband_id, wristband_id);
        assert_eq!(qr_data.fan_id, fan_id);
        assert!(qr_data.expires_at > Utc::now());
        assert!(!qr_data.signature.is_empty());
    }

    #[test]
    fn test_qr_code_validation() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let qr_data = service.generate_qr_code(&wristband_id, &fan_id);
        
        // When
        let result = service.validate_qr_code(&qr_data);
        
        // Then
        assert!(result.is_ok());
        match result.unwrap() {
            ValidationResult::Valid { wristband_id: result_wristband_id, fan_id: result_fan_id, .. } => {
                assert_eq!(result_wristband_id, wristband_id);
                assert_eq!(result_fan_id, fan_id);
            }
            _ => panic!("Expected valid result"),
        }
    }

    #[test]
    fn test_qr_code_encoding_decoding() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let qr_data = service.generate_qr_code(&wristband_id, &fan_id);
        
        // When
        let encoded = service.encode_qr_data(&qr_data);
        let decoded = service.decode_qr_data(&encoded);
        
        // Then
        assert!(decoded.is_ok());
        let decoded_data = decoded.unwrap();
        assert_eq!(decoded_data.wristband_id, qr_data.wristband_id);
        assert_eq!(decoded_data.fan_id, qr_data.fan_id);
    }

    #[test]
    fn test_wristband_qr_service() {
        // Given
        let service = WristbandQrService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        
        // When
        let wristband_qr = service.generate_wristband_qr(&wristband_id, &fan_id);
        
        // Then
        assert_eq!(wristband_qr.qr_data.wristband_id, wristband_id);
        assert_eq!(wristband_qr.qr_data.fan_id, fan_id);
        assert!(!wristband_qr.qr_url.is_empty());
        assert!(!wristband_qr.qr_image_url.is_empty());
    }
}
