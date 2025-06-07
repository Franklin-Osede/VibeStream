use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use anyhow::Result;

pub struct WalletClient {
    keypair: Keypair,
}

impl WalletClient {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub async fn get_balance(&self) -> Result<u64> {
        // TODO: Implementar la lógica para obtener el balance
        Ok(0)
    }

    pub async fn transfer_sol(&self, _recipient: &Pubkey, _amount: u64) -> Result<()> {
        // TODO: Implementar la lógica de transferencia de SOL
        Ok(())
    }
}