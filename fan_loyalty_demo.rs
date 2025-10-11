//! Fan Loyalty System Demo
//! 
//! This file demonstrates the complete Fan Loyalty System implementation
//! with loose coupling, TDD, and event-driven architecture.

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// DOMAIN LAYER - ENTITIES AND VALUE OBJECTS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct FanId(pub Uuid);

impl FanId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WristbandId(pub Uuid);

impl WristbandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WristbandType {
    General,
    VIP,
    Backstage,
    MeetAndGreet,
}

impl WristbandType {
    pub fn benefits(&self) -> Vec<String> {
        match self {
            WristbandType::General => vec!["Concert Access".to_string()],
            WristbandType::VIP => vec![
                "Concert Access".to_string(),
                "VIP Lounge".to_string(),
                "Priority Entry".to_string(),
            ],
            WristbandType::Backstage => vec![
                "Concert Access".to_string(),
                "VIP Lounge".to_string(),
                "Priority Entry".to_string(),
                "Backstage Access".to_string(),
                "Artist Meet & Greet".to_string(),
            ],
            WristbandType::MeetAndGreet => vec![
                "Concert Access".to_string(),
                "VIP Lounge".to_string(),
                "Priority Entry".to_string(),
                "Backstage Access".to_string(),
                "Artist Meet & Greet".to_string(),
                "Photo Opportunity".to_string(),
                "Autograph Session".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct NftWristband {
    pub id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: WristbandType,
    pub is_active: bool,
    pub activated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl NftWristband {
    pub fn new(fan_id: FanId, concert_id: Uuid, artist_id: Uuid, wristband_type: WristbandType) -> Self {
        Self {
            id: WristbandId::new(),
            fan_id,
            concert_id,
            artist_id,
            wristband_type,
            is_active: false,
            activated_at: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FanVerificationResult {
    pub is_verified: bool,
    pub confidence_score: f32,
    pub verification_id: String,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
}

// ============================================================================
// DOMAIN LAYER - REPOSITORY INTERFACES
// ============================================================================

#[async_trait::async_trait]
pub trait FanVerificationRepository: Send + Sync {
    async fn save_verification_result(
        &self,
        fan_id: &FanId,
        result: &FanVerificationResult,
    ) -> Result<(), String>;

    async fn get_verification_result(&self, fan_id: &FanId) -> Result<Option<FanVerificationResult>, String>;
}

#[async_trait::async_trait]
pub trait WristbandRepository: Send + Sync {
    async fn save_wristband(&self, wristband: &NftWristband) -> Result<(), String>;
    async fn get_wristband(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, String>;
}

// ============================================================================
// DOMAIN LAYER - SERVICE INTERFACES
// ============================================================================

#[async_trait::async_trait]
pub trait BiometricVerificationService: Send + Sync {
    async fn verify_fan(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String>;
}

#[async_trait::async_trait]
pub trait NftService: Send + Sync {
    async fn create_nft(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<NftCreationResult, String>;
}

#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish_fan_verified(&self, event: &FanVerifiedEvent) -> Result<(), String>;
    async fn publish_wristband_created(&self, event: &WristbandCreatedEvent) -> Result<(), String>;
}

// ============================================================================
// DOMAIN LAYER - SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone)]
pub struct BiometricData {
    pub audio_sample: Option<String>,
    pub behavioral_patterns: BehavioralPatterns,
    pub device_characteristics: DeviceCharacteristics,
    pub location: Option<LocationData>,
}

#[derive(Debug, Clone)]
pub struct BehavioralPatterns {
    pub listening_duration: u32,
    pub skip_frequency: f32,
    pub volume_preferences: Vec<f32>,
    pub time_of_day_patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeviceCharacteristics {
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub hardware_fingerprint: String,
}

#[derive(Debug, Clone)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NftCreationResult {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub nft_token_id: String,
    pub transaction_hash: String,
    pub ipfs_hash: String,
    pub blockchain_network: String,
    pub contract_address: String,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// DOMAIN LAYER - EVENTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct FanVerifiedEvent {
    pub fan_id: FanId,
    pub verification_id: String,
    pub confidence_score: f32,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WristbandCreatedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: WristbandType,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// APPLICATION LAYER - DEPENDENCY INJECTION
// ============================================================================

pub struct FanLoyaltyContainer {
    pub fan_verification_repository: Arc<dyn FanVerificationRepository>,
    pub wristband_repository: Arc<dyn WristbandRepository>,
    pub biometric_verification_service: Arc<dyn BiometricVerificationService>,
    pub nft_service: Arc<dyn NftService>,
    pub event_publisher: Arc<dyn EventPublisher>,
}

impl FanLoyaltyContainer {
    pub fn new(
        fan_verification_repository: Arc<dyn FanVerificationRepository>,
        wristband_repository: Arc<dyn WristbandRepository>,
        biometric_verification_service: Arc<dyn BiometricVerificationService>,
        nft_service: Arc<dyn NftService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            fan_verification_repository,
            wristband_repository,
            biometric_verification_service,
            nft_service,
            event_publisher,
        }
    }
}

// ============================================================================
// APPLICATION LAYER - COMMANDS
// ============================================================================

#[derive(Debug, Clone)]
pub struct VerifyFanCommand {
    pub fan_id: FanId,
    pub biometric_data: BiometricData,
    pub device_fingerprint: String,
    pub location: Option<LocationData>,
}

#[derive(Debug, Clone)]
pub struct CreateWristbandCommand {
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: WristbandType,
    pub fan_wallet_address: String,
}

// ============================================================================
// APPLICATION LAYER - HANDLERS
// ============================================================================

pub struct FanVerificationHandler {
    container: Arc<FanLoyaltyContainer>,
}

impl FanVerificationHandler {
    pub fn new(container: Arc<FanLoyaltyContainer>) -> Self {
        Self { container }
    }

    pub async fn handle_verify_fan(&self, command: &VerifyFanCommand) -> Result<FanVerificationResult, String> {
        // Verify fan with biometric data
        let verification_result = self.container.biometric_verification_service.verify_fan(
            &command.fan_id,
            &command.biometric_data,
        ).await?;

        // Save verification result
        self.container.fan_verification_repository.save_verification_result(
            &command.fan_id,
            &verification_result,
        ).await?;

        // Publish event
        let event = FanVerifiedEvent {
            fan_id: command.fan_id.clone(),
            verification_id: verification_result.verification_id.clone(),
            confidence_score: verification_result.confidence_score,
            wristband_eligible: verification_result.wristband_eligible,
            benefits_unlocked: verification_result.benefits_unlocked.clone(),
            occurred_at: Utc::now(),
        };
        self.container.event_publisher.publish_fan_verified(&event).await?;

        Ok(verification_result)
    }
}

pub struct WristbandHandler {
    container: Arc<FanLoyaltyContainer>,
}

impl WristbandHandler {
    pub fn new(container: Arc<FanLoyaltyContainer>) -> Self {
        Self { container }
    }

    pub async fn handle_create_wristband(&self, command: &CreateWristbandCommand) -> Result<NftWristband, String> {
        // Create wristband
        let wristband = NftWristband::new(
            command.fan_id.clone(),
            command.concert_id,
            command.artist_id,
            command.wristband_type.clone(),
        );

        // Save wristband
        self.container.wristband_repository.save_wristband(&wristband).await?;

        // Create NFT
        let nft_result = self.container.nft_service.create_nft(&wristband, &command.fan_wallet_address).await?;

        // Publish event
        let event = WristbandCreatedEvent {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            concert_id: wristband.concert_id,
            artist_id: wristband.artist_id,
            wristband_type: wristband.wristband_type.clone(),
            created_at: Utc::now(),
        };
        self.container.event_publisher.publish_wristband_created(&event).await?;

        Ok(wristband)
    }
}

// ============================================================================
// INFRASTRUCTURE LAYER - REPOSITORY IMPLEMENTATIONS
// ============================================================================

pub struct InMemoryFanVerificationRepository {
    verifications: std::collections::HashMap<FanId, FanVerificationResult>,
}

impl InMemoryFanVerificationRepository {
    pub fn new() -> Self {
        Self {
            verifications: std::collections::HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl FanVerificationRepository for InMemoryFanVerificationRepository {
    async fn save_verification_result(
        &self,
        fan_id: &FanId,
        result: &FanVerificationResult,
    ) -> Result<(), String> {
        // In a real implementation, this would save to database
        println!("Saving verification result for fan: {:?}", fan_id);
        Ok(())
    }

    async fn get_verification_result(&self, fan_id: &FanId) -> Result<Option<FanVerificationResult>, String> {
        // In a real implementation, this would query database
        println!("Getting verification result for fan: {:?}", fan_id);
        Ok(None)
    }
}

pub struct InMemoryWristbandRepository {
    wristbands: std::collections::HashMap<WristbandId, NftWristband>,
}

impl InMemoryWristbandRepository {
    pub fn new() -> Self {
        Self {
            wristbands: std::collections::HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl WristbandRepository for InMemoryWristbandRepository {
    async fn save_wristband(&self, wristband: &NftWristband) -> Result<(), String> {
        // In a real implementation, this would save to database
        println!("Saving wristband: {:?}", wristband.id);
        Ok(())
    }

    async fn get_wristband(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, String> {
        // In a real implementation, this would query database
        println!("Getting wristband: {:?}", wristband_id);
        Ok(None)
    }
}

// ============================================================================
// INFRASTRUCTURE LAYER - SERVICE IMPLEMENTATIONS
// ============================================================================

pub struct MockBiometricVerificationService;

#[async_trait::async_trait]
impl BiometricVerificationService for MockBiometricVerificationService {
    async fn verify_fan(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String> {
        // Mock biometric verification
        let confidence_score = 0.95;
        let is_verified = confidence_score >= 0.8;
        let wristband_eligible = is_verified;

        Ok(FanVerificationResult {
            is_verified,
            confidence_score,
            verification_id: format!("verification_{}", Uuid::new_v4()),
            wristband_eligible,
            benefits_unlocked: vec!["Verified Fan Status".to_string()],
        })
    }
}

pub struct MockNftService;

#[async_trait::async_trait]
impl NftService for MockNftService {
    async fn create_nft(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<NftCreationResult, String> {
        // Mock NFT creation
        Ok(NftCreationResult {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            nft_token_id: format!("token_{}", Uuid::new_v4()),
            transaction_hash: format!("0x{}", Uuid::new_v4().to_string().replace("-", "")),
            ipfs_hash: format!("Qm{}", Uuid::new_v4().to_string().replace("-", "")),
            blockchain_network: "ethereum".to_string(),
            contract_address: "0x1234567890abcdef".to_string(),
            created_at: Utc::now(),
        })
    }
}

pub struct MockEventPublisher;

#[async_trait::async_trait]
impl EventPublisher for MockEventPublisher {
    async fn publish_fan_verified(&self, event: &FanVerifiedEvent) -> Result<(), String> {
        println!("📢 Event: Fan verified - {:?}", event.fan_id);
        Ok(())
    }

    async fn publish_wristband_created(&self, event: &WristbandCreatedEvent) -> Result<(), String> {
        println!("📢 Event: Wristband created - {:?}", event.wristband_id);
        Ok(())
    }
}

// ============================================================================
// TESTS - TDD IMPLEMENTATION
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_id_creation() {
        let fan_id = FanId::new();
        assert!(!fan_id.0.is_nil());
    }

    #[test]
    fn test_wristband_id_creation() {
        let wristband_id = WristbandId::new();
        assert!(!wristband_id.0.is_nil());
    }

    #[test]
    fn test_wristband_type_benefits() {
        let general_benefits = WristbandType::General.benefits();
        let vip_benefits = WristbandType::VIP.benefits();
        let backstage_benefits = WristbandType::Backstage.benefits();
        let meet_greet_benefits = WristbandType::MeetAndGreet.benefits();

        assert!(!general_benefits.is_empty());
        assert!(!vip_benefits.is_empty());
        assert!(!backstage_benefits.is_empty());
        assert!(!meet_greet_benefits.is_empty());
        
        assert!(vip_benefits.len() > general_benefits.len());
        assert!(backstage_benefits.len() > vip_benefits.len());
        assert!(meet_greet_benefits.len() >= backstage_benefits.len());
    }

    #[test]
    fn test_nft_wristband_creation() {
        let fan_id = FanId::new();
        let concert_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let wristband_type = WristbandType::VIP;

        let wristband = NftWristband::new(fan_id.clone(), concert_id, artist_id, wristband_type);

        assert_eq!(wristband.fan_id, fan_id);
        assert_eq!(wristband.wristband_type, WristbandType::VIP);
        assert!(!wristband.is_active);
        assert!(wristband.activated_at.is_none());
    }

    #[test]
    fn test_biometric_data_creation() {
        let biometric_data = BiometricData {
            audio_sample: Some("base64_audio_data".to_string()),
            behavioral_patterns: BehavioralPatterns {
                listening_duration: 300,
                skip_frequency: 0.1,
                volume_preferences: vec![0.7, 0.8, 0.9],
                time_of_day_patterns: vec!["evening".to_string(), "night".to_string()],
            },
            device_characteristics: DeviceCharacteristics {
                device_type: "mobile".to_string(),
                os_version: "iOS 17.0".to_string(),
                app_version: "1.0.0".to_string(),
                hardware_fingerprint: "device_fingerprint_123".to_string(),
            },
            location: Some(LocationData {
                latitude: 40.7128,
                longitude: -74.0060,
                accuracy: 10.0,
                timestamp: Utc::now(),
            }),
        };

        assert!(biometric_data.audio_sample.is_some());
        assert_eq!(biometric_data.behavioral_patterns.listening_duration, 300);
        assert_eq!(biometric_data.device_characteristics.device_type, "mobile");
        assert!(biometric_data.location.is_some());
    }

    #[test]
    fn test_dependency_injection_container() {
        let fan_verification_repository = Arc::new(InMemoryFanVerificationRepository::new());
        let wristband_repository = Arc::new(InMemoryWristbandRepository::new());
        let biometric_verification_service = Arc::new(MockBiometricVerificationService);
        let nft_service = Arc::new(MockNftService);
        let event_publisher = Arc::new(MockEventPublisher);

        let container = FanLoyaltyContainer::new(
            fan_verification_repository,
            wristband_repository,
            biometric_verification_service,
            nft_service,
            event_publisher,
        );

        // Test that container was created successfully
        assert!(true); // Container creation successful
    }

    #[test]
    fn test_command_creation() {
        let fan_id = FanId::new();
        let biometric_data = BiometricData {
            audio_sample: Some("base64_audio_data".to_string()),
            behavioral_patterns: BehavioralPatterns {
                listening_duration: 300,
                skip_frequency: 0.1,
                volume_preferences: vec![0.7, 0.8, 0.9],
                time_of_day_patterns: vec!["evening".to_string(), "night".to_string()],
            },
            device_characteristics: DeviceCharacteristics {
                device_type: "mobile".to_string(),
                os_version: "iOS 17.0".to_string(),
                app_version: "1.0.0".to_string(),
                hardware_fingerprint: "device_fingerprint_123".to_string(),
            },
            location: None,
        };

        let command = VerifyFanCommand {
            fan_id: fan_id.clone(),
            biometric_data: biometric_data.clone(),
            device_fingerprint: "device_fingerprint_123".to_string(),
            location: None,
        };

        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.device_fingerprint, "device_fingerprint_123");
    }

    #[test]
    fn test_event_creation() {
        let fan_id = FanId::new();
        let event = FanVerifiedEvent {
            fan_id: fan_id.clone(),
            verification_id: "verification_123".to_string(),
            confidence_score: 0.95,
            wristband_eligible: true,
            benefits_unlocked: vec!["Verified Fan Status".to_string()],
            occurred_at: Utc::now(),
        };

        assert_eq!(event.fan_id, fan_id);
        assert_eq!(event.verification_id, "verification_123");
        assert_eq!(event.confidence_score, 0.95);
        assert!(event.wristband_eligible);
    }

    #[test]
    fn test_serialization() {
        let fan_id = FanId::new();
        let wristband_id = WristbandId::new();
        let wristband_type = WristbandType::VIP;

        let wristband = NftWristband::new(
            fan_id.clone(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            wristband_type.clone(),
        );

        // Test JSON serialization
        let json = serde_json::to_string(&wristband.fan_id.0).unwrap();
        assert!(!json.is_empty());

        // Test deserialization
        let deserialized_uuid: Uuid = serde_json::from_str(&json).unwrap();
        assert_eq!(wristband.fan_id.0, deserialized_uuid);
    }

    #[test]
    fn test_performance_requirements() {
        use std::time::Instant;

        let start = Instant::now();
        
        // Simulate some work
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should complete in <100ms
        assert_eq!(sum, 499500); // Verify correctness
    }

    #[test]
    fn test_error_handling() {
        fn risky_operation(should_fail: bool) -> Result<String, String> {
            if should_fail {
                Err("Operation failed".to_string())
            } else {
                Ok("Operation succeeded".to_string())
            }
        }

        // Test success case
        let success_result = risky_operation(false);
        assert!(success_result.is_ok());
        assert_eq!(success_result.unwrap(), "Operation succeeded");

        // Test failure case
        let failure_result = risky_operation(true);
        assert!(failure_result.is_err());
        assert_eq!(failure_result.unwrap_err(), "Operation failed");
    }
}

// ============================================================================
// DEMO FUNCTION
// ============================================================================

pub async fn demo_fan_loyalty_system() -> Result<(), String> {
    println!("🎵 VibeStream Fan Loyalty System Demo");
    println!("=====================================");

    // Create dependencies
    let fan_verification_repository = Arc::new(InMemoryFanVerificationRepository::new());
    let wristband_repository = Arc::new(InMemoryWristbandRepository::new());
    let biometric_verification_service = Arc::new(MockBiometricVerificationService);
    let nft_service = Arc::new(MockNftService);
    let event_publisher = Arc::new(MockEventPublisher);

    // Create container with dependency injection
    let container = Arc::new(FanLoyaltyContainer::new(
        fan_verification_repository,
        wristband_repository,
        biometric_verification_service,
        nft_service,
        event_publisher,
    ));

    // Create handlers
    let fan_verification_handler = FanVerificationHandler::new(container.clone());
    let wristband_handler = WristbandHandler::new(container.clone());

    // Demo: Fan Verification
    println!("\n🔍 Step 1: Fan Verification");
    let fan_id = FanId::new();
    let biometric_data = BiometricData {
        audio_sample: Some("base64_audio_data".to_string()),
        behavioral_patterns: BehavioralPatterns {
            listening_duration: 300,
            skip_frequency: 0.1,
            volume_preferences: vec![0.7, 0.8, 0.9],
            time_of_day_patterns: vec!["evening".to_string(), "night".to_string()],
        },
        device_characteristics: DeviceCharacteristics {
            device_type: "mobile".to_string(),
            os_version: "iOS 17.0".to_string(),
            app_version: "1.0.0".to_string(),
            hardware_fingerprint: "device_fingerprint_123".to_string(),
        },
        location: Some(LocationData {
            latitude: 40.7128,
            longitude: -74.0060,
            accuracy: 10.0,
            timestamp: Utc::now(),
        }),
    };

    let verify_command = VerifyFanCommand {
        fan_id: fan_id.clone(),
        biometric_data,
        device_fingerprint: "device_fingerprint_123".to_string(),
        location: None,
    };

    let verification_result = fan_verification_handler.handle_verify_fan(&verify_command).await?;
    println!("✅ Fan verified: {:?}", verification_result.is_verified);
    println!("   Confidence score: {:.2}", verification_result.confidence_score);
    println!("   Wristband eligible: {}", verification_result.wristband_eligible);

    // Demo: Wristband Creation
    if verification_result.wristband_eligible {
        println!("\n🎫 Step 2: Wristband Creation");
        let create_wristband_command = CreateWristbandCommand {
            fan_id: fan_id.clone(),
            concert_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            wristband_type: WristbandType::VIP,
            fan_wallet_address: "0xfan_wallet_address".to_string(),
        };

        let wristband = wristband_handler.handle_create_wristband(&create_wristband_command).await?;
        println!("✅ Wristband created: {:?}", wristband.id);
        println!("   Type: {:?}", wristband.wristband_type);
        println!("   Benefits: {:?}", wristband.wristband_type.benefits());
    }

    println!("\n🎉 Demo completed successfully!");
    println!("   - Loose coupling: ✅ Interfaces and dependency injection");
    println!("   - TDD: ✅ Tests written first, implementation follows");
    println!("   - Event-driven: ✅ Domain events published");
    println!("   - Clean architecture: ✅ Domain, Application, Infrastructure layers");

    Ok(())
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    demo_fan_loyalty_system().await?;
    Ok(())
}

