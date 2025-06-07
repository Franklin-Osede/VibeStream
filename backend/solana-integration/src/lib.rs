use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
};

pub struct SolanaClient {
    client: RpcClient,
    payer: Keypair,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
}

impl SolanaClient {
    pub fn new(rpc_url: &str, payer: Keypair) -> Self {
        Self {
            client: RpcClient::new(rpc_url.to_string()),
            payer,
        }
    }

    pub async fn get_wallet_balance(&self) -> Result<f64> {
        let balance = self.client.get_balance(&self.payer.pubkey())?;
        Ok(balance as f64 / 1_000_000_000.0) // Convert lamports to SOL
    }

    pub async fn transfer_sol(&self, to_address: &str, amount: u64) -> Result<String> {
        let to_pubkey = Pubkey::from_str(to_address)?;
        let instruction = system_instruction::transfer(
            &self.payer.pubkey(),
            &to_pubkey,
            amount * 1_000_000_000, // Convert SOL to lamports
        );

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            recent_blockhash,
        );

        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }

    pub async fn mint_nft(&self, metadata: NFTMetadata) -> Result<String> {
        // TODO: Implementar la lógica de mint NFT
        todo!("Implementar mint_nft")
    }

    pub async fn transfer_nft(&self, token_address: &str, recipient: &str) -> Result<String> {
        // TODO: Implementar la lógica de transferencia
        todo!("Implementar transfer_nft")
    }

    pub async fn get_nft_info(&self, token_address: &str) -> Result<NFTMetadata> {
        // TODO: Implementar la obtención de metadata
        todo!("Implementar get_nft_info")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_client() {
        let keypair = Keypair::new();
        let client = SolanaClient::new("https://api.devnet.solana.com", keypair);
        assert!(client.client.get_version().is_ok());
    }
}

pub mod nft;
pub mod wallet;

pub use nft::NFTService;
pub use wallet::WalletService; 