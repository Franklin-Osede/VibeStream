// DTOs - Data Transfer Objects para la application layer
// Estos objetos transportan datos entre capas sin lógica de negocio

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{RevenueAmount, OwnershipPercentage};

/// Command para comprar acciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSharesCommand {
    pub fractional_song_id: Uuid,
    pub buyer_id: Uuid,
    pub shares_quantity: u32,
    pub auto_confirm: bool, // Si true, confirma inmediatamente la compra
}

/// Resultado de la compra de acciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSharesResult {
    pub transaction_id: Uuid,
    pub fractional_song_id: Uuid,
    pub buyer_id: Uuid,
    pub shares_purchased: u32,
    pub total_cost: RevenueAmount,
    pub new_ownership_percentage: f64,
    pub transaction_status: String, // "Pending" | "Completed"
    pub remaining_available_shares: u32,
}

/// Command para transferir acciones entre usuarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSharesCommand {
    pub fractional_song_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub shares_quantity: u32,
    pub price_per_share: f64,
}

/// Resultado de transferencia de acciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSharesResult {
    pub transaction_id: Uuid,
    pub fractional_song_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub shares_transferred: u32,
    pub total_amount: RevenueAmount,
    pub transaction_status: String,
}

/// Request para crear una nueva canción fraccionada
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFractionalSongRequest {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub total_shares: u32,
    
    // 🆕 ARTIST CONTROL FIELDS
    pub artist_reserved_shares: u32,        // Cuántas shares se queda el artista
    pub artist_revenue_percentage: f64,     // % adicional de ingresos para el artista (0.0-1.0)
    
    pub initial_price_per_share: f64,
    
    // 🆕 OPTIONAL CAMPAIGN SETTINGS
    pub max_shares_per_user: Option<u32>,   // Límite por usuario
    pub campaign_duration_days: Option<u32>, // Duración de la campaña
    pub funding_goal: Option<f64>,          // Meta de financiamiento
}

impl CreateFractionalSongRequest {
    /// Validar request del artista
    pub fn validate(&self) -> Result<(), String> {
        if self.total_shares == 0 {
            return Err("Total shares must be greater than 0".to_string());
        }

        if self.artist_reserved_shares > self.total_shares {
            return Err("Artist reserved shares cannot exceed total shares".to_string());
        }

        if self.artist_revenue_percentage < 0.0 || self.artist_revenue_percentage > 1.0 {
            return Err("Artist revenue percentage must be between 0% and 100%".to_string());
        }

        if self.initial_price_per_share <= 0.0 {
            return Err("Initial price per share must be greater than 0".to_string());
        }

        // Validar que al menos 10% esté disponible para fans
        let fan_percentage = (self.total_shares - self.artist_reserved_shares) as f64 / self.total_shares as f64;
        if fan_percentage < 0.1 {
            return Err("At least 10% of shares must be available for fans".to_string());
        }

        Ok(())
    }

    /// Calcular shares disponibles para fans
    pub fn fan_available_shares(&self) -> u32 {
        self.total_shares - self.artist_reserved_shares
    }

    /// Calcular potencial funding total
    pub fn potential_funding(&self) -> f64 {
        self.fan_available_shares() as f64 * self.initial_price_per_share
    }
}

/// Resultado de crear canción fraccionada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFractionalSongResult {
    pub fractional_song_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub total_shares: u32,
    pub initial_share_price: f64,
    pub available_shares: u32,
    pub market_value: RevenueAmount,
}

/// Command para distribuir ingresos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenueCommand {
    pub fractional_song_id: Uuid,
    pub total_revenue: f64,
    pub revenue_source: String, // "streaming", "sales", "licensing", etc.
}

/// Resultado de distribución de ingresos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenueResult {
    pub fractional_song_id: Uuid,
    pub total_revenue_distributed: RevenueAmount,
    pub shareholders_count: u32,
    pub individual_distributions: Vec<IndividualDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualDistribution {
    pub user_id: Uuid,
    pub shares_owned: u32,
    pub ownership_percentage: f64,
    pub revenue_share: RevenueAmount,
}

/// Query para obtener portfolio de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPortfolioQuery {
    pub user_id: Uuid,
    pub include_detailed_breakdown: bool,
}

/// Response del portfolio de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPortfolioResponse {
    pub user_id: Uuid,
    pub total_investment: f64,
    pub total_earnings: f64,
    pub total_portfolio_value: f64,
    pub songs: Vec<PortfolioSongInfo>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSongInfo {
    pub song_id: Uuid,
    pub song_title: String,
    pub ownership_percentage: OwnershipPercentage,
    pub current_value: f64,
    pub total_earnings: f64,
    pub shares_owned: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_roi_percentage: f64,
    pub best_performing_song: Option<Uuid>,
    pub worst_performing_song: Option<Uuid>,
    pub average_monthly_earnings: f64,
}

/// Request para comprar shares
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSharesRequest {
    pub fractional_song_id: Uuid,
    pub buyer_id: Uuid,
    pub shares_quantity: u32,
    pub auto_confirm: bool,
}

/// Request para transferir shares
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSharesRequest {
    pub fractional_song_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub shares_quantity: u32,
    pub price_per_share: f64,
}

/// Request para distribuir revenue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenueRequest {
    pub fractional_song_id: Uuid,
    pub total_revenue: f64,
    pub revenue_source: String,
}

/// Query para obtener información de una canción fraccionada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFractionalSongQuery {
    pub fractional_song_id: Uuid,
    pub include_ownership_breakdown: bool,
    pub include_transaction_history: bool,
}

