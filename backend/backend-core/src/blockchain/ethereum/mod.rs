use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    core::types::{Address, TransactionReceipt, U256, H256},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Transaction,
    contract::Contract,
};
use std::str::FromStr;
use std::sync::Arc;

use crate::blockchain::common::{
    BlockchainClient, TokenClient, TokenInfo, TransactionInfo, TransactionStatus,
    BlockchainResult, TransactionHash,
};

pub struct EthereumClient {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
}

impl EthereumClient {
    pub fn new(rpc_url: String, private_key: String) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let provider = Arc::new(provider);
        let wallet = private_key.parse::<LocalWallet>()?;

        Ok(Self {
            provider,
            wallet,
        })
    }

    pub fn get_address(&self) -> Address {
        self.wallet.address()
    }

    pub async fn send_raw_transaction(&self, tx: Transaction) -> BlockchainResult<TransactionHash> {
        let signature = self.wallet.sign_transaction(&tx).await?;
        let tx_hash = self.provider.send_raw_transaction(signature).await?;
        Ok(tx_hash)
    }

    pub async fn estimate_gas(&self, tx: &Transaction) -> BlockchainResult<U256> {
        Ok(self.provider.estimate_gas(tx, None).await?)
    }

    pub async fn get_nonce(&self, address: Address) -> BlockchainResult<U256> {
        Ok(self.provider.get_transaction_count(address, None).await?)
    }
}

#[async_trait]
impl BlockchainClient for EthereumClient {
    async fn get_balance(&self, address: &str) -> Result<u64> {
        let address = Address::from_str(address)?;
        let balance = self.provider.get_balance(address, None).await?;
        Ok(balance.as_u64())
    }

    async fn transfer(&self, to: &str, amount: u64) -> Result<TransactionInfo> {
        let to_address = Address::from_str(to)?;
        let tx = Transaction {
            from: Some(self.get_address()),
            to: Some(to_address),
            value: U256::from(amount),
            ..Default::default()
        };

        let tx_hash = self.send_raw_transaction(tx).await?;
        
        Ok(TransactionInfo {
            hash: format!("0x{:x}", tx_hash),
            from: format!("0x{:x}", self.get_address()),
            to: to.to_string(),
            value: amount.to_string(),
            status: TransactionStatus::Pending,
        })
    }

    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus> {
        let tx_hash = H256::from_str(tx_hash)?;
        let receipt = self.provider.get_transaction_receipt(tx_hash).await?;

        match receipt {
            Some(TransactionReceipt { status: Some(status), .. }) => {
                if status.as_u64() == 1 {
                    Ok(TransactionStatus::Confirmed)
                } else {
                    Ok(TransactionStatus::Failed)
                }
            }
            _ => Ok(TransactionStatus::Pending),
        }
    }
}

#[async_trait]
impl TokenClient for EthereumClient {
    async fn get_token_info(&self, token_address: &str) -> Result<TokenInfo> {
        let contract_address = Address::from_str(token_address)?;
        
        let contract = Contract::new(
            contract_address,
            include_bytes!("../../../contracts/erc20_abi.json"),
            self.provider.clone(),
        );

        let symbol: String = contract.method("symbol", ())?.call().await?;
        let decimals: u8 = contract.method("decimals", ())?.call().await?;
        let total_supply: U256 = contract.method("totalSupply", ())?.call().await?;

        Ok(TokenInfo {
            address: token_address.to_string(),
            symbol,
            decimals,
            total_supply: total_supply.as_u64(),
        })
    }

    async fn transfer_token(&self, token_address: &str, to: &str, amount: u64) -> Result<TransactionInfo> {
        let contract_address = Address::from_str(token_address)?;
        let to_address = Address::from_str(to)?;

        let contract = Contract::new(
            contract_address,
            include_bytes!("../../../contracts/erc20_abi.json"),
            self.provider.clone(),
        );

        let tx = contract
            .method("transfer", (to_address, U256::from(amount)))?
            .send()
            .await?;

        Ok(TransactionInfo {
            hash: format!("0x{:x}", tx.tx_hash()),
            from: format!("0x{:x}", self.get_address()),
            to: to.to_string(),
            value: amount.to_string(),
            status: TransactionStatus::Pending,
        })
    }

    async fn get_token_balance(&self, token_address: &str, address: &str) -> Result<u64> {
        let contract_address = Address::from_str(token_address)?;
        let owner_address = Address::from_str(address)?;

        let contract = Contract::new(
            contract_address,
            include_bytes!("../../../contracts/erc20_abi.json"),
            self.provider.clone(),
        );

        let balance: U256 = contract
            .method("balanceOf", owner_address)?
            .call()
            .await?;

        Ok(balance.as_u64())
    }
}

#[cfg(test)]
mod tests; 