use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    commitment_config::{CommitmentConfig, CommitmentLevel},
};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};
use anyhow::{Result, anyhow, Context};
use std::time::{Duration, Instant};
use std::thread;
use log::{info, error, debug, warn};
use crate::metrics::WalletMetrics;

pub struct WalletClient {
    keypair: Keypair,
    client: RpcClient,
    metrics: WalletMetrics,
}

impl WalletClient {
    pub fn new(keypair: Keypair) -> Self {
        info!("Creando nuevo WalletClient...");
        // Conexión a Solana devnet con commitment máximo
        let client = RpcClient::new_with_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::finalized()
        );
        debug!("Cliente RPC inicializado con commitment finalized");
        Self { 
            keypair, 
            client,
            metrics: WalletMetrics::new(),
        }
    }

    pub fn get_address(&self) -> String {
        self.keypair.pubkey().to_string()
    }

    pub fn get_metrics(&self) -> &WalletMetrics {
        &self.metrics
    }

    pub async fn get_balance(&self) -> Result<u64> {
        debug!("Consultando balance para {}", self.get_address());
        let balance = self.client
            .get_balance_with_commitment(
                &self.keypair.pubkey(),
                CommitmentConfig::finalized()
            )
            .with_context(|| format!("Error al obtener balance para {}", self.get_address()))?
            .value;
        
        info!("Balance obtenido: {} SOL", balance as f64 / 1_000_000_000.0);
        Ok(balance)
    }

    pub async fn transfer(&mut self, to_address: &str, amount: u64) -> Result<String> {
        info!("Iniciando transferencia de {} lamports a {}", amount, to_address);
        let start_time = Instant::now();
        self.metrics.record_transaction_attempt();
        
        // Decodificar la dirección de destino
        let to_pubkey = Pubkey::try_from(bs58::decode(to_address)
            .into_vec()
            .with_context(|| format!("Error al decodificar la dirección: {}", to_address))?
            .as_slice())
            .with_context(|| "Dirección inválida")?;

        // Verificar balance antes de transferir
        let balance = self.get_balance().await?;
        if balance < amount {
            self.metrics.record_transaction_failure();
            return Err(anyhow!(
                "Balance insuficiente. Tienes {} SOL, necesitas {} SOL",
                balance as f64 / 1_000_000_000.0,
                amount as f64 / 1_000_000_000.0
            ));
        }

        // Crear la instrucción de transferencia
        let instruction = system_instruction::transfer(
            &self.keypair.pubkey(),
            &to_pubkey,
            amount,
        );
        debug!("Instrucción de transferencia creada");

        // Obtener el último hash del bloque
        let recent_blockhash = self.client
            .get_latest_blockhash()
            .with_context(|| "Error al obtener el último blockhash")?;
        debug!("Blockhash obtenido: {}", recent_blockhash);

        // Crear y firmar la transacción
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            recent_blockhash,
        );
        debug!("Transacción creada y firmada");

        // Enviar y confirmar la transacción con configuración más estricta
        info!("Enviando transacción...");
        let signature = self.client
            .send_and_confirm_transaction_with_spinner_and_config(
                &transaction,
                CommitmentConfig::finalized(),
                RpcSendTransactionConfig {
                    skip_preflight: false,
                    preflight_commitment: Some(CommitmentLevel::Finalized),
                    encoding: None,
                    max_retries: Some(10),
                    min_context_slot: None,
                },
            )
            .with_context(|| "Error al enviar la transacción")?;
        
        info!("Transacción enviada con éxito. Firma: {}", signature);

        // Esperar un poco más para asegurar la propagación
        thread::sleep(Duration::from_secs(1));
        debug!("Esperando confirmación adicional...");

        // Verificar que la transacción se confirmó
        let confirmation = self.client
            .get_signature_status_with_commitment(&signature, CommitmentConfig::finalized())
            .with_context(|| format!("Error al verificar la transacción: {}", signature))?;

        let elapsed_time = start_time.elapsed();

        match confirmation {
            Some(Ok(_)) => {
                info!("Transacción confirmada exitosamente");
                self.metrics.record_transaction_success(amount, elapsed_time);
                Ok(signature.to_string())
            },
            Some(Err(e)) => {
                error!("La transacción falló: {:?}", e);
                self.metrics.record_transaction_failure();
                Err(anyhow!("La transacción falló: {:?}", e))
            },
            None => {
                warn!("La transacción no se encontró después de la confirmación");
                self.metrics.record_transaction_failure();
                Err(anyhow!("La transacción no se encontró"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_keypair() -> Keypair {
        Keypair::new()
    }

    #[tokio::test]
    async fn test_wallet_creation() {
        let keypair = create_test_keypair();
        let wallet = WalletClient::new(keypair);
        assert!(wallet.keypair.to_bytes().len() == 64, "El keypair debe tener 64 bytes");
        assert_eq!(wallet.get_metrics().get_success_rate(), 0.0, "Las métricas deben iniciar en 0");
    }

    #[tokio::test]
    async fn test_get_balance() {
        let wallet = WalletClient::new(create_test_keypair());
        let balance = wallet.get_balance().await;
        assert!(balance.is_ok(), "Debe poder obtener el balance");
    }

    #[tokio::test]
    async fn test_transfer_insufficient_funds() {
        let mut wallet = WalletClient::new(create_test_keypair());
        let recipient = create_test_keypair();
        let amount = 100_000_000; // 0.1 SOL
        
        let result = wallet.transfer(&recipient.pubkey().to_string(), amount).await;
        assert!(result.is_err(), "Debe fallar por falta de fondos");
        assert!(result.unwrap_err().to_string().contains("Balance insuficiente"), 
            "El error debe indicar balance insuficiente");
        assert_eq!(wallet.get_metrics().failed_transactions, 1, "Debe registrar la transacción fallida");
    }

    #[tokio::test]
    async fn test_invalid_address() {
        let mut wallet = WalletClient::new(create_test_keypair());
        let result = wallet.transfer("invalid_address", 100).await;
        assert!(result.is_err(), "Debe fallar con dirección inválida");
        assert!(result.unwrap_err().to_string().contains("Error al decodificar"), 
            "El error debe indicar problema de decodificación");
        assert_eq!(wallet.get_metrics().failed_transactions, 1, "Debe registrar la transacción fallida");
    }
}