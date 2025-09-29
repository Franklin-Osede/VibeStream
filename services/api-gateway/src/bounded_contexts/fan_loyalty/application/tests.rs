#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fan_loyalty::domain::{
        FanId, BiometricData, LoyaltyTier, WristbandType, WristbandId,
        BehavioralPatterns, DeviceAuthenticity, LocationConsistency,
    };
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_verify_fan_command_creation() {
        // Given
        let fan_id = FanId::new();
        let biometric_data = create_test_biometric_data();
        
        // When
        let command = VerifyFanCommand::new(fan_id.clone(), biometric_data.clone());
        
        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.biometric_data.audio_presence, biometric_data.audio_presence);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_create_nft_wristband_command_creation() {
        // Given
        let fan_id = FanId::new();
        let artist_id = Uuid::new_v4();
        let concert_id = Uuid::new_v4();
        let wristband_type = WristbandType::VIP;
        
        // When
        let command = CreateNftWristbandCommand::new(
            fan_id.clone(),
            artist_id,
            concert_id,
            wristband_type.clone(),
        );
        
        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.artist_id, artist_id);
        assert_eq!(command.concert_id, concert_id);
        assert_eq!(command.wristband_type, wristband_type);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_activate_nft_wristband_command_creation() {
        // Given
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        
        // When
        let command = ActivateNftWristbandCommand::new(wristband_id.clone(), fan_id.clone());
        
        // Then
        assert_eq!(command.wristband_id, wristband_id);
        assert_eq!(command.fan_id, fan_id);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_use_nft_wristband_command_creation() {
        // Given
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let concert_id = Uuid::new_v4();
        
        // When
        let command = UseNftWristbandCommand::new(wristband_id.clone(), fan_id.clone(), concert_id);
        
        // Then
        assert_eq!(command.wristband_id, wristband_id);
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.concert_id, concert_id);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_add_loyalty_points_command_creation() {
        // Given
        let fan_id = FanId::new();
        let points = 100;
        let reason = "Listening to music".to_string();
        
        // When
        let command = AddLoyaltyPointsCommand::new(fan_id.clone(), points, reason.clone());
        
        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.points, points);
        assert_eq!(command.reason, reason);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_redeem_loyalty_points_command_creation() {
        // Given
        let fan_id = FanId::new();
        let points = 50;
        let reason = "Redeemed for benefits".to_string();
        
        // When
        let command = RedeemLoyaltyPointsCommand::new(fan_id.clone(), points, reason.clone());
        
        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.points, points);
        assert_eq!(command.reason, reason);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_upgrade_loyalty_tier_command_creation() {
        // Given
        let fan_id = FanId::new();
        let new_tier = LoyaltyTier::Gold;
        let reason = "Biometric verification improved".to_string();
        
        // When
        let command = UpgradeLoyaltyTierCommand::new(fan_id.clone(), new_tier.clone(), reason.clone());
        
        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.new_tier, new_tier);
        assert_eq!(command.reason, reason);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_revoke_nft_wristband_command_creation() {
        // Given
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let reason = "Fraudulent activity detected".to_string();
        
        // When
        let command = RevokeNftWristbandCommand::new(wristband_id.clone(), fan_id.clone(), reason.clone());
        
        // Then
        assert_eq!(command.wristband_id, wristband_id);
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.reason, reason);
        assert!(command.timestamp <= Utc::now());
    }

    #[test]
    fn test_get_fan_loyalty_query_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let query = GetFanLoyaltyQuery::new(fan_id.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
    }

    #[test]
    fn test_get_fan_wristbands_query_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let query = GetFanWristbandsQuery::new(fan_id.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
    }

    #[test]
    fn test_get_concert_wristbands_query_creation() {
        // Given
        let concert_id = Uuid::new_v4();
        
        // When
        let query = GetConcertWristbandsQuery::new(concert_id);
        
        // Then
        assert_eq!(query.concert_id, concert_id);
    }

    #[test]
    fn test_get_fan_biometric_score_query_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let query = GetFanBiometricScoreQuery::new(fan_id.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
    }

    #[test]
    fn test_get_fan_loyalty_points_query_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let query = GetFanLoyaltyPointsQuery::new(fan_id.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
    }

    #[test]
    fn test_get_fan_verification_status_query_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let query = GetFanVerificationStatusQuery::new(fan_id.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
    }

    #[test]
    fn test_get_wristband_by_qr_code_query_creation() {
        // Given
        let qr_code = "VIBESTREAM_WRISTBAND_123456789".to_string();
        
        // When
        let query = GetWristbandByQrCodeQuery::new(qr_code.clone());
        
        // Then
        assert_eq!(query.qr_code, qr_code);
    }

    #[test]
    fn test_get_fan_engagement_metrics_query_creation() {
        // Given
        let fan_id = FanId::new();
        
        // When
        let query = GetFanEngagementMetricsQuery::new(fan_id.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
        assert!(query.start_date.is_none());
        assert!(query.end_date.is_none());
    }

    #[test]
    fn test_get_fan_engagement_metrics_query_with_date_range() {
        // Given
        let fan_id = FanId::new();
        let start_date = Utc::now() - chrono::Duration::days(30);
        let end_date = Utc::now();
        
        // When
        let query = GetFanEngagementMetricsQuery::new(fan_id.clone())
            .with_date_range(start_date, end_date);
        
        // Then
        assert_eq!(query.fan_id, fan_id);
        assert!(query.start_date.is_some());
        assert!(query.end_date.is_some());
    }

    #[test]
    fn test_get_artist_fan_statistics_query_creation() {
        // Given
        let artist_id = Uuid::new_v4();
        
        // When
        let query = GetArtistFanStatisticsQuery::new(artist_id);
        
        // Then
        assert_eq!(query.artist_id, artist_id);
        assert!(query.start_date.is_none());
        assert!(query.end_date.is_none());
    }

    #[test]
    fn test_get_artist_fan_statistics_query_with_date_range() {
        // Given
        let artist_id = Uuid::new_v4();
        let start_date = Utc::now() - chrono::Duration::days(30);
        let end_date = Utc::now();
        
        // When
        let query = GetArtistFanStatisticsQuery::new(artist_id)
            .with_date_range(start_date, end_date);
        
        // Then
        assert_eq!(query.artist_id, artist_id);
        assert!(query.start_date.is_some());
        assert!(query.end_date.is_some());
    }

    #[test]
    fn test_get_concert_access_list_query_creation() {
        // Given
        let concert_id = Uuid::new_v4();
        
        // When
        let query = GetConcertAccessListQuery::new(concert_id);
        
        // Then
        assert_eq!(query.concert_id, concert_id);
    }

    #[test]
    fn test_check_fan_eligibility_query_creation() {
        // Given
        let fan_id = FanId::new();
        let wristband_type = WristbandType::VIP;
        
        // When
        let query = CheckFanEligibilityQuery::new(fan_id.clone(), wristband_type.clone());
        
        // Then
        assert_eq!(query.fan_id, fan_id);
        assert_eq!(query.wristband_type, wristband_type);
    }

    // Helper function to create test biometric data
    fn create_test_biometric_data() -> BiometricData {
        BiometricData {
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
        }
    }
}
