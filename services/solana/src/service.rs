use crate::client::SolanaClient;
use vibestream_types::*;

pub struct SolanaService {
    client: SolanaClient,
}

impl SolanaService {
    pub fn new(client: SolanaClient) -> Self {
        Self { client }
    }
    
    pub async fn process_message(&self, message: SolanaMessage) -> Result<ServiceResponse> {
        match message {
            SolanaMessage::GetBalance(wallet) => {
                // TODO: Implementar lógica real
                let balance = Balance {
                    wallet,
                    amount: 1000,
                    token_symbol: "SOL".to_string(),
                    last_updated: Timestamp::now(),
                };
                Ok(ServiceResponse::Balance(balance))
            }
            SolanaMessage::SendTransaction { from, to, amount } => {
                // TODO: Implementar lógica real
                let transaction = Transaction {
                    id: RequestId::new(),
                    hash: "mock_hash".to_string(),
                    from,
                    to,
                    amount,
                    blockchain: Blockchain::Solana,
                    timestamp: Timestamp::now(),
                    status: TransactionStatus::Pending,
                };
                Ok(ServiceResponse::Transaction(transaction))
            }
            SolanaMessage::GetTransactionStatus(_hash) => {
                // TODO: Implementar lógica real
                Err(VibeStreamError::Internal { 
                    message: "Not implemented".to_string() 
                })
            }
            SolanaMessage::CreateStream(stream) => {
                // TODO: Implementar lógica real
                Ok(ServiceResponse::Stream(stream))
            }
        }
    }
} 