/// Response con información completa de una canción fraccionada
#[derive(Debug, Serialize, Deserialize)]
pub struct FractionalSongResponse {
    pub id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    
    // Shares information
    pub total_shares: u32,
    pub available_shares: u32,
    pub current_price_per_share: f64,
    
    // 🆕 ARTIST CONTROL INFO
    pub artist_reserved_shares: u32,
    pub fan_available_shares: u32,
    pub artist_revenue_percentage: f64,
    pub artist_ownership_percentage: f64,   // Calculated field
    pub max_fan_ownership_percentage: f64,  // Calculated field
    
    // 🆕 CAMPAIGN INFO
    pub potential_fan_funding: f64,         // fan_available_shares * price
    pub current_funding_raised: f64,        // sold_shares * price
    pub funding_completion_percentage: f64, // % of fan shares sold
    
    // 🆕 MARKET STATS
    pub total_shareholders: u32,
    pub avg_shares_per_holder: f64,
    pub price_change_24h: Option<f64>,
    
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl FractionalSongResponse {
    /// Constructor desde FractionalSong entity
    pub fn from_entity(song: &crate::domain::entities::FractionalSong, total_shareholders: u32) -> Self {
        let sold_shares = song.fan_available_shares() - song.available_shares();
        let current_funding_raised = sold_shares as f64 * song.current_price_per_share().amount();
        let potential_fan_funding = song.fan_available_shares() as f64 * song.current_price_per_share().amount();
        
        Self {
            id: song.id(),
            song_id: song.song_id(),
            artist_id: song.artist_id(),
            title: song.title().to_string(),
            
            total_shares: song.total_shares(),
            available_shares: song.available_shares(),
            current_price_per_share: song.current_price_per_share().amount(),
            
            artist_reserved_shares: song.artist_reserved_shares(),
            fan_available_shares: song.fan_available_shares(),
            artist_revenue_percentage: song.artist_revenue_percentage(),
            artist_ownership_percentage: song.artist_ownership_percentage(),
            max_fan_ownership_percentage: song.max_fan_ownership_percentage(),
            
            potential_fan_funding,
            current_funding_raised,
            funding_completion_percentage: if song.fan_available_shares() > 0 {
                (sold_shares as f64 / song.fan_available_shares() as f64) * 100.0
            } else { 0.0 },
            
            total_shareholders,
            avg_shares_per_holder: if total_shareholders > 0 {
                sold_shares as f64 / total_shareholders as f64
            } else { 0.0 },
            price_change_24h: None, // TODO: Implementar con price history
            
            created_at: song.created_at(),
        }
    }
}

/// Query para buscar canciones fraccionadas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFractionalSongsQuery {
    pub title_pattern: Option<String>,
    pub artist_id: Option<Uuid>,
    pub min_available_shares: Option<u32>,
    pub max_share_price: Option<f64>,
    pub min_roi: Option<f64>,
    pub sort_by: Option<String>, // "price", "popularity", "roi", "created_date"
    pub sort_direction: Option<String>, // "asc" | "desc"
    pub page: u32,
    pub page_size: u32,
}

/// Resultado de búsqueda de canciones fraccionadas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFractionalSongsResult {
    pub songs: Vec<FractionalSongSummary>,
    pub total_count: u32,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractionalSongSummary {
    pub fractional_song_id: Uuid,
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub available_shares: u32,
    pub total_shares: u32,
    pub current_share_price: f64,
    pub sold_percentage: f64,
    pub total_revenue: RevenueAmount,
    pub market_value: RevenueAmount,
    pub shareholders_count: u32,
    pub average_roi: f64,
}

/// Query para obtener estadísticas de mercado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketStatsQuery {
    pub period_days: u32, // 7, 30, 90, 365
    pub include_genre_breakdown: bool,
    pub include_trending_songs: bool,
}

/// Resultado de estadísticas de mercado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStatsResult {
    pub total_songs: u32,
    pub total_market_value: RevenueAmount,
    pub total_revenue_distributed: RevenueAmount,
    pub average_share_price: f64,
    pub total_shareholders: u32,
    pub most_active_songs: Vec<FractionalSongSummary>,
    pub genre_breakdown: Option<Vec<GenreStats>>,
    pub trending_songs: Option<Vec<TrendingSongItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreStats {
    pub genre: String,
    pub songs_count: u32,
    pub total_value: RevenueAmount,
    pub average_roi: f64,
    pub growth_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingSongItem {
    pub fractional_song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub current_share_price: f64,
    pub price_change_percentage: f64,
    pub transaction_volume: RevenueAmount,
    pub trending_score: f64,
}

/// Error responses para la API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_type: String, // "ValidationError", "BusinessRuleViolation", "NotFound", etc.
    pub message: String,
    pub details: Option<Vec<String>>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    pub fn validation_error(message: String, details: Option<Vec<String>>) -> Self {
        Self {
            error_type: "ValidationError".to_string(),
            message,
            details,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn business_rule_violation(message: String) -> Self {
        Self {
            error_type: "BusinessRuleViolation".to_string(),
            message,
            details: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn not_found(resource: String) -> Self {
        Self {
            error_type: "NotFound".to_string(),
            message: format!("{} no encontrado", resource),
            details: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn internal_error(message: String) -> Self {
        Self {
            error_type: "InternalError".to_string(),
            message,
            details: None,
            timestamp: chrono::Utc::now(),
        }
    }
} 