use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::entities::{WristbandId, FanId, WristbandType, NftWristband};
use crate::bounded_contexts::fan_loyalty::domain::services::{
    NftService, NftCreationResult, NftMetadata, NftAttribute,
};
use crate::shared::infrastructure::clients::blockchain_client::BlockchainClient;
use std::sync::Arc;
use async_trait::async_trait;

/// Blockchain implementation of NftService
#[derive(Clone)]
pub struct BlockchainNftService {
    blockchain_client: Arc<BlockchainClient>,
    contract_address: String,
}

impl BlockchainNftService {
    pub fn new(blockchain_client: Arc<BlockchainClient>, contract_address: String) -> Self {
        Self {
            blockchain_client,
            contract_address,
        }
    }

    /// Create NFT metadata (helper)
    fn create_nft_metadata(&self, wristband: &NftWristband) -> NftMetadata {
        NftMetadata {
            name: format!("VibeStream {:?} Wristband", wristband.wristband_type),
            description: format!(
                "Digital wristband for {:?} concert access. Benefits: {}",
                wristband.wristband_type,
                wristband.wristband_type.benefits().join(", ")
            ),
            image: self.generate_wristband_image_url(&wristband.wristband_type),
            attributes: self.create_wristband_attributes(wristband),
            external_url: format!("https://vibestream.com/wristband/{}", wristband.id.0),
            background_color: self.get_wristband_color(&wristband.wristband_type),
        }
    }

