use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
use vibestream_types::*;

pub struct SolanaClient {
    rpc_client: RpcClient,
    keypair: Keypair,
}

impl SolanaClient {
    pub fn new(rpc_url: String, private_key_bytes: Vec<u8>) -> Result<Self> {
        let rpc_client = RpcClient::new(rpc_url);
        
        let keypair = Keypair::from_bytes(&private_key_bytes)
            .map_err(|e| VibeStreamError::Validation { 
                message: format!("Invalid private key: {}", e) 
            })?;
        
        Ok(Self {
            rpc_client,
            keypair,
        })
    }
    
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        self.rpc_client
            .get_balance(pubkey)
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to get balance: {}", e) 
            })
    }
    
    pub async fn send_transaction(&self, transaction: &Transaction) -> Result<Signature> {
        self.rpc_client
            .send_and_confirm_transaction(transaction)
            .map_err(|e| VibeStreamError::Network { 
                message: format!("Failed to send transaction: {}", e) 
            })
    }
    
    pub fn get_pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
} 