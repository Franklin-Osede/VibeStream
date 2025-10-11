//! Database Integration Test for Fan Loyalty System
//! 
//! TDD REFACTOR PHASE - Test real database integration

use std::sync::Arc;
use sqlx::PgPool;
use crate::bounded_contexts::fan_loyalty::domain::{FanId, WristbandId, WristbandType, FanVerificationResult, NftWristband, QrCode};
use crate::bounded_contexts::fan_loyalty::infrastructure::postgres_repositories::{
    PostgresFanVerificationRepository, PostgresWristbandRepository, 
    PostgresQrCodeRepository, PostgresZkProofRepository, PostgresNftRepository
};

/// Test database integration - TDD REFACTOR PHASE
#[tokio::test]
async fn test_database_integration() {
    // TDD REFACTOR PHASE: Test real database integration
    
    // Create test database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/vibestream_test".to_string());
    
    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    // Create repositories
    let fan_verification_repo = Arc::new(PostgresFanVerificationRepository::new(pool.clone()));
    let wristband_repo = Arc::new(PostgresWristbandRepository::new(pool.clone()));
    let qr_code_repo = Arc::new(PostgresQrCodeRepository::new(pool.clone()));
    let zk_proof_repo = Arc::new(PostgresZkProofRepository::new(pool.clone()));
    let nft_repo = Arc::new(PostgresNftRepository::new(pool.clone()));
    
    // Test 1: Fan Verification Repository
    println!("📋 Test 1: Fan Verification Repository");
    let fan_id = FanId::new();
    let verification_result = FanVerificationResult {
        is_verified: true,
        confidence_score: 0.95,
        verification_id: format!("verification_{}", fan_id.to_string()),
        wristband_eligible: true,
        benefits_unlocked: vec!["Verified Fan Status".to_string()],
    };
    
    // Save verification result
    fan_verification_repo.save_verification_result(&fan_id, &verification_result).await
        .expect("Failed to save verification result");
    
    // Get verification result
    let saved_result = fan_verification_repo.get_verification_result(&fan_id).await
        .expect("Failed to get verification result");
    
    assert!(saved_result.is_some());
    let result = saved_result.unwrap();
    assert!(result.is_verified);
    assert!(result.wristband_eligible);
    assert!(result.confidence_score > 0.9);
    println!("✅ Fan verification repository working");
    
    // Test 2: Wristband Repository
    println!("📋 Test 2: Wristband Repository");
    let wristband = NftWristband::new(
        fan_id.clone(),
        "concert_123".to_string(),
        "artist_456".to_string(),
        WristbandType::VIP,
    );
    
    // Save wristband
    wristband_repo.save_wristband(&wristband).await
        .expect("Failed to save wristband");
    
    // Get wristband
    let saved_wristband = wristband_repo.get_wristband(&wristband.id).await
        .expect("Failed to get wristband");
    
    assert!(saved_wristband.is_some());
    let wristband = saved_wristband.unwrap();
    assert_eq!(wristband.fan_id, fan_id);
    assert_eq!(wristband.concert_id, "concert_123");
    assert_eq!(wristband.artist_id, "artist_456");
    println!("✅ Wristband repository working");
    
    // Test 3: QR Code Repository
    println!("📋 Test 3: QR Code Repository");
    let qr_code = QrCode::new(wristband.id.clone());
    
    // Save QR code
    qr_code_repo.save_qr_code(&qr_code).await
        .expect("Failed to save QR code");
    
    // Get QR code
    let saved_qr_code = qr_code_repo.get_qr_code(&qr_code.code).await
        .expect("Failed to get QR code");
    
    assert!(saved_qr_code.is_some());
    let qr_code = saved_qr_code.unwrap();
    assert_eq!(qr_code.wristband_id, wristband.id);
    assert!(qr_code.is_valid);
    println!("✅ QR code repository working");
    
    // Test 4: ZK Proof Repository
    println!("📋 Test 4: ZK Proof Repository");
    let proof_id = uuid::Uuid::new_v4();
    let proof_data = "mock_proof_data".to_string();
    
    // Save ZK proof
    zk_proof_repo.save_zk_proof(proof_id, proof_data.clone()).await
        .expect("Failed to save ZK proof");
    
    // Get ZK proof
    let saved_proof = zk_proof_repo.get_zk_proof(proof_id).await
        .expect("Failed to get ZK proof");
    
    assert!(saved_proof.is_some());
    assert_eq!(saved_proof.unwrap(), proof_data);
    println!("✅ ZK proof repository working");
    
    // Test 5: NFT Repository
    println!("📋 Test 5: NFT Repository");
    let fan_wallet_address = "0xfan_wallet_address";
    
    // Mint NFT
    let transaction_hash = nft_repo.mint_nft(&wristband.id, fan_wallet_address).await
        .expect("Failed to mint NFT");
    
    assert!(!transaction_hash.is_empty());
    assert!(transaction_hash.starts_with("0x"));
    println!("✅ NFT minting working");
    
    // Verify NFT ownership
    let is_owner = nft_repo.verify_nft_ownership(&wristband.id, fan_wallet_address).await
        .expect("Failed to verify NFT ownership");
    
    assert!(is_owner);
    println!("✅ NFT ownership verification working");
    
    println!("\n🎉 All database integration tests passed!");
    println!("🎯 TDD REFACTOR PHASE: Database integration working!");
}

