#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fan_loyalty::domain::{WristbandId, FanId, WristbandType, NftWristband};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_qr_code_service_creation() {
        // Given
        let base_url = "https://vibestream.com".to_string();
        let secret_key = "secret_key".to_string();
        
        // When
        let service = QrCodeService::new(base_url.clone(), secret_key.clone());
        
        // Then
        assert_eq!(service.base_url, base_url);
        assert_eq!(service.secret_key, secret_key);
    }

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
    fn test_qr_code_validation_success() {
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
    fn test_qr_code_validation_expired() {
        // Given
        let service = QrCodeService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let mut qr_data = service.generate_qr_code(&wristband_id, &fan_id);
        qr_data.expires_at = Utc::now() - chrono::Duration::days(1); // Expired
        
        // When
        let result = service.validate_qr_code(&qr_data);
        
        // Then
        assert!(result.is_ok());
        match result.unwrap() {
            ValidationResult::Expired { reason } => {
                assert_eq!(reason, "QR code has expired");
            }
            _ => panic!("Expected expired result"),
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
        assert_eq!(decoded_data.signature, qr_data.signature);
    }

    #[test]
    fn test_wristband_qr_service_creation() {
        // Given
        let base_url = "https://vibestream.com".to_string();
        let secret_key = "secret_key".to_string();
        
        // When
        let service = WristbandQrService::new(base_url.clone(), secret_key.clone());
        
        // Then
        assert_eq!(service.qr_service.base_url, base_url);
        assert_eq!(service.qr_service.secret_key, secret_key);
    }

    #[test]
    fn test_wristband_qr_generation() {
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

    #[test]
    fn test_wristband_qr_validation() {
        // Given
        let service = WristbandQrService::new(
            "https://vibestream.com".to_string(),
            "secret_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let qr_data = service.qr_service.generate_qr_code(&wristband_id, &fan_id);
        
        // When
        let result = service.validate_wristband_qr(&qr_data);
        
        // Then
        assert!(result.is_ok());
        match result.unwrap() {
            WristbandValidationResult::Valid { wristband_id: result_wristband_id, fan_id: result_fan_id, benefits, .. } => {
                assert_eq!(result_wristband_id, wristband_id);
                assert_eq!(result_fan_id, fan_id);
                assert!(!benefits.is_empty());
            }
            _ => panic!("Expected valid result"),
        }
    }

    #[test]
    fn test_nft_service_creation() {
        // Given
        let blockchain_network = "ethereum".to_string();
        let contract_address = "0x1234567890abcdef".to_string();
        let private_key = "private_key".to_string();
        
        // When
        let service = NftService::new(blockchain_network.clone(), contract_address.clone(), private_key.clone());
        
        // Then
        assert_eq!(service.blockchain_network, blockchain_network);
        assert_eq!(service.contract_address, contract_address);
        assert_eq!(service.private_key, private_key);
    }

    #[test]
    fn test_nft_metadata_creation() {
        // Given
        let service = NftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let wristband = NftWristband::new(
            FanId::new(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            WristbandType::VIP,
        );
        
        // When
        let metadata = service.create_nft_metadata(&wristband);
        
        // Then
        assert!(metadata.name.contains("VIP"));
        assert!(metadata.description.contains("VIP"));
        assert!(!metadata.image.is_empty());
        assert!(!metadata.attributes.is_empty());
        assert_eq!(metadata.background_color, "#f39c12");
    }

    #[test]
    fn test_wristband_attributes_creation() {
        // Given
        let service = NftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let wristband = NftWristband::new(
            FanId::new(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            WristbandType::Backstage,
        );
        
        // When
        let attributes = service.create_wristband_attributes(&wristband);
        
        // Then
        assert_eq!(attributes.len(), 5);
        assert!(attributes.iter().any(|attr| attr.trait_type == "Type"));
        assert!(attributes.iter().any(|attr| attr.trait_type == "Rarity"));
        assert!(attributes.iter().any(|attr| attr.value == "Epic"));
    }

    #[test]
    fn test_wristband_nft_service_creation() {
        // Given
        let blockchain_network = "ethereum".to_string();
        let contract_address = "0x1234567890abcdef".to_string();
        let private_key = "private_key".to_string();
        
        // When
        let service = WristbandNftService::new(blockchain_network.clone(), contract_address.clone(), private_key.clone());
        
        // Then
        assert_eq!(service.nft_service.blockchain_network, blockchain_network);
        assert_eq!(service.nft_service.contract_address, contract_address);
        assert_eq!(service.nft_service.private_key, private_key);
    }

    #[test]
    fn test_wristband_nft_creation() {
        // Given
        let service = WristbandNftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let wristband = NftWristband::new(
            FanId::new(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            WristbandType::General,
        );
        
        // When
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            service.create_wristband_nft(&wristband, "0xfan_wallet_address")
        );
        
        // Then
        assert!(result.is_ok());
        let nft_result = result.unwrap();
        assert_eq!(nft_result.wristband_id, wristband.id);
        assert_eq!(nft_result.fan_id, wristband.fan_id);
        assert!(!nft_result.nft_token_id.is_empty());
        assert!(!nft_result.transaction_hash.is_empty());
        assert!(!nft_result.ipfs_hash.is_empty());
        assert_eq!(nft_result.blockchain_network, "ethereum");
        assert_eq!(nft_result.contract_address, "0x1234567890abcdef");
    }

    #[test]
    fn test_wristband_nft_verification() {
        // Given
        let service = WristbandNftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let wristband_id = WristbandId::new();
        let fan_wallet_address = "0xfan_wallet_address";
        
        // When
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            service.verify_wristband_nft(&wristband_id, fan_wallet_address)
        );
        
        // Then
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_nft_service_mint_wristband() {
        // Given
        let service = NftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let wristband = NftWristband::new(
            FanId::new(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            WristbandType::VIP,
        );
        let fan_wallet_address = "0xfan_wallet_address";
        
        // When
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            service.mint_wristband_nft(&wristband, fan_wallet_address)
        );
        
        // Then
        assert!(result.is_ok());
        let mint_result = result.unwrap();
        assert!(!mint_result.transaction_hash.is_empty());
        assert!(!mint_result.ipfs_hash.is_empty());
        assert!(!mint_result.nft_token_id.is_empty());
        assert_eq!(mint_result.blockchain_network, "ethereum");
        assert_eq!(mint_result.contract_address, "0x1234567890abcdef");
    }

    #[test]
    fn test_nft_service_verify_ownership() {
        // Given
        let service = NftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let fan_wallet_address = "0xfan_wallet_address";
        let token_id = "123456789";
        
        // When
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            service.verify_nft_ownership(fan_wallet_address, token_id)
        );
        
        // Then
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_nft_service_transfer_nft() {
        // Given
        let service = NftService::new(
            "ethereum".to_string(),
            "0x1234567890abcdef".to_string(),
            "private_key".to_string(),
        );
        let from_address = "0xfrom_address";
        let to_address = "0xto_address";
        let token_id = "123456789";
        
        // When
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            service.transfer_nft(from_address, to_address, token_id)
        );
        
        // Then
        assert!(result.is_ok());
        let transaction_hash = result.unwrap();
        assert!(!transaction_hash.is_empty());
        assert!(transaction_hash.starts_with("0x"));
    }
}
