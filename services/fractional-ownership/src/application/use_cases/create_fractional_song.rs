use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::FractionalSong;
use crate::domain::value_objects::{OwnershipPercentage, SharePrice};
use crate::domain::repositories::FractionalOwnershipRepository;
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateFractionalSongCommand {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub song_title: String,
    pub total_shares: u32,
    pub initial_price_per_share: SharePrice,
    pub artist_reserved_percentage: OwnershipPercentage,
}

pub struct CreateFractionalSongUseCase {
    repository: Arc<dyn FractionalOwnershipRepository>,
}

impl CreateFractionalSongUseCase {
    pub fn new(repository: Arc<dyn FractionalOwnershipRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateFractionalSongCommand) -> Result<FractionalSong, FractionalOwnershipError> {
        // Verificar que la canción no existe ya
        if let Some(_) = self.repository.load_aggregate(&command.song_id).await? {
            return Err(FractionalOwnershipError::SongAlreadyExists);
        }

        // Crear nuevo agregado
        let mut aggregate = FractionalOwnershipAggregate::create_new(
            command.song_id,
            command.artist_id,
            command.song_title,
            command.total_shares,
            command.initial_price_per_share,
            command.artist_reserved_percentage,
        )?;

        // Guardar el agregado
        self.repository.save_aggregate(&aggregate).await?;

        Ok(aggregate.fractional_song().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_fractional_song_success() {
        // TODO: Implementar test cuando tengamos el repository mock
    }
} 