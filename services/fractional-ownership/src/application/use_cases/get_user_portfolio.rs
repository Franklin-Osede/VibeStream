use crate::domain::entities::{ShareOwnership, FractionalSong};
use crate::domain::value_objects::{OwnershipPercentage, RevenueAmount, SharePrice};
use crate::domain::repositories::FractionalOwnershipRepository;
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GetUserPortfolioQuery {
    pub user_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct UserPortfolioResponse {
    pub user_id: Uuid,
    pub total_songs: u32,
    pub total_investment: SharePrice,
    pub total_revenue_earned: RevenueAmount,
    pub ownerships: Vec<PortfolioItem>,
}

#[derive(Debug, Clone)]
pub struct PortfolioItem {
    pub song_id: Uuid,
    pub song_title: String,
    pub ownership_percentage: OwnershipPercentage,
    pub current_value: SharePrice,
    pub revenue_earned: RevenueAmount,
}

pub struct GetUserPortfolioUseCase {
    repository: Arc<dyn FractionalOwnershipRepository>,
}

impl GetUserPortfolioUseCase {
    pub fn new(repository: Arc<dyn FractionalOwnershipRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, query: GetUserPortfolioQuery) -> Result<UserPortfolioResponse, FractionalOwnershipError> {
        // Obtener todas las participaciones del usuario
        let user_ownerships = self.repository
            .get_user_ownerships(&query.user_id)
            .await?;

        let mut portfolio_items = Vec::new();
        let mut total_investment = SharePrice::from_amount(0.0)?;
        let mut total_revenue = RevenueAmount::from_amount(0.0)?;

        for ownership in user_ownerships {
            // Cargar información de la canción
            if let Some(aggregate) = self.repository.load_aggregate(&ownership.song_id).await? {
                let song = aggregate.fractional_song();
                
                // Calcular valor actual
                let current_value = SharePrice::from_amount(
                    ownership.percentage.as_f64() * song.current_price_per_share.as_f64()
                )?;

                // Calcular ingresos totales del usuario para esta canción
                let user_revenue = self.repository
                    .get_user_revenue_for_song(&query.user_id, &ownership.song_id)
                    .await?
                    .unwrap_or(RevenueAmount::from_amount(0.0)?);

                portfolio_items.push(PortfolioItem {
                    song_id: ownership.song_id,
                    song_title: song.title.clone(),
                    ownership_percentage: ownership.percentage,
                    current_value,
                    revenue_earned: user_revenue,
                });

                total_investment = SharePrice::from_amount(
                    total_investment.as_f64() + ownership.purchase_price.as_f64()
                )?;
                total_revenue = RevenueAmount::from_amount(
                    total_revenue.as_f64() + user_revenue.as_f64()
                )?;
            }
        }

        Ok(UserPortfolioResponse {
            user_id: query.user_id,
            total_songs: portfolio_items.len() as u32,
            total_investment,
            total_revenue_earned: total_revenue,
            ownerships: portfolio_items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_portfolio_success() {
        // TODO: Implementar test cuando tengamos el repository mock
    }
} 