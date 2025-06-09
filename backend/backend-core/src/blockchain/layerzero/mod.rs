use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ethers::{
    core::types::{Address, U256},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};
use std::str::FromStr;
use std::sync::Arc;

use crate::blockchain::{
    ethereum::EthereumClient,
    solana::SolanaClient,
    common::{TransactionInfo, TransactionStatus},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainMessage {
    pub source_chain: ChainId,
    pub destination_chain: ChainId,
    pub payload: Vec<u8>,
    pub gas_limit: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChainId {
    Ethereum = 1,
    BSC = 56,
    Polygon = 137,
    Avalanche = 43114,
    Solana = 1399811149,
}

pub struct LayerZeroClient {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
    endpoint_contract: Address,
}

#[async_trait]
pub trait CrossChainMessenger {
    async fn send_cross_chain_message(
        &self,
        destination_chain_id: u16,
        destination_address: &str,
        payload: Vec<u8>,
    ) -> Result<String>;

    async fn estimate_fees(
        &self,
        destination_chain_id: u16,
        destination_address: &str,
        payload: Vec<u8>,
    ) -> Result<u64>;

    async fn verify_message(
        &self,
        source_chain_id: u16,
        source_address: &str,
        nonce: u64,
        payload: Vec<u8>,
    ) -> Result<bool>;
}

impl LayerZeroClient {
    pub fn new(rpc_url: String, private_key: String, endpoint_address: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let provider = Arc::new(provider);
        let wallet = private_key.parse::<LocalWallet>()?;
        let endpoint_contract = Address::from_str(endpoint_address)?;

        Ok(Self {
            provider,
            wallet,
            endpoint_contract,
        })
    }

    fn get_address(&self) -> Address {
        self.wallet.address()
    }

    pub async fn send_cross_chain_message(
        &self,
        message: CrossChainMessage,
    ) -> Result<TransactionInfo> {
        match (message.source_chain, message.destination_chain) {
            (ChainId::Ethereum, ChainId::Solana) => {
                self.send_eth_to_solana(message).await
            }
            (ChainId::Solana, ChainId::Ethereum) => {
                self.send_solana_to_eth(message).await
            }
            _ => Err(anyhow::anyhow!("Unsupported chain combination")),
        }
    }

    async fn send_eth_to_solana(&self, message: CrossChainMessage) -> Result<TransactionInfo> {
        // TODO: Implementar envío de mensaje de Ethereum a Solana
        Ok(TransactionInfo {
            hash: "pending".to_string(),
            from: self.get_address().to_string(),
            to: self.endpoint_contract.to_string(),
            value: "0".to_string(),
            status: TransactionStatus::Pending,
        })
    }

    async fn send_solana_to_eth(&self, message: CrossChainMessage) -> Result<TransactionInfo> {
        // TODO: Implementar envío de mensaje de Solana a Ethereum
        Ok(TransactionInfo {
            hash: "pending".to_string(),
            from: self.get_address().to_string(),
            to: self.endpoint_contract.to_string(),
            value: "0".to_string(),
            status: TransactionStatus::Pending,
        })
    }

    pub async fn verify_message_delivery(&self, tx_hash: &str, chain_id: ChainId) -> Result<bool> {
        // TODO: Implementar verificación de entrega de mensaje
        Ok(false)
    }
}

#[async_trait]
impl CrossChainMessenger for LayerZeroClient {
    async fn send_cross_chain_message(
        &self,
        destination_chain_id: u16,
        destination_address: &str,
        payload: Vec<u8>,
    ) -> Result<String> {
        let destination = Address::from_str(destination_address)?;
        
        // Crear una instancia del contrato de LayerZero Endpoint
        let contract = ethers::contract::Contract::new(
            self.endpoint_contract,
            include_bytes!("../../../contracts/layerzero_endpoint_abi.json"),
            self.provider.clone(),
        );

        // Preparar los parámetros para send()
        let adapter_params = ethers::core::types::Bytes::from(vec![1, 0, 0, 0]); // Version 1
        
        // Enviar el mensaje cross-chain
        let tx = contract
            .method(
                "send",
                (
                    destination_chain_id,
                    ethers::core::types::Bytes::from(destination.as_bytes().to_vec()),
                    ethers::core::types::Bytes::from(payload),
                    self.get_address(),
                    Address::zero(),
                    adapter_params,
                ),
            )?
            .send()
            .await?;

        Ok(format!("0x{:x}", tx.tx_hash()))
    }

    async fn estimate_fees(
        &self,
        destination_chain_id: u16,
        destination_address: &str,
        payload: Vec<u8>,
    ) -> Result<u64> {
        let destination = Address::from_str(destination_address)?;
        
        // Crear una instancia del contrato de LayerZero Endpoint
        let contract = ethers::contract::Contract::new(
            self.endpoint_contract,
            include_bytes!("../../../contracts/layerzero_endpoint_abi.json"),
            self.provider.clone(),
        );

        // Preparar los parámetros para estimateNativeFees()
        let adapter_params = ethers::core::types::Bytes::from(vec![1, 0, 0, 0]); // Version 1
        
        // Estimar las tarifas
        let (native_fee, _): (U256, U256) = contract
            .method(
                "estimateNativeFees",
                (
                    destination_chain_id,
                    ethers::core::types::Bytes::from(destination.as_bytes().to_vec()),
                    ethers::core::types::Bytes::from(payload),
                    false,
                    adapter_params,
                ),
            )?
            .call()
            .await?;

        Ok(native_fee.as_u64())
    }

    async fn verify_message(
        &self,
        source_chain_id: u16,
        source_address: &str,
        nonce: u64,
        payload: Vec<u8>,
    ) -> Result<bool> {
        let source = Address::from_str(source_address)?;
        
        // Crear una instancia del contrato de LayerZero Endpoint
        let contract = ethers::contract::Contract::new(
            self.endpoint_contract,
            include_bytes!("../../../contracts/layerzero_endpoint_abi.json"),
            self.provider.clone(),
        );

        // Verificar el mensaje
        let stored_payload: ethers::core::types::Bytes = contract
            .method(
                "storedPayload",
                (
                    source_chain_id,
                    ethers::core::types::Bytes::from(source.as_bytes().to_vec()),
                    nonce,
                ),
            )?
            .call()
            .await?;

        // Comparar el payload almacenado con el proporcionado
        Ok(stored_payload.to_vec() == payload)
    }
}

// Módulo para contratos específicos de LayerZero
pub mod contracts {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EndpointConfig {
        pub chain_id: ChainId,
        pub contract_address: String,
        pub gas_limit: u64,
    }

    impl EndpointConfig {
        pub fn new(chain_id: ChainId, contract_address: String, gas_limit: u64) -> Self {
            Self {
                chain_id,
                contract_address,
                gas_limit,
            }
        }
    }
}

#[cfg(test)]
mod tests; 