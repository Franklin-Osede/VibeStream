use crate::domain::repositories::FractionalOwnershipRepository;
use crate::domain::errors::FractionalOwnershipError;
use crate::application::dtos::{GetUserPortfolioQuery, UserPortfolioResponse, PortfolioSongInfo, PerformanceMetrics};

pub struct GetUserPortfolioUseCase<R: FractionalOwnershipRepository> {
    repository: R,
}

impl<R: FractionalOwnershipRepository> GetUserPortfolioUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, query: GetUserPortfolioQuery) -> Result<UserPortfolioResponse, FractionalOwnershipError> {
        // Obtener todas las ownerships del usuario
        let user_ownerships = self.repository
            .get_user_ownerships(&query.user_id)
            .await?;

        if user_ownerships.is_empty() {
            return Ok(UserPortfolioResponse {
                user_id: query.user_id,
                total_investment: 0.0,
                total_earnings: 0.0,
                total_portfolio_value: 0.0,
                songs: vec![],
                performance_metrics: PerformanceMetrics::default(),
            });
        }

        let mut songs = Vec::new();
        let mut total_investment = 0.0;
        let mut total_earnings = 0.0;
        let mut total_portfolio_value = 0.0;

        for ownership in user_ownerships {
            // Obtener datos de la canción desde el aggregate
            if let Some(aggregate) = self.repository.load_aggregate(&ownership.song_id()).await? {
                let song = aggregate.fractional_song();
                
                // Calcular métricas para esta canción
                let investment_in_song = ownership.percentage().as_f64() * song.current_price_per_share().as_f64();
                let current_value = ownership.calculate_current_value(song.current_price_per_share());
                
                // Obtener earnings del usuario para esta canción específica
                let user_earnings_amount = self.repository
                    .get_user_revenue_for_song(&query.user_id, &ownership.song_id())
                    .await?;
                
                let user_earnings = match user_earnings_amount {
                    Some(revenue) => revenue.as_f64(),
                    None => 0.0,
                };

                songs.push(PortfolioSongInfo {
                    song_id: ownership.song_id(),
                    song_title: song.title().to_string(),
                    ownership_percentage: ownership.percentage().clone(),
                    current_value: current_value.as_f64(),
                    total_earnings: user_earnings,
                    shares_owned: ownership.shares_owned(),
                });

                total_investment += ownership.purchase_price().as_f64();
                total_earnings += user_earnings;
                total_portfolio_value += current_value.as_f64();
            }
        }

        Ok(UserPortfolioResponse {
            user_id: query.user_id,
            total_investment,
            total_earnings,
            total_portfolio_value,
            songs,
            performance_metrics: PerformanceMetrics::default(),
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