use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::ShareOwnership;
use crate::domain::value_objects::RevenueAmount;
use crate::domain::events::RevenueDistributed;
use crate::domain::repositories::FractionalOwnershipRepository;
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DistributeRevenueCommand {
    pub song_id: Uuid,
    pub total_revenue: RevenueAmount,
    pub revenue_period: String,
}

pub struct DistributeRevenueUseCase {
    repository: Arc<dyn FractionalOwnershipRepository>,
}

impl DistributeRevenueUseCase {
    pub fn new(repository: Arc<dyn FractionalOwnershipRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: DistributeRevenueCommand) -> Result<HashMap<Uuid, RevenueAmount>, FractionalOwnershipError> {
        // Cargar el agregado
        let mut aggregate = self.repository
            .load_aggregate(&command.song_id)
            .await?
            .ok_or(FractionalOwnershipError::SongNotFound)?;

        // Ejecutar la distribución de ingresos
        let distribution = aggregate.distribute_revenue(
            command.total_revenue,
            command.revenue_period,
        )?;

        // Guardar cambios
        self.repository.save_aggregate(&aggregate).await?;

        Ok(distribution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::FractionalSong;

    #[tokio::test]
    async fn test_distribute_revenue_success() {
        // TODO: Implementar test cuando tengamos el repository mock
    }
} 