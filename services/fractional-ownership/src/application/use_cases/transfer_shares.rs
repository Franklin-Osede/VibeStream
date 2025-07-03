use crate::domain::entities::ShareTransaction;
use crate::domain::value_objects::{OwnershipPercentage, SharePrice};
use crate::domain::repositories::FractionalOwnershipRepository;
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransferSharesCommand {
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub song_id: Uuid,
    pub percentage: OwnershipPercentage,
    pub transfer_price: SharePrice,
}

pub struct TransferSharesUseCase {
    repository: Arc<dyn FractionalOwnershipRepository>,
}

impl TransferSharesUseCase {
    pub fn new(repository: Arc<dyn FractionalOwnershipRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: TransferSharesCommand) -> Result<ShareTransaction, FractionalOwnershipError> {
        // Cargar el agregado
        let mut aggregate = self.repository
            .load_aggregate(&command.song_id)
            .await?
            .ok_or(FractionalOwnershipError::SongNotFound)?;

        // Ejecutar la transferencia en el agregado
        let transaction = aggregate.transfer_shares(
            command.from_user_id,
            command.to_user_id,
            command.percentage,
            command.transfer_price,
        )?;

        // Guardar cambios
        self.repository.save_aggregate(&aggregate).await?;

        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::FractionalSong;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_transfer_shares_success() {
        // TODO: Implementar test cuando tengamos el repository mock
    }
} 