/// Test database transactions - TDD REFACTOR PHASE
#[tokio::test]
async fn test_database_transactions() {
    // TDD REFACTOR PHASE: Test database transactions
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/vibestream_test".to_string());
    
    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    // Test transaction rollback
    let mut tx = pool.begin().await
        .expect("Failed to begin transaction");
    
    let fan_id = FanId::new();
    let verification_result = FanVerificationResult {
        is_verified: true,
        confidence_score: 0.95,
        verification_id: format!("verification_{}", fan_id.to_string()),
        wristband_eligible: true,
        benefits_unlocked: vec!["Verified Fan Status".to_string()],
    };
    
    // Save verification result in transaction
    let fan_verification_repo = PostgresFanVerificationRepository::new(pool.clone());
    fan_verification_repo.save_verification_result(&fan_id, &verification_result).await
        .expect("Failed to save verification result");
    
    // Rollback transaction
    tx.rollback().await
        .expect("Failed to rollback transaction");
    
    // Verify that the data was not saved
    let saved_result = fan_verification_repo.get_verification_result(&fan_id).await
        .expect("Failed to get verification result");
    
    assert!(saved_result.is_none());
    println!("✅ Database transactions working");
    
    println!("\n🎉 All database transaction tests passed!");
}

/// Test database performance - TDD REFACTOR PHASE
#[tokio::test]
async fn test_database_performance() {
    // TDD REFACTOR PHASE: Test database performance
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/vibestream_test".to_string());
    
    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    let fan_verification_repo = Arc::new(PostgresFanVerificationRepository::new(pool.clone()));
    
    // Test bulk operations
    let start_time = std::time::Instant::now();
    
    for i in 0..100 {
        let fan_id = FanId::new();
        let verification_result = FanVerificationResult {
            is_verified: true,
            confidence_score: 0.95,
            verification_id: format!("verification_{}_{}", fan_id.to_string(), i),
            wristband_eligible: true,
            benefits_unlocked: vec!["Verified Fan Status".to_string()],
        };
        
        fan_verification_repo.save_verification_result(&fan_id, &verification_result).await
            .expect("Failed to save verification result");
    }
    
    let duration = start_time.elapsed();
    println!("✅ Bulk operations completed in {:?}", duration);
    
    // Verify performance is reasonable (less than 1 second for 100 operations)
    assert!(duration.as_secs() < 1);
    
    println!("\n🎉 All database performance tests passed!");
}
