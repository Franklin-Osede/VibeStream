use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::{FractionalSong, ShareOwnership, ShareTransaction};
use crate::domain::errors::FractionalOwnershipError;
use crate::domain::value_objects::RevenueAmount;
use async_trait::async_trait;
use uuid::Uuid;

/// Repository trait para agregados de fractional ownership
/// Esta es una interface/contrato que define qué operaciones necesita el dominio
/// Las implementaciones concretas estarán en la capa de infrastructure
#[async_trait]
pub trait FractionalOwnershipRepository: Send + Sync {
    /// Obtener aggregate completo por ID de canción fraccionada
    async fn get_by_id(&self, song_id: Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError>;
    
    /// Métodos adicionales necesarios para los use cases
    async fn load_aggregate(&self, song_id: &Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError>;
    async fn save_aggregate(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError>;
    async fn get_user_ownerships(&self, user_id: &Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError>;
    async fn get_user_revenue_for_song(&self, user_id: &Uuid, song_id: &Uuid) -> Result<Option<RevenueAmount>, FractionalOwnershipError>;
    
    /// Guardar cambios en el aggregate
    async fn save(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError>;
    
    /// Eliminar aggregate
    async fn delete(&self, song_id: Uuid) -> Result<(), FractionalOwnershipError>;
    
    /// Buscar aggregates por criterios específicos
    async fn find_by_artist_id(&self, artist_id: Uuid) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError>;
    
    /// Obtener todas las canciones fraccionadas con paginación
    async fn get_all_paginated(&self, page: u32, size: u32) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError>;
}

/// Repository trait específico para entidades FractionalSong
/// Para casos donde no necesitamos el aggregate completo
#[async_trait]
pub trait FractionalSongRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<FractionalSong>, FractionalOwnershipError>;
    async fn save(&self, song: &FractionalSong) -> Result<(), FractionalOwnershipError>;
    async fn get_by_artist(&self, artist_id: Uuid) -> Result<Vec<FractionalSong>, FractionalOwnershipError>;
    async fn search_by_title(&self, title_pattern: &str) -> Result<Vec<FractionalSong>, FractionalOwnershipError>;
}

/// Repository trait para ShareOwnership
#[async_trait]
pub trait ShareOwnershipRepository: Send + Sync {
    async fn get_by_user_and_song(&self, user_id: Uuid, song_id: Uuid) -> Result<Option<ShareOwnership>, FractionalOwnershipError>;
    async fn get_by_user(&self, user_id: Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError>;
    async fn get_by_song(&self, song_id: Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError>;
    async fn save(&self, ownership: &ShareOwnership) -> Result<(), FractionalOwnershipError>;
    async fn delete(&self, id: Uuid) -> Result<(), FractionalOwnershipError>;
}

/// Repository trait para ShareTransaction
#[async_trait]
pub trait ShareTransactionRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ShareTransaction>, FractionalOwnershipError>;
    async fn save(&self, transaction: &ShareTransaction) -> Result<(), FractionalOwnershipError>;
    async fn get_pending_by_user(&self, user_id: Uuid) -> Result<Vec<ShareTransaction>, FractionalOwnershipError>;
    async fn get_by_song(&self, song_id: Uuid) -> Result<Vec<ShareTransaction>, FractionalOwnershipError>;
    async fn get_transactions_history(&self, user_id: Uuid, page: u32, size: u32) -> Result<Vec<ShareTransaction>, FractionalOwnershipError>;
}

/// Repository trait para consultas de análisis y reporting
#[async_trait]
pub trait FractionalOwnershipAnalyticsRepository: Send + Sync {
    /// Obtener estadísticas de ownership por usuario
    async fn get_user_portfolio_summary(&self, user_id: Uuid) -> Result<UserPortfolioSummary, FractionalOwnershipError>;
    
    /// Obtener trending songs (más transacciones recientes)
    async fn get_trending_songs(&self, limit: u32) -> Result<Vec<TrendingSong>, FractionalOwnershipError>;
    
    /// Obtener estadísticas de mercado por género
    async fn get_genre_market_stats(&self, genre: &str) -> Result<GenreMarketStats, FractionalOwnershipError>;
    
    /// Obtener top shareholders de una canción
    async fn get_top_shareholders(&self, song_id: Uuid, limit: u32) -> Result<Vec<TopShareholder>, FractionalOwnershipError>;
}

// DTOs para queries de analytics (estas van en el domain porque definen contratos)

#[derive(Debug, Clone)]
pub struct UserPortfolioSummary {
    pub user_id: Uuid,
    pub total_investment_value: f64,
    pub total_earnings: f64,
    pub number_of_songs: u32,
    pub average_roi: f64,
    pub songs_breakdown: Vec<SongInvestmentSummary>,
}

#[derive(Debug, Clone)]
pub struct SongInvestmentSummary {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub shares_owned: u32,
    pub investment_value: f64,
    pub earnings_to_date: f64,
    pub current_roi: f64,
}

#[derive(Debug, Clone)]
pub struct TrendingSong {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub recent_transactions_count: u32,
    pub recent_volume: f64,
    pub price_change_percentage: f64,
}

#[derive(Debug, Clone)]
pub struct GenreMarketStats {
    pub genre: String,
    pub total_songs: u32,
    pub total_market_value: f64,
    pub average_price_per_share: f64,
    pub most_active_songs: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct TopShareholder {
    pub user_id: Uuid,
    pub shares_owned: u32,
    pub ownership_percentage: f64,
    pub investment_value: f64,
}

/// Unit of Work pattern para operaciones transaccionales
/// Permite agrupar múltiples operaciones en una sola transacción
#[async_trait]
pub trait FractionalOwnershipUnitOfWork: Send + Sync {
    type FractionalOwnershipRepo: FractionalOwnershipRepository;
    type ShareOwnershipRepo: ShareOwnershipRepository;
    type ShareTransactionRepo: ShareTransactionRepository;
    
    fn fractional_ownership_repository(&self) -> &Self::FractionalOwnershipRepo;
    fn share_ownership_repository(&self) -> &Self::ShareOwnershipRepo;
    fn share_transaction_repository(&self) -> &Self::ShareTransactionRepo;
    
    /// Comenzar transacción
    async fn begin_transaction(&mut self) -> Result<(), FractionalOwnershipError>;
    
    /// Confirmar todos los cambios
    async fn commit(&mut self) -> Result<(), FractionalOwnershipError>;
    
    /// Revertir todos los cambios
    async fn rollback(&mut self) -> Result<(), FractionalOwnershipError>;
}

/// Specification pattern para consultas complejas
/// Permite encapsular lógica de filtrado/búsqueda de manera reutilizable
pub trait FractionalOwnershipSpecification {
    fn is_satisfied_by(&self, aggregate: &FractionalOwnershipAggregate) -> bool;
    fn to_sql_where_clause(&self) -> String;
}

/// Especificaciones concretas de ejemplo
pub struct HighPerformingSongsSpec {
    pub min_roi_percentage: f64,
}

impl FractionalOwnershipSpecification for HighPerformingSongsSpec {
    fn is_satisfied_by(&self, aggregate: &FractionalOwnershipAggregate) -> bool {
        // Calcular ROI promedio de todos los shareholders
        let total_investment: f64 = aggregate.ownerships().values()
            .map(|o| o.purchase_price().amount() * o.shares_owned() as f64)
            .sum();
        
        let total_earnings: f64 = aggregate.ownerships().values()
            .map(|o| o.total_earnings().amount())
            .sum();
        
        if total_investment > 0.0 {
            let roi = (total_earnings / total_investment) * 100.0;
            roi >= self.min_roi_percentage
        } else {
            false
        }
    }
    
    fn to_sql_where_clause(&self) -> String {
        format!("(total_earnings / total_investment) * 100 >= {}", self.min_roi_percentage)
    }
}

pub struct AvailableSharesSpec {
    pub min_available_percentage: f64,
}

impl FractionalOwnershipSpecification for AvailableSharesSpec {
    fn is_satisfied_by(&self, aggregate: &FractionalOwnershipAggregate) -> bool {
        let available_percentage = (aggregate.fractional_song().available_shares() as f64 / 
                                   aggregate.fractional_song().total_shares() as f64) * 100.0;
        available_percentage >= self.min_available_percentage
    }
    
    fn to_sql_where_clause(&self) -> String {
        format!("(available_shares::float / total_shares::float) * 100 >= {}", self.min_available_percentage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_performing_songs_spec_should_work() {
        // Crear un aggregate de prueba
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        
        // Para pruebas, necesitaríamos crear un aggregate mock
        // Esto se testearía mejor con agregates reales
        let spec = HighPerformingSongsSpec {
            min_roi_percentage: 20.0,
        };
        
        // Test del SQL generation
        let sql = spec.to_sql_where_clause();
        assert!(sql.contains("20"));
        assert!(sql.contains("total_earnings"));
        assert!(sql.contains("total_investment"));
    }

    #[test]
    fn available_shares_spec_should_generate_correct_sql() {
        let spec = AvailableSharesSpec {
            min_available_percentage: 50.0,
        };
        
        let sql = spec.to_sql_where_clause();
        assert!(sql.contains("50"));
        assert!(sql.contains("available_shares"));
        assert!(sql.contains("total_shares"));
    }
} 