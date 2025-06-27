// Modelos de base de datos para fractional ownership
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractionalSongModel {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub total_shares: i32,
    pub available_shares: i32,
    pub current_price_per_share: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareOwnershipModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub shares_owned: i32,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareTransactionModel {
    pub id: Uuid,
    pub buyer_id: Option<Uuid>,
    pub seller_id: Option<Uuid>,
    pub song_id: Uuid,
    pub shares_quantity: i32,
    pub price_per_share: f64,
    pub transaction_type: String, // "purchase", "transfer", "sale"
    pub status: String, // "pending", "completed", "cancelled"
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
} 