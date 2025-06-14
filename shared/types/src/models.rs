use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::IpAddr;

// User models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub wallet_address: Option<String>,
    pub role: UserRole,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "artist")]
    Artist,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

// Artist models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stage_name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArtist {
    pub stage_name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
}

// Song models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: Option<i32>,
    pub genre: Option<String>,
    pub ipfs_hash: Option<String>,
    pub metadata_url: Option<String>,
    pub nft_contract_address: Option<String>,
    pub nft_token_id: Option<String>,
    pub royalty_percentage: Option<rust_decimal::Decimal>,
    pub is_minted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSong {
    pub title: String,
    pub duration_seconds: Option<i32>,
    pub genre: Option<String>,
    pub ipfs_hash: Option<String>,
    pub metadata_url: Option<String>,
    pub royalty_percentage: Option<rust_decimal::Decimal>,
}

// Playlist models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub cover_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylist {
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub cover_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistSong {
    pub id: Uuid,
    pub playlist_id: Uuid,
    pub song_id: Uuid,
    pub position: i32,
    pub added_at: DateTime<Utc>,
}

// Transaction models (extending existing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: Uuid,
    pub request_id: String,
    pub user_id: Option<Uuid>,
    pub blockchain: crate::blockchain::Blockchain,
    pub transaction_type: TransactionType,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64, // Amount in wei (ETH) or lamports (SOL)
    pub tx_hash: Option<String>,
    pub status: TransactionStatus,
    pub error_message: Option<String>,
    pub gas_used: Option<i64>,
    pub gas_price: Option<i64>,
    pub block_number: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "mint_nft")]
    MintNft,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "royalty_payment")]
    RoyaltyPayment,
    #[serde(rename = "purchase")]
    Purchase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for TransactionStatus {
    fn default() -> Self {
        TransactionStatus::Pending
    }
}

// Royalty models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyPayment {
    pub id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub amount: i64, // Amount in wei or lamports
    pub blockchain: crate::blockchain::Blockchain,
    pub payment_date: DateTime<Utc>,
    pub status: TransactionStatus,
}

// Listen event models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub song_id: Uuid,
    pub listen_duration_seconds: i32,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub zk_proof_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateListenEvent {
    pub song_id: Uuid,
    pub listen_duration_seconds: i32,
    pub user_agent: Option<String>,
    pub zk_proof_hash: Option<String>,
} 