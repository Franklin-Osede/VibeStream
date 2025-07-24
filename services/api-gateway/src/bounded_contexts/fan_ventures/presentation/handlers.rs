use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::fractional_ownership::application::FractionalOwnershipApplicationService;
use crate::auth::Claims;

// ====== REQUEST/RESPONSE TYPES ======

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContractRequest {
    pub song_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub minimum_investment: Option<f64>,
    pub vesting_period_months: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContractResponse {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_shares: u32,
    pub shares_available: u32,
    pub price_per_share: f64,
    pub market_cap: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseSharesRequest {
    pub investor_id: Uuid,
    pub shares_to_purchase: u32,
    pub investment_amount: f64,
    pub payment_method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseSharesResponse {
    pub transaction_id: Uuid,
    pub shares_purchased: u32,
    pub total_cost: f64,
    pub ownership_percentage: f64,
    pub estimated_returns: f64,
    pub blockchain_tx_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPortfolioResponse {
    pub user_id: Uuid,
    pub total_invested: f64,
    pub current_value: f64,
    pub total_returns: f64,
    pub roi_percentage: f64,
    pub active_contracts: u32,
    pub positions: Vec<PortfolioPosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioPosition {
    pub contract_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub shares_owned: u32,
    pub ownership_percentage: f64,
    pub initial_investment: f64,
    pub current_value: f64,
    pub revenue_earned: f64,
    pub purchase_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractDetailsResponse {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub total_shares: u32,
    pub shares_sold: u32,
    pub shares_available: u32,
    pub price_per_share: f64,
    pub total_invested: f64,
    pub investor_count: u32,
    pub monthly_revenue: f64,
    pub total_revenue: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub roi_percentage: f64,
    pub monthly_growth: f64,
    pub revenue_per_share: f64,
    pub risk_score: f64,
}

// ====== STATE TYPE ======

pub type AppState = crate::services::AppState;

// ====== HANDLERS ======

/// POST /api/v1/ownership/contracts - Create new ownership contract
pub async fn create_ownership_contract(
    State(_state): State<AppState>,
    claims: Claims,
    Json(request): Json<CreateContractRequest>,
) -> Result<ResponseJson<CreateContractResponse>, StatusCode> {
    // Verify user is artist or admin
    if claims.role != "artist" && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Mock response for now - replace with real application service call
    let contract_id = Uuid::new_v4();
    let shares_available = ((100.0 - request.artist_retained_percentage) / 100.0 * request.total_shares as f64) as u32;
    let market_cap = request.total_shares as f64 * request.price_per_share;

    let response = CreateContractResponse {
        contract_id,
        song_id: request.song_id,
        total_shares: request.total_shares,
        shares_available,
        price_per_share: request.price_per_share,
        market_cap,
        status: "active".to_string(),
        created_at: Utc::now(),
    };

    tracing::info!("✅ Created ownership contract {} for song {}", contract_id, request.song_id);
    Ok(ResponseJson(response))
}

/// POST /api/v1/ownership/contracts/{id}/purchase - Purchase shares
pub async fn purchase_shares(
    State(_state): State<AppState>,
    Path(contract_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<PurchaseSharesRequest>,
) -> Result<ResponseJson<PurchaseSharesResponse>, StatusCode> {
    let transaction_id = Uuid::new_v4();
    let ownership_percentage = (request.shares_to_purchase as f64 / 1000.0) * 100.0; // Assuming 1000 total shares
    let estimated_returns = request.investment_amount * 0.12; // 12% estimated annual return

    let response = PurchaseSharesResponse {
        transaction_id,
        shares_purchased: request.shares_to_purchase,
        total_cost: request.investment_amount,
        ownership_percentage,
        estimated_returns,
        blockchain_tx_hash: Some(format!("0x{:x}", transaction_id.as_u128())),
    };

    tracing::info!("✅ User {} purchased {} shares of contract {}", claims.sub, request.shares_to_purchase, contract_id);
    Ok(ResponseJson(response))
}

/// GET /api/v1/ownership/contracts/{id} - Get contract details
pub async fn get_contract_details(
    State(_state): State<AppState>,
    Path(contract_id): Path<Uuid>,
    _claims: Claims,
) -> Result<ResponseJson<ContractDetailsResponse>, StatusCode> {
    // Mock response - replace with real data fetching
    let response = ContractDetailsResponse {
        contract_id,
        song_id: Uuid::new_v4(),
        song_title: "Sample Song".to_string(),
        artist_name: "Sample Artist".to_string(),
        total_shares: 1000,
        shares_sold: 750,
        shares_available: 250,
        price_per_share: 10.0,
        total_invested: 7500.0,
        investor_count: 15,
        monthly_revenue: 450.0,
        total_revenue: 5400.0,
        status: "active".to_string(),
        created_at: Utc::now(),
        performance_metrics: PerformanceMetrics {
            roi_percentage: 12.5,
            monthly_growth: 2.1,
            revenue_per_share: 7.2,
            risk_score: 3.5,
        },
    };

    tracing::info!("📊 Contract details requested for {}", contract_id);
    Ok(ResponseJson(response))
}

/// GET /api/v1/ownership/users/{id}/portfolio - Get user portfolio
pub async fn get_user_portfolio(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<UserPortfolioResponse>, StatusCode> {
    // Verify user can access this portfolio
    if claims.sub != user_id.to_string() && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Mock portfolio data
    let positions = vec![
        PortfolioPosition {
            contract_id: Uuid::new_v4(),
            song_title: "Hit Song #1".to_string(),
            artist_name: "Popular Artist".to_string(),
            shares_owned: 50,
            ownership_percentage: 5.0,
            initial_investment: 500.0,
            current_value: 625.0,
            revenue_earned: 75.0,
            purchase_date: Utc::now() - chrono::Duration::days(90),
        },
        PortfolioPosition {
            contract_id: Uuid::new_v4(),
            song_title: "Indie Gem".to_string(),
            artist_name: "Rising Star".to_string(),
            shares_owned: 25,
            ownership_percentage: 2.5,
            initial_investment: 250.0,
            current_value: 280.0,
            revenue_earned: 18.0,
            purchase_date: Utc::now() - chrono::Duration::days(45),
        },
    ];

    let total_invested = positions.iter().map(|p| p.initial_investment).sum();
    let current_value = positions.iter().map(|p| p.current_value).sum();
    let total_returns = positions.iter().map(|p| p.revenue_earned).sum();
    let roi_percentage = if total_invested > 0.0 { 
        ((current_value - total_invested) / total_invested) * 100.0 
    } else { 0.0 };

    let response = UserPortfolioResponse {
        user_id,
        total_invested,
        current_value,
        total_returns,
        roi_percentage,
        active_contracts: positions.len() as u32,
        positions,
    };

    tracing::info!("📈 Portfolio requested for user {}", user_id);
    Ok(ResponseJson(response))
}

/// GET /api/v1/ownership/contracts - List contracts with filtering
pub async fn list_contracts(
    State(_state): State<AppState>,
    Query(params): Query<ListContractsQuery>,
    _claims: Claims,
) -> Result<ResponseJson<Vec<ContractSummary>>, StatusCode> {
    // Mock contract list
    let contracts = vec![
        ContractSummary {
            contract_id: Uuid::new_v4(),
            song_title: "Trending Hit".to_string(),
            artist_name: "Chart Topper".to_string(),
            total_shares: 1000,
            shares_available: 300,
            price_per_share: 15.0,
            roi_percentage: 18.5,
            risk_level: "Medium".to_string(),
            monthly_revenue: 600.0,
            created_at: Utc::now() - chrono::Duration::days(30),
        },
        ContractSummary {
            contract_id: Uuid::new_v4(),
            song_title: "Underground Classic".to_string(),
            artist_name: "Cult Artist".to_string(),
            total_shares: 500,
            shares_available: 150,
            price_per_share: 8.0,
            roi_percentage: 22.1,
            risk_level: "High".to_string(),
            monthly_revenue: 200.0,
            created_at: Utc::now() - chrono::Duration::days(15),
        },
    ];

    tracing::info!("📋 Contract list requested with params: {:?}", params);
    Ok(ResponseJson(contracts))
}

#[derive(Debug, Deserialize)]
pub struct ListContractsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub artist_id: Option<Uuid>,
    pub min_roi: Option<f64>,
    pub max_risk: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContractSummary {
    pub contract_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub total_shares: u32,
    pub shares_available: u32,
    pub price_per_share: f64,
    pub roi_percentage: f64,
    pub risk_level: String,
    pub monthly_revenue: f64,
    pub created_at: DateTime<Utc>,
}

/// POST /api/v1/ownership/contracts/{id}/distribute - Distribute revenue
pub async fn distribute_revenue(
    State(_state): State<AppState>,
    Path(contract_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<DistributeRevenueRequest>,
) -> Result<ResponseJson<DistributeRevenueResponse>, StatusCode> {
    // Only admins and artists can distribute revenue
    if claims.role != "admin" && claims.role != "artist" {
        return Err(StatusCode::FORBIDDEN);
    }

    let distribution_id = Uuid::new_v4();
    let platform_fee = request.total_revenue * 0.1; // 10% platform fee
    let artist_share = request.total_revenue * 0.5; // 50% to artist
    let investor_share = request.total_revenue - platform_fee - artist_share; // 40% to investors

    let response = DistributeRevenueResponse {
        distribution_id,
        contract_id,
        total_revenue: request.total_revenue,
        platform_fee,
        artist_share,
        investor_share,
        investors_paid: 12, // Mock number
        transaction_hashes: vec![
            format!("0x{:x}", Uuid::new_v4().as_u128()),
            format!("0x{:x}", Uuid::new_v4().as_u128()),
        ],
        distributed_at: Utc::now(),
    };

    tracing::info!("💰 Revenue distributed for contract {}: ${}", contract_id, request.total_revenue);
    Ok(ResponseJson(response))
}

#[derive(Debug, Deserialize)]
pub struct DistributeRevenueRequest {
    pub total_revenue: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub source: String, // "streaming", "sales", "licensing", etc.
}

#[derive(Debug, Serialize)]
pub struct DistributeRevenueResponse {
    pub distribution_id: Uuid,
    pub contract_id: Uuid,
    pub total_revenue: f64,
    pub platform_fee: f64,
    pub artist_share: f64,
    pub investor_share: f64,
    pub investors_paid: u32,
    pub transaction_hashes: Vec<String>,
    pub distributed_at: DateTime<Utc>,
} 