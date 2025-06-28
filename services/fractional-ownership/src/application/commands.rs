// CQRS Commands para fractional ownership
use uuid::Uuid;
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use vibestream_types::Command;

/// Command para comprar acciones de una canción fraccionada
#[derive(Debug, Clone)]
pub struct PurchaseSharesCommand {
    pub fractional_song_id: Uuid,
    pub buyer_id: Uuid,
    pub shares_quantity: u32,
    pub auto_confirm: bool, // Si true, confirma inmediatamente la compra
}

impl Command for PurchaseSharesCommand {}

#[derive(Debug, Clone)]
pub struct TransferSharesCommand {
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub song_id: Uuid,
    pub percentage: OwnershipPercentage,
    pub transfer_price: SharePrice,
}

impl Command for TransferSharesCommand {}

#[derive(Debug, Clone)]
pub struct DistributeRevenueCommand {
    pub song_id: Uuid,
    pub total_revenue: RevenueAmount,
    pub revenue_period: String,
}

impl Command for DistributeRevenueCommand {}

#[derive(Debug, Clone)]
pub struct CreateFractionalSongCommand {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub song_title: String,
    pub total_shares: u32,
    pub initial_price_per_share: SharePrice,
    pub artist_reserved_percentage: OwnershipPercentage,
}

impl Command for CreateFractionalSongCommand {} 