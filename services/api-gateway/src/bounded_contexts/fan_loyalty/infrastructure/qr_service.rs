use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

use crate::bounded_contexts::fan_loyalty::domain::entities::{
    WristbandId, FanId, QrCode, QrCodeValidation, QrCodeScanResult, LocationData
};

/// QR Code service for wristband access control
#[derive(Debug, Clone)]
pub struct QrCodeService {
    base_url: String,
    secret_key: String,
    expiration_hours: i64,
}


impl QrCodeService {
    pub fn new(base_url: String, secret_key: String, expiration_hours: i64) -> Self {
        Self {
            base_url,
            secret_key,
            expiration_hours,
        }
    }

    /// Generate QR code for wristband
    pub async fn generate_qr_code(&self, wristband_id: &WristbandId) -> Result<QrCode, String> {
        let code = self.generate_unique_code(wristband_id);
        let url = self.generate_qr_url(&code);
        let expires_at = Utc::now() + Duration::hours(self.expiration_hours);
        
        Ok(QrCode {
            code,
            url,
            wristband_id: *wristband_id,
            expires_at,
            created_at: Utc::now(),
        })
    }

    /// Validate QR code
    pub async fn validate_qr_code(&self, code: &str) -> Result<QrCodeValidation, String> {
        // Parse the QR code to extract wristband ID and signature
        let (wristband_id, signature) = self.parse_qr_code(code)?;
        
        // Verify signature
        if !self.verify_signature(&wristband_id, signature) {
            return Err("Invalid QR code signature".to_string());
        }

        // Check expiration
        let expires_at = self.get_expiration_from_code(code)?;
        if Utc::now() > expires_at {
            return Err("QR code has expired".to_string());
        }

        Ok(QrCodeValidation {
            is_valid: true,
            wristband_id,
            expires_at: Some(expires_at),
        })
    }

    /// Scan QR code for access control
    pub async fn scan_qr_code(
        &self,
        code: &str,
        scanner_id: &str,
        location: Option<LocationData>,
    ) -> Result<QrCodeScanResult, String> {
        // Validate QR code first
        let validation = self.validate_qr_code(code).await?;
        
        if !validation.is_valid {
            return Ok(QrCodeScanResult {
                scan_successful: false,
                wristband_id: None,
                fan_id: None,
                access_granted: false,
                benefits_available: vec![],
                scan_timestamp: Utc::now(),
            });
        }

        // Log scan event
        self.log_scan_event(scanner_id, &validation.wristband_id, location).await?;

        // Determine access and benefits
        let (access_granted, benefits) = self.determine_access_and_benefits(&validation.wristband_id).await?;

        Ok(QrCodeScanResult {
            scan_successful: true,
            wristband_id: Some(validation.wristband_id),
            fan_id: Some(FanId::new()), // Would fetch from database
            access_granted,
            benefits_available: benefits,
            scan_timestamp: Utc::now(),
        })
    }

    /// Generate unique code for wristband
    fn generate_unique_code(&self, wristband_id: &WristbandId) -> String {
        let timestamp = Utc::now().timestamp();
        let data = format!("{}{}{}", wristband_id.0, timestamp, self.secret_key);
        let hash = Sha256::digest(data.as_bytes());
        let signature = general_purpose::STANDARD.encode(&hash[..16]);
        format!("VS{}{}", wristband_id.0.to_string()[..8].to_uppercase(), signature)
    }

    /// Generate QR URL
    fn generate_qr_url(&self, code: &str) -> String {
        format!("{}/wristband/{}", self.base_url, code)
    }

    /// Parse QR code to extract wristband ID and signature
    fn parse_qr_code(&self, code: &str) -> Result<(WristbandId, String), String> {
        if !code.starts_with("VS") {
            return Err("Invalid QR code format".to_string());
        }

        let code_without_prefix = &code[2..];
        if code_without_prefix.len() < 24 {
            return Err("QR code too short".to_string());
        }

        let wristband_part = &code_without_prefix[..8];
        let signature = &code_without_prefix[8..];

        // Convert wristband part back to UUID (simplified)
        let wristband_id = Uuid::parse_str(&format!(
            "{}-{}-{}-{}-{}",
            &wristband_part[..8],
            &wristband_part[8..12],
            &wristband_part[12..16],
            &wristband_part[16..20],
            &wristband_part[20..24]
        )).map_err(|_| "Invalid wristband ID in QR code")?;

        Ok((WristbandId(wristband_id), signature.to_string()))
    }

