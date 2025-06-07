use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use anyhow::{Result, anyhow};

mod wallet;
mod nft;

pub use wallet::WalletClient;
pub use nft::NFTClient;

#[derive(Debug, Clone)]
pub struct NFTMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
}

pub struct SolanaClient {
    wallet_client: WalletClient,
    nft_client: NFTClient,
}

impl SolanaClient {
    pub fn new(keypair: Keypair) -> Self {
        let keypair_copy = Keypair::from_bytes(&keypair.to_bytes()).unwrap();
        let wallet_client = WalletClient::new(keypair);
        let nft_client = NFTClient::new(keypair_copy);
        
        Self {
            wallet_client,
            nft_client,
        }
    }

    pub async fn transfer_sol(&self, to_address: &str, amount: u64) -> Result<String> {
        let bytes = bs58::decode(to_address)
            .into_vec()
            .map_err(|e| anyhow!("Error al decodificar la dirección: {}", e))?;
        
        if bytes.len() != 32 {
            return Err(anyhow!("La dirección debe tener 32 bytes, tiene {}", bytes.len()));
        }
        
        let to_pubkey = Pubkey::new_from_array(bytes.try_into().unwrap());
        self.wallet_client.transfer_sol(&to_pubkey, amount).await?;
        Ok("Transferencia completada con éxito".to_string())
    }

    pub async fn get_wallet_balance(&self) -> Result<u64> {
        self.wallet_client.get_balance().await
    }

    pub async fn mint_nft(&self, _metadata: NFTMetadata) -> Result<String> {
        // TODO: Implementar la lógica de mint NFT
        Ok("NFT minteado".to_string())
    }

    pub async fn transfer_nft(&self, _token_address: &str, _recipient: &str) -> Result<String> {
        // TODO: Implementar la lógica de transferencia de NFT
        Ok("NFT transferido".to_string())
    }

    pub async fn get_nft_info(&self, _token_address: &str) -> Result<NFTMetadata> {
        // TODO: Implementar la lógica para obtener información del NFT
        Ok(NFTMetadata {
            name: "Test NFT".to_string(),
            symbol: "TEST".to_string(),
            description: "Test NFT Description".to_string(),
            image: "https://test.com/image.png".to_string(),
        })
    }
} 