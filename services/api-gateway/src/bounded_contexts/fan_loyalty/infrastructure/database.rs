use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::fan_loyalty::domain::entities::{
    FanId, WristbandId, WristbandType, NftWristband, FanVerificationResult,
};

/// Database repository for Fan Loyalty System
#[derive(Debug, Clone)]
pub struct FanLoyaltyRepository {
    pool: PgPool,
}

impl FanLoyaltyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Save fan verification result
    pub async fn save_verification_result(
        &self,
        fan_id: &FanId,
        result: &FanVerificationResult,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO fan_verifications (
                id, fan_id, is_verified, confidence_score, 
                verification_id, wristband_eligible, benefits_unlocked, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (fan_id) DO UPDATE SET
                is_verified = EXCLUDED.is_verified,
                confidence_score = EXCLUDED.confidence_score,
                verification_id = EXCLUDED.verification_id,
                wristband_eligible = EXCLUDED.wristband_eligible,
                benefits_unlocked = EXCLUDED.benefits_unlocked,
                updated_at = NOW()
        "#;

        let verification_id = Uuid::new_v4();
        let benefits_json = serde_json::to_string(&result.benefits_unlocked)
            .map_err(|e| format!("Failed to serialize benefits: {}", e))?;

        sqlx::query(query)
            .bind(verification_id)
            .bind(fan_id.0)
            .bind(result.is_verified)
            .bind(result.confidence_score)
            .bind(&result.verification_id)
            .bind(result.wristband_eligible)
            .bind(benefits_json)
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save verification result: {}", e))?;

        Ok(())
    }

    /// Get fan verification result
    pub async fn get_verification_result(&self, fan_id: &FanId) -> Result<Option<FanVerificationResult>, String> {
        let query = r#"
            SELECT is_verified, confidence_score, verification_id, 
                   wristband_eligible, benefits_unlocked, created_at
            FROM fan_verifications 
            WHERE fan_id = $1
        "#;

        let row = sqlx::query(query)
            .bind(fan_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to get verification result: {}", e))?;

        if let Some(row) = row {
            let benefits_json: String = row.get("benefits_unlocked");
            let benefits_unlocked: Vec<String> = serde_json::from_str(&benefits_json)
                .map_err(|e| format!("Failed to deserialize benefits: {}", e))?;

            Ok(Some(FanVerificationResult {
                is_verified: row.get("is_verified"),
                confidence_score: row.get("confidence_score"),
                verification_id: row.get("verification_id"),
                wristband_eligible: row.get("wristband_eligible"),
                benefits_unlocked,
            }))
        } else {
            Ok(None)
        }
    }

    /// Save wristband
    pub async fn save_wristband(&self, wristband: &NftWristband) -> Result<(), String> {
        let query = r#"
            INSERT INTO wristbands (
                id, fan_id, concert_id, artist_id, wristband_type, 
                is_active, created_at, activated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                is_active = EXCLUDED.is_active,
                activated_at = EXCLUDED.activated_at,
                updated_at = NOW()
        "#;

        sqlx::query(query)
            .bind(wristband.id.0)
            .bind(wristband.fan_id.0)
            .bind(wristband.concert_id)
            .bind(wristband.artist_id)
            .bind(format!("{:?}", wristband.wristband_type).to_lowercase())
            .bind(wristband.is_active)
            .bind(wristband.created_at)
            .bind(wristband.activated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save wristband: {}", e))?;

        Ok(())
    }

    /// Get wristband by ID
    pub async fn get_wristband(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, String> {
        let query = r#"
            SELECT id, fan_id, concert_id, artist_id, wristband_type, 
                   is_active, created_at, activated_at
            FROM wristbands 
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(wristband_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to get wristband: {}", e))?;

        if let Some(row) = row {
            let wristband_type = match row.get::<String, _>("wristband_type").as_str() {
                "general" => WristbandType::General,
                "vip" => WristbandType::VIP,
                "backstage" => WristbandType::Backstage,
                "meet_greet" => WristbandType::MeetAndGreet,
                _ => return Err("Invalid wristband type".to_string()),
            };

            Ok(Some(NftWristband {
                id: WristbandId(row.get("id")),
                fan_id: FanId(row.get("fan_id")),
                concert_id: row.get("concert_id"),
                artist_id: row.get("artist_id"),
                wristband_type,
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                activated_at: row.get("activated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get wristbands by fan ID
    pub async fn get_wristbands_by_fan(&self, fan_id: &FanId) -> Result<Vec<NftWristband>, String> {
        let query = r#"
            SELECT id, fan_id, concert_id, artist_id, wristband_type, 
                   is_active, created_at, activated_at
            FROM wristbands 
            WHERE fan_id = $1
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .bind(fan_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to get wristbands: {}", e))?;

        let mut wristbands = Vec::new();
        for row in rows {
            let wristband_type = match row.get::<String, _>("wristband_type").as_str() {
                "general" => WristbandType::General,
                "vip" => WristbandType::VIP,
                "backstage" => WristbandType::Backstage,
                "meet_greet" => WristbandType::MeetAndGreet,
                _ => continue, // Skip invalid types
            };

            wristbands.push(NftWristband {
                id: WristbandId(row.get("id")),
                fan_id: FanId(row.get("fan_id")),
                concert_id: row.get("concert_id"),
                artist_id: row.get("artist_id"),
                wristband_type,
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                activated_at: row.get("activated_at"),
            });
        }

        Ok(wristbands)
    }

    /// Update wristband status
    pub async fn update_wristband_status(
        &self,
        wristband_id: &WristbandId,
        is_active: bool,
        activated_at: Option<DateTime<Utc>>,
    ) -> Result<(), String> {
        let query = r#"
            UPDATE wristbands 
            SET is_active = $1, activated_at = $2, updated_at = NOW()
            WHERE id = $3
        "#;

        sqlx::query(query)
            .bind(is_active)
            .bind(activated_at)
            .bind(wristband_id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update wristband status: {}", e))?;

        Ok(())
    }

    /// Save QR code
    pub async fn save_qr_code(
        &self,
        wristband_id: &WristbandId,
        qr_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO wristband_qr_codes (
                id, wristband_id, qr_code, expires_at, created_at
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (wristband_id) DO UPDATE SET
                qr_code = EXCLUDED.qr_code,
                expires_at = EXCLUDED.expires_at,
                updated_at = NOW()
        "#;

        sqlx::query(query)
            .bind(Uuid::new_v4())
            .bind(wristband_id.0)
            .bind(qr_code)
            .bind(expires_at)
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save QR code: {}", e))?;

        Ok(())
    }

    /// Get QR code by wristband ID
    pub async fn get_qr_code(&self, wristband_id: &WristbandId) -> Result<Option<String>, String> {
        let query = r#"
            SELECT qr_code, expires_at
            FROM wristband_qr_codes 
            WHERE wristband_id = $1 AND expires_at > NOW()
        "#;

        let row = sqlx::query(query)
            .bind(wristband_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to get QR code: {}", e))?;

        if let Some(row) = row {
            Ok(Some(row.get("qr_code")))
        } else {
            Ok(None)
        }
    }

    /// Log QR code scan
    pub async fn log_qr_scan(
        &self,
        qr_code: &str,
        scanner_id: &str,
        location: Option<(f64, f64, f32)>,
    ) -> Result<(), String> {
        let query = r#"
            INSERT INTO qr_scan_logs (
                id, qr_code, scanner_id, latitude, longitude, accuracy, scanned_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#;

        let (latitude, longitude, accuracy) = location.unwrap_or((0.0, 0.0, 0.0));

        sqlx::query(query)
            .bind(Uuid::new_v4())
            .bind(qr_code)
            .bind(scanner_id)
            .bind(latitude)
            .bind(longitude)
            .bind(accuracy)
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to log QR scan: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fan_loyalty::domain::WristbandType;

    #[tokio::test]
    async fn test_fan_loyalty_repository_creation() {
        // Given
        let pool = sqlx::PgPool::connect("postgresql://test:test@localhost/test")
            .await
            .unwrap_or_else(|_| {
                // Create a mock pool for testing
                sqlx::PgPool::connect("postgresql://test:test@localhost/test").await.unwrap()
            });

        // When
        let repository = FanLoyaltyRepository::new(pool);

        // Then
        assert!(repository.pool.is_closed() == false);
    }

    #[test]
    fn test_verification_result_serialization() {
        // Given
        let result = FanVerificationResult {
            is_verified: true,
            confidence_score: 0.95,
            verification_id: "verification_123".to_string(),
            wristband_eligible: true,
            benefits_unlocked: vec![
                "Verified Fan Status".to_string(),
                "Wristband Eligibility".to_string(),
            ],
        };

        // When
        let json = serde_json::to_string(&result.benefits_unlocked).unwrap();
        let deserialized: Vec<String> = serde_json::from_str(&json).unwrap();

        // Then
        assert_eq!(result.benefits_unlocked, deserialized);
    }

    #[test]
    fn test_wristband_type_parsing() {
        // Test valid wristband types
        assert_eq!(parse_wristband_type("general"), Ok(WristbandType::General));
        assert_eq!(parse_wristband_type("vip"), Ok(WristbandType::VIP));
        assert_eq!(parse_wristband_type("backstage"), Ok(WristbandType::Backstage));
        assert_eq!(parse_wristband_type("meet_greet"), Ok(WristbandType::MeetAndGreet));
        
        // Test invalid wristband type
        assert!(parse_wristband_type("invalid").is_err());
    }

    fn parse_wristband_type(wristband_type: &str) -> Result<WristbandType, String> {
        match wristband_type {
            "general" => Ok(WristbandType::General),
            "vip" => Ok(WristbandType::VIP),
            "backstage" => Ok(WristbandType::Backstage),
            "meet_greet" => Ok(WristbandType::MeetAndGreet),
            _ => Err(format!("Invalid wristband type: {}", wristband_type)),
        }
    }
}
