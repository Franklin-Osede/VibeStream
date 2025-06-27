// DTOs - Data Transfer Objects para la application layer
// Estos objetos transportan datos entre capas sin lógica de negocio

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::value_objects::RevenueAmount;

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

/// Command para crear una canción fraccionada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFractionalSongCommand {
    pub song_id: Uuid, // Referencia al Song Context
    pub artist_id: Uuid,
    pub title: String,
    pub total_shares: u32,
    pub initial_share_price: f64,
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

/// Resultado del portfolio de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPortfolioResult {
    pub user_id: Uuid,
    pub total_investment_value: RevenueAmount,
    pub total_current_value: RevenueAmount,
    pub total_earnings: RevenueAmount,
    pub overall_roi_percentage: f64,
    pub songs_count: u32,
    pub songs_breakdown: Vec<SongPortfolioItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPortfolioItem {
    pub fractional_song_id: Uuid,
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub shares_owned: u32,
    pub ownership_percentage: f64,
    pub original_investment: RevenueAmount,
    pub current_value: RevenueAmount,
    pub earnings_to_date: RevenueAmount,
    pub roi_percentage: f64,
    pub last_revenue_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// Query para obtener información de una canción fraccionada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFractionalSongQuery {
    pub fractional_song_id: Uuid,
    pub include_ownership_breakdown: bool,
    pub include_transaction_history: bool,
}

/// Resultado de información de canción fraccionada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractionalSongResult {
    pub fractional_song_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub total_shares: u32,
    pub available_shares: u32,
    pub sold_percentage: f64,
    pub current_share_price: f64,
    pub total_revenue: RevenueAmount,
    pub market_value: RevenueAmount,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub ownership_breakdown: Option<Vec<OwnershipItem>>,
    pub recent_transactions: Option<Vec<TransactionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipItem {
    pub user_id: Uuid,
    pub shares_owned: u32,
    pub ownership_percentage: f64,
    pub investment_value: RevenueAmount,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionItem {
    pub transaction_id: Uuid,
    pub transaction_type: String, // "Purchase" | "Transfer"
    pub buyer_id: Option<Uuid>,
    pub seller_id: Option<Uuid>,
    pub shares_quantity: u32,
    pub price_per_share: f64,
    pub total_amount: RevenueAmount,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
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