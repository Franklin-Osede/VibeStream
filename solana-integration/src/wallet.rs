use solana_sdk::{
    signature::Keypair,
};
use anyhow::Result;

#[allow(dead_code)]
pub struct WalletClient {
    keypair: Keypair,
}

impl WalletClient {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub async fn get_balance(&self) -> Result<u64> {
        // TODO: Implementar la obtención del balance
        Ok(1000)
    }

    pub async fn transfer(&self, _to: &str, _amount: u64) -> Result<String> {
        // TODO: Implementar la transferencia
        Ok("tx_hash".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::Signer;
    
    fn create_test_keypair() -> Keypair {
        Keypair::new()
    }

    #[tokio::test]
    async fn test_wallet_creation() {
        let keypair = create_test_keypair();
        let wallet = WalletClient::new(keypair);
        assert!(wallet.keypair.to_bytes().len() == 64, "El keypair debe tener 64 bytes");
    }

    #[tokio::test]
    async fn test_get_balance() {
        let wallet = WalletClient::new(create_test_keypair());
        let balance = wallet.get_balance().await.unwrap();
        assert!(balance > 0, "El balance debe ser mayor que 0");
    }

    #[tokio::test]
    async fn test_transfer() {
        let wallet = WalletClient::new(create_test_keypair());
        let recipient = create_test_keypair();
        let amount = 100;
        
        let result = wallet.transfer(&recipient.pubkey().to_string(), amount).await;
        assert!(result.is_ok(), "La transferencia debe completarse sin errores");
        
        let tx_hash = result.unwrap();
        assert!(!tx_hash.is_empty(), "El hash de la transacción no debe estar vacío");
    }
}