    /// Upload metadata to IPFS (helper)
    async fn upload_metadata_to_ipfs(&self, metadata: &NftMetadata) -> Result<String, String> {
        // In a real implementation, this would upload to IPFS
        let json_data = serde_json::to_string(metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;
        
        // Simulate IPFS upload hash
        let ipfs_hash = format!("Qm{}", base64::encode(&json_data)[..46].to_string());
        Ok(ipfs_hash)
    }

    /// Mint on blockchain (helper)
    async fn mint_on_chain(
        &self,
        fan_wallet_address: &str,
        ipfs_hash: &str,
        wristband_id: &WristbandId,
    ) -> Result<String, String> {
         // Verify connection
         let _block = self.blockchain_client.get_block_number().await
            .map_err(|e| format!("Blockchain Error (Connection): {}", e))?;

        if self.blockchain_client.wallet.is_some() {
             // Mock call to confirm signing works
             self.blockchain_client.send_transaction(&self.contract_address, 0).await
                .map_err(|e| format!("Blockchain Error (Tx): {}", e))
        } else {
             // Read-only / Simulation
             Ok(format!(
                "0x{}",
                base64::encode(format!("{}{}{}", fan_wallet_address, ipfs_hash, wristband_id.0))
            ))
        }
    }

    fn generate_token_id(&self, wristband_id: &WristbandId) -> String {
        let uuid_bytes = wristband_id.0.as_bytes();
        let mut token_id = 0u64;
        for (i, &byte) in uuid_bytes.iter().enumerate() {
            token_id += (byte as u64) << (i * 8);
        }
        token_id.to_string()
    }

    fn create_wristband_attributes(&self, wristband: &NftWristband) -> Vec<NftAttribute> {
        vec![
            NftAttribute {
                trait_type: "Type".to_string(),
                value: format!("{:?}", wristband.wristband_type),
            },
            NftAttribute {
                trait_type: "Concert ID".to_string(),
                value: wristband.concert_id.to_string(),
            },
            NftAttribute {
                trait_type: "Artist ID".to_string(),
                value: wristband.artist_id.to_string(),
            },
            NftAttribute {
                trait_type: "Rarity".to_string(),
                value: self.get_wristband_rarity(&wristband.wristband_type),
            },
            NftAttribute {
                trait_type: "Benefits".to_string(),
                value: wristband.wristband_type.benefits().join(", "),
            },
        ]
    }

    fn generate_wristband_image_url(&self, wristband_type: &WristbandType) -> String {
        let image_name = match wristband_type {
            WristbandType::General => "general_wristband",
            WristbandType::VIP => "vip_wristband",
            WristbandType::Backstage => "backstage_wristband",
            WristbandType::MeetAndGreet => "meet_greet_wristband",
        };
        
        format!("https://vibestream.com/images/wristbands/{}.png", image_name)
    }

    fn get_wristband_color(&self, wristband_type: &WristbandType) -> String {
        match wristband_type {
            WristbandType::General => "#3498db", // Blue
            WristbandType::VIP => "#f39c12", // Orange
            WristbandType::Backstage => "#e74c3c", // Red
            WristbandType::MeetAndGreet => "#9b59b6", // Purple
        }.to_string()
    }

    fn get_wristband_rarity(&self, wristband_type: &WristbandType) -> String {
        match wristband_type {
            WristbandType::General => "Common",
            WristbandType::VIP => "Rare",
            WristbandType::Backstage => "Epic",
            WristbandType::MeetAndGreet => "Legendary",
        }.to_string()
    }
}

#[async_trait]
impl NftService for BlockchainNftService {
    async fn create_nft(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<NftCreationResult, String> {
        let metadata = self.create_nft_metadata(wristband);
        let ipfs_hash = self.upload_metadata_to_ipfs(&metadata).await?;
        let transaction_hash = self.mint_on_chain(fan_wallet_address, &ipfs_hash, &wristband.id).await?;
        
        Ok(NftCreationResult {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            nft_token_id: self.generate_token_id(&wristband.id),
            transaction_hash,
            ipfs_hash,
            blockchain_network: format!("ChainID: {}", self.blockchain_client.chain_id),
            contract_address: self.contract_address.clone(),
            created_at: Utc::now(),
        })
    }

    async fn verify_nft_ownership(&self, _fan_wallet_address: &str, _token_id: &str) -> Result<bool, String> {
        // Implementation would query blockchain
        Ok(true)
    }

    async fn transfer_nft(&self, from_address: &str, to_address: &str, token_id: &str) -> Result<String, String> {
        let _block = self.blockchain_client.get_block_number().await
             .map_err(|e| format!("Blockchain Error: {}", e))?;
             
        Ok(format!("0x_transfer_{}_{}_{}", from_address, to_address, token_id))
    }

    async fn get_nft_metadata(&self, _token_id: &str) -> Result<Option<NftMetadata>, String> {
        Ok(None)
    }

    async fn mint_nft_wristband(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<String, String> {
        let result = self.create_nft(wristband, fan_wallet_address).await?;
        Ok(result.transaction_hash)
    }
}

// Supporting structs that match what MockNftService might have need (or are just legacy from my previous edits)
// If NftMintResult was used by WristbandNftService, I keep it or remove it if I remove WristbandNftService

/// NFT mint result (Infrastructure specific, maybe redundant now but keeping for compatibility if needed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMintResult {
    pub transaction_hash: String,
    pub ipfs_hash: String,
    pub nft_token_id: String,
    pub blockchain_network: String,
    pub contract_address: String,
}

/// Wristband NFT result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WristbandNftResult {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub nft_token_id: String,
    pub transaction_hash: String,
    pub ipfs_hash: String,
    pub blockchain_network: String,
    pub contract_address: String,
    pub created_at: DateTime<Utc>,
}

/// Wrapper for use in legacy parts if any
#[derive(Clone)]
pub struct WristbandNftService {
    nft_service: BlockchainNftService,
}

impl WristbandNftService {
    pub fn new(blockchain_client: Arc<BlockchainClient>, contract_address: String) -> Self {
        Self {
            nft_service: BlockchainNftService::new(blockchain_client, contract_address),
        }
    }
    
    /// Create wristband NFT
    pub async fn create_wristband_nft(
        &self,
        wristband: &NftWristband,
        fan_wallet_address: &str,
    ) -> Result<WristbandNftResult, String> {
        let result = self.nft_service.create_nft(wristband, fan_wallet_address).await?;
        
        Ok(WristbandNftResult {
            wristband_id: result.wristband_id,
            fan_id: result.fan_id,
            nft_token_id: result.nft_token_id,
            transaction_hash: result.transaction_hash,
            ipfs_hash: result.ipfs_hash,
            blockchain_network: result.blockchain_network,
            contract_address: result.contract_address,
            created_at: result.created_at,
        })
    }
}
