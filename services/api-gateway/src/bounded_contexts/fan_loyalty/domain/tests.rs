#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fan_loyalty::domain::{
        aggregates::*, entities::*, services::*,
    };
    use chrono::Utc;

    #[test]
    fn test_fan_loyalty_aggregate_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let aggregate = FanLoyaltyAggregate::new(fan_id.clone());
        
        // Then
        assert_eq!(aggregate.fan_id, fan_id);
        assert_eq!(aggregate.loyalty_tier, LoyaltyTier::Bronze);
        assert_eq!(aggregate.verification_status, VerificationStatus::Pending);
        assert_eq!(aggregate.biometric_score.value, 0.0);
    }

    #[test]
    fn test_biometric_score_update() {
        // Given
        let fan_id = FanId::new();
        let mut aggregate = FanLoyaltyAggregate::new(fan_id);
        
        // When
        let result = aggregate.update_biometric_score(0.8);
        
        // Then
        assert!(result.is_ok());
        assert_eq!(aggregate.biometric_score.value, 0.8);
        assert_eq!(aggregate.loyalty_tier, LoyaltyTier::Gold);
    }

    #[test]
    fn test_biometric_score_update_invalid() {
        // Given
        let fan_id = FanId::new();
        let mut aggregate = FanLoyaltyAggregate::new(fan_id);
        
        // When
        let result = aggregate.update_biometric_score(1.5);
        
        // Then
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Biometric score must be between 0.0 and 1.0");
    }

    #[test]
    fn test_loyalty_tier_calculation() {
        // Given
        let fan_id = FanId::new();
        let mut aggregate = FanLoyaltyAggregate::new(fan_id);
        
        // Test Bronze tier
        aggregate.update_biometric_score(0.3).unwrap();
        assert_eq!(aggregate.loyalty_tier, LoyaltyTier::Bronze);
        
        // Test Silver tier
        aggregate.update_biometric_score(0.6).unwrap();
        assert_eq!(aggregate.loyalty_tier, LoyaltyTier::Silver);
        
        // Test Gold tier
        aggregate.update_biometric_score(0.8).unwrap();
        assert_eq!(aggregate.loyalty_tier, LoyaltyTier::Gold);
        
        // Test Platinum tier
        aggregate.update_biometric_score(0.95).unwrap();
        assert_eq!(aggregate.loyalty_tier, LoyaltyTier::Platinum);
    }

    #[test]
    fn test_fan_verification_success() {
        // Given
        let fan_id = FanId::new();
        let mut aggregate = FanLoyaltyAggregate::new(fan_id.clone());
        
        let biometric_data = BiometricData {
            audio_presence: true,
            behavioral_patterns: BehavioralPatterns {
                listening_duration: 120.0,
                skip_rate: 0.2,
                repeat_rate: 0.3,
                interaction_frequency: 0.1,
            },
            device_authenticity: DeviceAuthenticity {
                device_id: "device_123".to_string(),
                app_version: "1.0.0".to_string(),
                os_version: "iOS 15.0".to_string(),
                is_emulator: false,
                is_rooted: false,
            },
            location_consistency: LocationConsistency {
                current_location: Some((40.7128, -74.0060)),
                previous_locations: vec![(40.7128, -74.0060)],
                location_variance: 0.1,
            },
        };
        
        // When
        let result = aggregate.verify_fan(biometric_data);
        
        // Then
        assert!(result.is_ok());
        match result.unwrap() {
            VerificationResult::Verified { fan_id: result_fan_id, loyalty_tier, biometric_score } => {
                assert_eq!(result_fan_id, fan_id);
                assert!(biometric_score.value >= 0.5);
                assert!(matches!(loyalty_tier, LoyaltyTier::Silver | LoyaltyTier::Gold | LoyaltyTier::Platinum));
            }
            _ => panic!("Expected verified result"),
        }
    }

    #[test]
    fn test_fan_verification_failure() {
        // Given
        let fan_id = FanId::new();
        let mut aggregate = FanLoyaltyAggregate::new(fan_id.clone());
        
        let biometric_data = BiometricData {
            audio_presence: false,
            behavioral_patterns: BehavioralPatterns {
                listening_duration: 5.0,
                skip_rate: 0.9,
                repeat_rate: 0.0,
                interaction_frequency: 0.0,
            },
            device_authenticity: DeviceAuthenticity {
                device_id: "device_123".to_string(),
                app_version: "1.0.0".to_string(),
                os_version: "iOS 15.0".to_string(),
                is_emulator: true,
                is_rooted: false,
            },
            location_consistency: LocationConsistency {
                current_location: None,
                previous_locations: vec![],
                location_variance: 0.0,
            },
        };
        
        // When
        let result = aggregate.verify_fan(biometric_data);
        
        // Then
        assert!(result.is_ok());
        match result.unwrap() {
            VerificationResult::Failed { reason } => {
                assert_eq!(reason, "Insufficient biometric verification");
            }
            _ => panic!("Expected failed result"),
        }
    }

    #[test]
    fn test_behavioral_patterns_consistency() {
        // Given - Consistent patterns
        let consistent_patterns = BehavioralPatterns {
            listening_duration: 120.0,
            skip_rate: 0.2,
            repeat_rate: 0.3,
            interaction_frequency: 0.1,
        };
        
        // When
        let is_consistent = consistent_patterns.is_consistent();
        
        // Then
        assert!(is_consistent);
        
        // Given - Inconsistent patterns
        let inconsistent_patterns = BehavioralPatterns {
            listening_duration: 5.0,
            skip_rate: 0.9,
            repeat_rate: 0.0,
            interaction_frequency: 0.0,
        };
        
        // When
        let is_consistent = inconsistent_patterns.is_consistent();
        
        // Then
        assert!(!is_consistent);
    }

    #[test]
    fn test_device_authenticity_verification() {
        // Given - Authentic device
        let authentic_device = DeviceAuthenticity {
            device_id: "device_123".to_string(),
            app_version: "1.0.0".to_string(),
            os_version: "iOS 15.0".to_string(),
            is_emulator: false,
            is_rooted: false,
        };
        
        // When
        let is_verified = authentic_device.is_verified();
        
        // Then
        assert!(is_verified);
        
        // Given - Emulator device
        let emulator_device = DeviceAuthenticity {
            device_id: "device_123".to_string(),
            app_version: "1.0.0".to_string(),
            os_version: "iOS 15.0".to_string(),
            is_emulator: true,
            is_rooted: false,
        };
        
        // When
        let is_verified = emulator_device.is_verified();
        
        // Then
        assert!(!is_verified);
    }

    #[test]
    fn test_location_consistency() {
        // Given - Consistent location
        let consistent_location = LocationConsistency {
            current_location: Some((40.7128, -74.0060)),
            previous_locations: vec![(40.7128, -74.0060), (40.7129, -74.0061)],
            location_variance: 0.1,
        };
        
        // When
        let is_reasonable = consistent_location.is_reasonable();
        
        // Then
        assert!(is_reasonable);
        
        // Given - Inconsistent location (too far)
        let inconsistent_location = LocationConsistency {
            current_location: Some((40.7128, -74.0060)),
            previous_locations: vec![(50.0000, -100.0000)], // Very far
            location_variance: 0.1,
        };
        
        // When
        let is_reasonable = inconsistent_location.is_reasonable();
        
        // Then
        assert!(!is_reasonable);
    }

    #[test]
    fn test_loyalty_tier_benefits() {
        // Test Bronze benefits
        let bronze_benefits = LoyaltyTier::Bronze.benefits();
        assert_eq!(bronze_benefits.len(), 1);
        assert!(bronze_benefits.contains(&"Basic streaming access".to_string()));
        
        // Test Platinum benefits
        let platinum_benefits = LoyaltyTier::Platinum.benefits();
        assert_eq!(platinum_benefits.len(), 6);
        assert!(platinum_benefits.contains(&"VIP concert wristbands".to_string()));
    }

    #[test]
    fn test_nft_wristband_creation() {
        // Given
        let fan_id = FanId::new();
        let artist_id = uuid::Uuid::new_v4();
        let concert_id = uuid::Uuid::new_v4();
        let wristband_type = WristbandType::VIP;
        
        // When
        let wristband = NftWristband::new(fan_id, artist_id, concert_id, wristband_type);
        
        // Then
        assert_eq!(wristband.fan_id, fan_id);
        assert_eq!(wristband.artist_id, artist_id);
        assert_eq!(wristband.concert_id, concert_id);
        assert_eq!(wristband.wristband_type, WristbandType::VIP);
        assert_eq!(wristband.status, WristbandStatus::Active);
        assert!(!wristband.qr_code.is_empty());
        assert!(wristband.is_valid());
    }

    #[test]
    fn test_nft_wristband_validation() {
        // Given
        let fan_id = FanId::new();
        let artist_id = uuid::Uuid::new_v4();
        let concert_id = uuid::Uuid::new_v4();
        let wristband_type = WristbandType::General;
        let wristband = NftWristband::new(fan_id, artist_id, concert_id, wristband_type);
        
        // When
        let wristband_service = NftWristbandService;
        let result = wristband_service.validate_wristband(&wristband);
        
        // Then
        assert!(result.is_ok());
        match result.unwrap() {
            ValidationResult::Valid { wristband_id, fan_id, benefits } => {
                assert_eq!(wristband_id, wristband.id);
                assert_eq!(fan_id, wristband.fan_id);
                assert!(!benefits.is_empty());
            }
            _ => panic!("Expected valid result"),
        }
    }

    #[test]
    fn test_biometric_verification_service() {
        // Given
        let service = BiometricVerificationService;
        let biometric_data = BiometricData {
            audio_presence: true,
            behavioral_patterns: BehavioralPatterns {
                listening_duration: 120.0,
                skip_rate: 0.2,
                repeat_rate: 0.3,
                interaction_frequency: 0.1,
            },
            device_authenticity: DeviceAuthenticity {
                device_id: "device_123".to_string(),
                app_version: "1.0.0".to_string(),
                os_version: "iOS 15.0".to_string(),
                is_emulator: false,
                is_rooted: false,
            },
            location_consistency: LocationConsistency {
                current_location: Some((40.7128, -74.0060)),
                previous_locations: vec![(40.7128, -74.0060)],
                location_variance: 0.1,
            },
        };
        
        // When
        let score = service.calculate_biometric_score(&biometric_data);
        
        // Then
        assert!(score.value > 0.0);
        assert!(score.value <= 1.0);
    }

    #[test]
    fn test_loyalty_calculation_service() {
        // Given
        let service = LoyaltyCalculationService;
        
        // When
        let tier = service.calculate_loyalty_tier(0.8);
        
        // Then
        assert_eq!(tier, LoyaltyTier::Gold);
        
        // When
        let points = service.calculate_loyalty_points(10.0, 0.2);
        
        // Then
        assert_eq!(points, 99); // (10 * 10) - (0.2 * 0.5) = 100 - 0.1 = 99
    }

    #[test]
    fn test_wristband_type_benefits() {
        // Test General wristband benefits
        let general_benefits = WristbandType::General.benefits();
        assert_eq!(general_benefits.len(), 2);
        assert!(general_benefits.contains(&"Concert access".to_string()));
        
        // Test VIP wristband benefits
        let vip_benefits = WristbandType::VIP.benefits();
        assert_eq!(vip_benefits.len(), 4);
        assert!(vip_benefits.contains(&"VIP seating".to_string()));
        
        // Test Backstage wristband benefits
        let backstage_benefits = WristbandType::Backstage.benefits();
        assert_eq!(backstage_benefits.len(), 4);
        assert!(backstage_benefits.contains(&"Backstage access".to_string()));
    }
}