    /// Verify QR code signature
    fn verify_signature(&self, wristband_id: &WristbandId, signature: &str) -> bool {
        // In a real implementation, this would verify the cryptographic signature
        // For now, we'll do a simple validation
        signature.len() == 24 && signature.chars().all(|c| c.is_alphanumeric())
    }

    /// Get expiration from QR code
    fn get_expiration_from_code(&self, _code: &str) -> Result<DateTime<Utc>, String> {
        // In a real implementation, this would extract timestamp from the code
        // For now, we'll return a default expiration
        Ok(Utc::now() + Duration::hours(self.expiration_hours))
    }

    /// Log scan event
    async fn log_scan_event(
        &self,
        scanner_id: &str,
        wristband_id: &WristbandId,
        location: Option<LocationData>,
    ) -> Result<(), String> {
        // In a real implementation, this would log to database
        println!(
            "QR Code scanned: scanner_id={}, wristband_id={}, location={:?}",
            scanner_id, wristband_id.0, location
        );
        Ok(())
    }

    /// Determine access and benefits based on wristband
    async fn determine_access_and_benefits(
        &self,
        wristband_id: &WristbandId,
    ) -> Result<(bool, Vec<String>), String> {
        // In a real implementation, this would query the database
        // For now, we'll return mock data
        let benefits = vec![
            "Concert Access".to_string(),
            "VIP Lounge".to_string(),
            "Meet & Greet".to_string(),
        ];
        Ok((true, benefits))
    }
}

// Structs moved to domain::entities


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_qr_code() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
            24,
        );
        let wristband_id = WristbandId(Uuid::new_v4());
        
        // When
        let qr_code = service.generate_qr_code(&wristband_id).await.unwrap();
        
        // Then
        assert!(qr_code.code.starts_with("VS"));
        assert!(qr_code.url.contains(&qr_code.code));
        assert_eq!(qr_code.wristband_id, wristband_id);
        assert!(qr_code.expires_at > Utc::now());
    }

    #[tokio::test]
    async fn test_validate_qr_code() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
            24,
        );
        let wristband_id = WristbandId(Uuid::new_v4());
        let qr_code = service.generate_qr_code(&wristband_id).await.unwrap();
        
        // When
        let validation = service.validate_qr_code(&qr_code.code).await;
        
        // Then
        assert!(validation.is_ok());
        let validation = validation.unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.wristband_id, wristband_id);
    }

    #[tokio::test]
    async fn test_scan_qr_code() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
            24,
        );
        let wristband_id = WristbandId(Uuid::new_v4());
        let qr_code = service.generate_qr_code(&wristband_id).await.unwrap();

        // When
        let scan_result = service.scan_qr_code(
            &qr_code.code,
            "scanner_123",
            Some(LocationData {
                latitude: 40.7128,
                longitude: -74.0060,
                accuracy: 10.0,
                timestamp: Utc::now(),
            }),
        ).await.unwrap();

        // Then
        assert!(scan_result.scan_successful);
        assert_eq!(scan_result.wristband_id, Some(wristband_id));
        assert!(scan_result.access_granted);
        assert!(!scan_result.benefits_available.is_empty());
    }

    #[test]
    fn test_qr_code_serialization() {
        // Given
        let qr_code = QrCode {
            code: "VS12345678ABCDEF1234567890".to_string(),
            url: "https://vibestream.com/wristband/VS12345678ABCDEF1234567890".to_string(),
            wristband_id: WristbandId(Uuid::new_v4()),
            expires_at: Utc::now() + Duration::hours(24),
            created_at: Utc::now(),
        };
        
        // When
        let json = serde_json::to_string(&qr_code).unwrap();
        let deserialized: QrCode = serde_json::from_str(&json).unwrap();
        
        // Then
        assert_eq!(qr_code.code, deserialized.code);
        assert_eq!(qr_code.url, deserialized.url);
        assert_eq!(qr_code.wristband_id, deserialized.wristband_id);
    }

    #[test]
    fn test_invalid_qr_code_validation() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
            24,
        );
        
        // When
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            service.validate_qr_code("invalid_code")
        );
        
        // Then
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid QR code format"));
    }
}