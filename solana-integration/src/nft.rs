use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use anyhow::Result;

pub struct NFTClient {
    keypair: Keypair,
}

impl NFTClient {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub async fn mint_nft(&self, _metadata_uri: &str) -> Result<Pubkey> {
        // TODO: Implementar la lógica de mint NFT
        Ok(self.keypair.pubkey())
    }

    pub async fn transfer_nft(&self, _nft_address: &Pubkey, _recipient: &Pubkey) -> Result<()> {
        // TODO: Implementar la lógica de transferencia de NFT
        Ok(())
    }
} 