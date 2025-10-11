//! PostgreSQL Repositories for Fan Loyalty System
//! 
//! TDD REFACTOR PHASE - Real database implementations

use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::entities::{
    FanId, WristbandId, FanVerificationResult, NftWristband, QrCode
};
use crate::bounded_contexts::fan_loyalty::domain::repositories::{
    FanVerificationRepository, WristbandRepository, QrCodeRepository, 
    ZkProofRepository, NftRepository
};
use crate::shared::domain::errors::AppError;

// ============================================================================
// POSTGRES FAN VERIFICATION REPOSITORY
// ============================================================================

pub struct PostgresFanVerificationRepository {
    pool: PgPool,
}

impl PostgresFanVerificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FanVerificationRepository for PostgresFanVerificationRepository {
    async fn save_verification_result(
        &self,
        fan_id: &FanId,
        result: &FanVerificationResult,
    ) -> Result<(), AppError> {
        let query = r#"
            INSERT INTO fan_verifications (
                fan_id, is_verified, confidence_score, verification_id, 
                wristband_eligible, benefits_unlocked
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (fan_id) DO UPDATE SET
                is_verified = EXCLUDED.is_verified,
                confidence_score = EXCLUDED.confidence_score,
                verification_id = EXCLUDED.verification_id,
                wristband_eligible = EXCLUDED.wristband_eligible,
                benefits_unlocked = EXCLUDED.benefits_unlocked,
                updated_at = NOW()
        "#;
        
        sqlx::query(query)
            .bind(&fan_id.0)
            .bind(result.is_verified)
            .bind(result.confidence_score)
            .bind(&result.verification_id)
            .bind(result.wristband_eligible)
            .bind(serde_json::to_value(&result.benefits_unlocked).unwrap())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save verification result: {}", e)))?;
        
        Ok(())
    }

    async fn get_verification_result(&self, fan_id: &FanId) -> Result<Option<FanVerificationResult>, AppError> {
        let query = r#"
            SELECT is_verified, confidence_score, verification_id, 
                   wristband_eligible, benefits_unlocked
            FROM fan_verifications 
            WHERE fan_id = $1
        "#;
        
        let row = sqlx::query(query)
            .bind(&fan_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get verification result: {}", e)))?;
        
        if let Some(row) = row {
            let benefits: Vec<String> = serde_json::from_value(
                row.try_get("benefits_unlocked")?
            ).unwrap_or_default();
            
            Ok(Some(FanVerificationResult {
                is_verified: row.try_get("is_verified")?,
                confidence_score: row.try_get("confidence_score")?,
                verification_id: row.try_get("verification_id")?,
                wristband_eligible: row.try_get("wristband_eligible")?,
                benefits_unlocked: benefits,
            }))
        } else {
            Ok(None)
        }
    }
}

// ============================================================================
// POSTGRES WRISTBAND REPOSITORY
// ============================================================================

pub struct PostgresWristbandRepository {
    pool: PgPool,
}

impl PostgresWristbandRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WristbandRepository for PostgresWristbandRepository {
    async fn save_wristband(&self, wristband: &NftWristband) -> Result<(), AppError> {
        let query = r#"
            INSERT INTO nft_wristbands (
                id, fan_id, concert_id, artist_id, wristband_type, 
                is_active, activated_at, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                is_active = EXCLUDED.is_active,
                activated_at = EXCLUDED.activated_at,
                updated_at = NOW()
        "#;
        
        sqlx::query(query)
            .bind(&wristband.id.0)
            .bind(&wristband.fan_id.0)
            .bind(&wristband.concert_id)
            .bind(&wristband.artist_id)
            .bind(format!("{:?}", wristband.wristband_type))
            .bind(wristband.is_active)
            .bind(wristband.activated_at)
            .bind(wristband.created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save wristband: {}", e)))?;
        
        Ok(())
    }

    async fn get_wristband(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, AppError> {
        let query = r#"
            SELECT id, fan_id, concert_id, artist_id, wristband_type, 
                   is_active, activated_at, created_at
            FROM nft_wristbands 
            WHERE id = $1
        "#;
        
        let row = sqlx::query(query)
            .bind(&wristband_id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get wristband: {}", e)))?;
        
        if let Some(row) = row {
            let wristband_type_str: String = row.try_get("wristband_type")?;
            let wristband_type = match wristband_type_str.as_str() {
                "General" => crate::bounded_contexts::fan_loyalty::domain::WristbandType::General,
                "VIP" => crate::bounded_contexts::fan_loyalty::domain::WristbandType::VIP,
                "Backstage" => crate::bounded_contexts::fan_loyalty::domain::WristbandType::Backstage,
                "MeetAndGreet" => crate::bounded_contexts::fan_loyalty::domain::WristbandType::MeetAndGreet,
                _ => return Err(AppError::ValidationError(format!("Invalid wristband type: {}", wristband_type_str))),
            };
            
            Ok(Some(NftWristband {
                id: WristbandId(row.try_get("id")?),
                fan_id: FanId(row.try_get("fan_id")?),
                concert_id: row.try_get("concert_id")?,
                artist_id: row.try_get("artist_id")?,
                wristband_type,
                is_active: row.try_get("is_active")?,
                activated_at: row.try_get("activated_at")?,
                created_at: row.try_get("created_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_wristband_status(&self, wristband_id: &WristbandId, status: &str) -> Result<(), AppError> {
        let is_active = status == "active";
        let activated_at = if is_active { Some(Utc::now()) } else { None };
        
        let query = r#"
            UPDATE nft_wristbands 
            SET is_active = $2, activated_at = $3, updated_at = NOW()
            WHERE id = $1
        "#;
        
        sqlx::query(query)
            .bind(&wristband_id.0)
            .bind(is_active)
            .bind(activated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update wristband status: {}", e)))?;
        
        Ok(())
    }
}

// ============================================================================
// POSTGRES QR CODE REPOSITORY
// ============================================================================

pub struct PostgresQrCodeRepository {
    pool: PgPool,
}

impl PostgresQrCodeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QrCodeRepository for PostgresQrCodeRepository {
    async fn save_qr_code(&self, qr_code: &QrCode) -> Result<(), AppError> {
        let query = r#"
            INSERT INTO qr_codes (
                code, wristband_id, is_valid, expires_at, created_at
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (code) DO UPDATE SET
                is_valid = EXCLUDED.is_valid,
                expires_at = EXCLUDED.expires_at,
                updated_at = NOW()
        "#;
        
        sqlx::query(query)
            .bind(&qr_code.code)
            .bind(&qr_code.wristband_id.0)
            .bind(qr_code.is_valid)
            .bind(qr_code.expires_at)
            .bind(qr_code.created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save QR code: {}", e)))?;
        
        Ok(())
    }

    async fn get_qr_code(&self, code: &str) -> Result<Option<QrCode>, AppError> {
        let query = r#"
            SELECT code, wristband_id, is_valid, created_at, expires_at
            FROM qr_codes 
            WHERE code = $1
        "#;
        
        let row = sqlx::query(query)
            .bind(code)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get QR code: {}", e)))?;
        
        if let Some(row) = row {
            Ok(Some(QrCode {
                code: row.try_get("code")?,
                wristband_id: WristbandId(row.try_get("wristband_id")?),
                is_valid: row.try_get("is_valid")?,
                created_at: row.try_get("created_at")?,
                expires_at: row.try_get("expires_at")?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn invalidate_qr_code(&self, code: &str) -> Result<(), AppError> {
        let query = r#"
            UPDATE qr_codes 
            SET is_valid = FALSE, updated_at = NOW()
            WHERE code = $1
        "#;
        
        sqlx::query(query)
            .bind(code)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to invalidate QR code: {}", e)))?;
        
        Ok(())
    }
}

// ============================================================================
// POSTGRES ZK PROOF REPOSITORY
// ============================================================================

pub struct PostgresZkProofRepository {
    pool: PgPool,
}

impl PostgresZkProofRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ZkProofRepository for PostgresZkProofRepository {
    async fn save_zk_proof(&self, proof_id: Uuid, proof_data: String) -> Result<(), AppError> {
        let query = r#"
            INSERT INTO zk_proofs (proof_id, proof_data, circuit_name)
            VALUES ($1, $2, $3)
            ON CONFLICT (proof_id) DO UPDATE SET
                proof_data = EXCLUDED.proof_data,
                updated_at = NOW()
        "#;
        
        sqlx::query(query)
            .bind(proof_id)
            .bind(proof_data)
            .bind("fan_loyalty_verification")
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save ZK proof: {}", e)))?;
        
        Ok(())
    }

    async fn get_zk_proof(&self, proof_id: Uuid) -> Result<Option<String>, AppError> {
        let query = r#"
            SELECT proof_data FROM zk_proofs WHERE proof_id = $1
        "#;
        
        let row = sqlx::query(query)
            .bind(proof_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get ZK proof: {}", e)))?;
        
        if let Some(row) = row {
            Ok(Some(row.try_get("proof_data")?))
        } else {
            Ok(None)
        }
    }
}

// ============================================================================
// POSTGRES NFT REPOSITORY
// ============================================================================

pub struct PostgresNftRepository {
    pool: PgPool,
}

impl PostgresNftRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NftRepository for PostgresNftRepository {
    async fn mint_nft(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<String, AppError> {
        // This would integrate with real blockchain services
        // For now, we'll simulate the NFT minting process
        
        let transaction_hash = format!("0x{}", Uuid::new_v4().to_string().replace("-", ""));
        
        // Update wristband with NFT information
        let query = r#"
            UPDATE nft_wristbands 
            SET nft_token_id = $2, transaction_hash = $3, updated_at = NOW()
            WHERE id = $1
        "#;
        
        sqlx::query(query)
            .bind(&wristband_id.0)
            .bind(format!("token_{}", wristband_id.0))
            .bind(&transaction_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update wristband with NFT info: {}", e)))?;
        
        Ok(transaction_hash)
    }

    async fn verify_nft_ownership(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<bool, AppError> {
        // This would verify ownership on the blockchain
        // For now, we'll simulate the verification process
        
        let query = r#"
            SELECT COUNT(*) as count
            FROM nft_wristbands 
            WHERE id = $1 AND transaction_hash IS NOT NULL
        "#;
        
        let row = sqlx::query(query)
            .bind(&wristband_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to verify NFT ownership: {}", e)))?;
        
        let count: i64 = row.try_get("count")?;
        Ok(count > 0)
    }
}
