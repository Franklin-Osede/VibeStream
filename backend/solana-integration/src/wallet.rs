use anyhow::Result;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
};

pub struct WalletService {
    keypair: Keypair,
}

impl WalletService {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub fn get_public_key(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    pub async fn get_balance(&self) -> Result<u64> {
        // TODO: Implementar get_balance
        todo!("Implementar get_balance")
    }

    pub async fn transfer_sol(&self, recipient: &Pubkey, amount: u64) -> Result<()> {
        // TODO: Implementar transfer_sol
        todo!("Implementar transfer_sol")
    }
} 