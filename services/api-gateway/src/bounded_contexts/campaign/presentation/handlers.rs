use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::auth::Claims;
use crate::services::AppState;

// ====== REQUEST/RESPONSE TYPES ======

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub song_id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub boost_multiplier: f64,
    pub target_revenue: Option<f64>,
    pub artwork_ipfs_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignResponse {
    pub campaign_id: Uuid,
    pub name: String,
    pub song_id: Uuid,
    pub status: String,
    pub nft_contract_address: Option<String>,
    pub max_nfts: u32,
    pub nft_price: f64,
    pub estimated_revenue: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseNFTRequest {
    pub quantity: u32,
    pub payment_method: String, // "crypto", "credit_card", "platform_balance"
    pub wallet_address: String,
    pub boost_preferences: Option<BoostPreferences>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoostPreferences {
    pub auto_apply_boost: bool,
    pub boost_duration_days: Option<u32>,
    pub target_songs: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseNFTResponse {
    pub transaction_id: Uuid,
    pub campaign_id: Uuid,
    pub nft_ids: Vec<String>,
    pub quantity: u32,
    pub total_cost: f64,
    pub boost_multiplier: f64,
    pub boost_expires_at: Option<DateTime<Utc>>,
    pub blockchain_tx_hash: Option<String>,
    pub estimated_delivery: String,
    pub purchase_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignDetailsResponse {
    pub campaign_id: Uuid,
    pub name: String,
    pub description: String,
    pub song_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub status: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub nfts_sold: u32,
    pub nfts_remaining: u32,
    pub current_revenue: f64,
    pub target_revenue: Option<f64>,
    pub completion_percentage: f64,
    pub boost_multiplier: f64,
    pub artwork_url: Option<String>,
    pub analytics: CampaignAnalytics,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignAnalytics {
    pub total_buyers: u32,
    pub unique_buyers: u32,
    pub average_purchase_size: f64,
    pub daily_sales: Vec<DailySales>,
    pub top_buyers: Vec<TopBuyer>,
    pub conversion_rate: f64,
    pub social_engagement: SocialEngagement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySales {
    pub date: String,
    pub nfts_sold: u32,
    pub revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopBuyer {
    pub user_id: String,
    pub username: String,
    pub nfts_purchased: u32,
    pub total_spent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialEngagement {
    pub shares: u32,
    pub likes: u32,
    pub comments: u32,
    pub reach: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCampaignsResponse {
    pub campaigns: Vec<CampaignSummary>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignSummary {
    pub campaign_id: Uuid,
    pub name: String,
    pub song_title: String,
    pub artist_name: String,
    pub status: String,
    pub nft_price: f64,
    pub nfts_sold: u32,
    pub max_nfts: u32,
    pub completion_percentage: f64,
    pub current_revenue: f64,
    pub boost_multiplier: f64,
    pub ends_at: DateTime<Utc>,
    pub popularity_score: f64,
}

#[derive(Debug, Deserialize)]
pub struct ListCampaignsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<String>, // "active", "ended", "upcoming", "all"
    pub artist_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub sort_by: Option<String>, // "popularity", "ending_soon", "newest", "revenue"
    pub search: Option<String>,
}

// ====== HANDLERS ======

/// POST /api/v1/campaigns - Create new campaign
pub async fn create_campaign(
    State(_state): State<AppState>,
    claims: Claims,
    Json(request): Json<CreateCampaignRequest>,
) -> Result<ResponseJson<CreateCampaignResponse>, StatusCode> {
    // Verify user is artist or admin
    if claims.role != "artist" && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let campaign_id = Uuid::new_v4();
    let estimated_revenue = request.nft_price * request.max_nfts as f64;

    // Mock NFT contract deployment
    let nft_contract_address = Some(format!("0x{:x}", Uuid::new_v4().as_u128()));

    let response = CreateCampaignResponse {
        campaign_id,
        name: request.name,
        song_id: request.song_id,
        status: "upcoming".to_string(),
        nft_contract_address,
        max_nfts: request.max_nfts,
        nft_price: request.nft_price,
        estimated_revenue,
        created_at: Utc::now(),
    };

    tracing::info!("✅ Created campaign {} for song {}", campaign_id, request.song_id);
    Ok(ResponseJson(response))
}

/// POST /api/v1/campaigns/{id}/activate - Activate campaign
pub async fn activate_campaign(
    State(_state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<ActivateCampaignResponse>, StatusCode> {
    // Only artists and admins can activate campaigns
    if claims.role != "artist" && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let response = ActivateCampaignResponse {
        campaign_id,
        status: "active".to_string(),
        activated_at: Utc::now(),
        message: "Campaign activated successfully".to_string(),
    };

    tracing::info!("🚀 Campaign {} activated", campaign_id);
    Ok(ResponseJson(response))
}

#[derive(Debug, Serialize)]
pub struct ActivateCampaignResponse {
    pub campaign_id: Uuid,
    pub status: String,
    pub activated_at: DateTime<Utc>,
    pub message: String,
}

/// POST /api/v1/campaigns/{id}/purchase - Purchase NFT from campaign
pub async fn purchase_nft(
    State(_state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<PurchaseNFTRequest>,
) -> Result<ResponseJson<PurchaseNFTResponse>, StatusCode> {
    let transaction_id = Uuid::new_v4();
    let nft_price = 25.0; // Mock price
    let total_cost = nft_price * request.quantity as f64;
    let boost_multiplier = 2.0; // Mock boost
    
    // Generate mock NFT IDs
    let nft_ids: Vec<String> = (0..request.quantity)
        .map(|i| format!("NFT_{:x}_{}", Uuid::new_v4().as_u128(), i))
        .collect();

    let boost_expires_at = Some(Utc::now() + chrono::Duration::days(30));

    let response = PurchaseNFTResponse {
        transaction_id,
        campaign_id,
        nft_ids,
        quantity: request.quantity,
        total_cost,
        boost_multiplier,
        boost_expires_at,
        blockchain_tx_hash: Some(format!("0x{:x}", transaction_id.as_u128())),
        estimated_delivery: "immediate".to_string(),
        purchase_date: Utc::now(),
    };

    tracing::info!("💎 User {} purchased {} NFTs from campaign {}", 
                  claims.sub, request.quantity, campaign_id);
    Ok(ResponseJson(response))
}

/// GET /api/v1/campaigns/{id} - Get campaign details
pub async fn get_campaign_details(
    State(_state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    _claims: Claims,
) -> Result<ResponseJson<CampaignDetailsResponse>, StatusCode> {
    // Mock campaign data
    let nfts_sold = 750u32;
    let max_nfts = 1000u32;
    let completion_percentage = (nfts_sold as f64 / max_nfts as f64) * 100.0;

    let analytics = CampaignAnalytics {
        total_buyers: 125,
        unique_buyers: 98,
        average_purchase_size: 6.12,
        daily_sales: vec![
            DailySales {
                date: "2024-01-15".to_string(),
                nfts_sold: 45,
                revenue: 1125.0,
            },
            DailySales {
                date: "2024-01-16".to_string(),
                nfts_sold: 52,
                revenue: 1300.0,
            },
        ],
        top_buyers: vec![
            TopBuyer {
                user_id: Uuid::new_v4().to_string(),
                username: "CryptoWhale123".to_string(),
                nfts_purchased: 50,
                total_spent: 1250.0,
            },
        ],
        conversion_rate: 8.5,
        social_engagement: SocialEngagement {
            shares: 256,
            likes: 1240,
            comments: 89,
            reach: 15600,
        },
    };

    let response = CampaignDetailsResponse {
        campaign_id,
        name: "Epic Summer Drop".to_string(),
        description: "Limited edition NFTs with exclusive listening boosts".to_string(),
        song_id: Uuid::new_v4(),
        song_title: "Summer Vibes".to_string(),
        artist_name: "DJ Awesome".to_string(),
        status: "active".to_string(),
        start_date: Utc::now() - chrono::Duration::days(15),
        end_date: Utc::now() + chrono::Duration::days(45),
        nft_price: 25.0,
        max_nfts,
        nfts_sold,
        nfts_remaining: max_nfts - nfts_sold,
        current_revenue: nfts_sold as f64 * 25.0,
        target_revenue: Some(25000.0),
        completion_percentage,
        boost_multiplier: 2.0,
        artwork_url: Some("https://ipfs.io/ipfs/QmSampleHash".to_string()),
        analytics,
        created_at: Utc::now() - chrono::Duration::days(20),
    };

    tracing::info!("📊 Campaign details requested for {}", campaign_id);
    Ok(ResponseJson(response))
}

/// GET /api/v1/campaigns - List campaigns with filtering
pub async fn list_campaigns(
    State(_state): State<AppState>,
    Query(params): Query<ListCampaignsQuery>,
    _claims: Claims,
) -> Result<ResponseJson<ListCampaignsResponse>, StatusCode> {
    // Mock campaign data
    let campaigns = vec![
        CampaignSummary {
            campaign_id: Uuid::new_v4(),
            name: "Epic Summer Drop".to_string(),
            song_title: "Summer Vibes".to_string(),
            artist_name: "DJ Awesome".to_string(),
            status: "active".to_string(),
            nft_price: 25.0,
            nfts_sold: 750,
            max_nfts: 1000,
            completion_percentage: 75.0,
            current_revenue: 18750.0,
            boost_multiplier: 2.0,
            ends_at: Utc::now() + chrono::Duration::days(45),
            popularity_score: 8.7,
        },
        CampaignSummary {
            campaign_id: Uuid::new_v4(),
            name: "Underground Exclusive".to_string(),
            song_title: "Deep Beats".to_string(),
            artist_name: "Bass Master".to_string(),
            status: "active".to_string(),
            nft_price: 15.0,
            nfts_sold: 400,
            max_nfts: 500,
            completion_percentage: 80.0,
            current_revenue: 6000.0,
            boost_multiplier: 1.5,
            ends_at: Utc::now() + chrono::Duration::days(20),
            popularity_score: 7.2,
        },
    ];

    let page = params.page.unwrap_or(1);
    let per_page = params.limit.unwrap_or(20);

    let response = ListCampaignsResponse {
        campaigns,
        total: 2,
        page,
        per_page,
        has_more: false,
    };

    tracing::info!("📋 Campaign list requested with filters: {:?}", params);
    Ok(ResponseJson(response))
}

/// GET /api/v1/campaigns/{id}/analytics - Get detailed analytics
pub async fn get_campaign_analytics(
    State(_state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<DetailedAnalyticsResponse>, StatusCode> {
    // Only campaign owner or admin can view detailed analytics
    if claims.role != "artist" && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let response = DetailedAnalyticsResponse {
        campaign_id,
        performance: PerformanceMetrics {
            total_revenue: 18750.0,
            net_revenue: 16875.0, // After platform fees
            roi_percentage: 287.5,
            conversion_rate: 8.5,
            engagement_rate: 12.3,
            viral_coefficient: 1.4,
        },
        demographics: Demographics {
            age_distribution: vec![
                AgeGroup { range: "18-24".to_string(), percentage: 35.2 },
                AgeGroup { range: "25-34".to_string(), percentage: 42.1 },
                AgeGroup { range: "35-44".to_string(), percentage: 18.7 },
                AgeGroup { range: "45+".to_string(), percentage: 4.0 },
            ],
            geographic_distribution: vec![
                GeoData { country: "US".to_string(), buyers: 45, revenue: 8125.0 },
                GeoData { country: "UK".to_string(), buyers: 23, revenue: 4200.0 },
                GeoData { country: "CA".to_string(), buyers: 18, revenue: 3150.0 },
            ],
        },
        trends: TrendAnalysis {
            daily_metrics: generate_mock_daily_metrics(),
            peak_hours: vec![19, 20, 21, 22], // Evening hours
            seasonal_patterns: vec![
                SeasonalPattern { month: "January".to_string(), multiplier: 1.2 },
                SeasonalPattern { month: "December".to_string(), multiplier: 1.8 },
            ],
        },
        projections: ProjectionData {
            estimated_final_sales: 980,
            projected_revenue: 24500.0,
            completion_likelihood: 95.2,
            optimal_pricing: OptimalPricing {
                suggested_price: 26.50,
                demand_elasticity: -0.8,
                price_sensitivity: 0.65,
            },
        },
    };

    tracing::info!("📈 Detailed analytics requested for campaign {}", campaign_id);
    Ok(ResponseJson(response))
}

#[derive(Debug, Serialize)]
pub struct DetailedAnalyticsResponse {
    pub campaign_id: Uuid,
    pub performance: PerformanceMetrics,
    pub demographics: Demographics,
    pub trends: TrendAnalysis,
    pub projections: ProjectionData,
}

#[derive(Debug, Serialize)]
pub struct PerformanceMetrics {
    pub total_revenue: f64,
    pub net_revenue: f64,
    pub roi_percentage: f64,
    pub conversion_rate: f64,
    pub engagement_rate: f64,
    pub viral_coefficient: f64,
}

#[derive(Debug, Serialize)]
pub struct Demographics {
    pub age_distribution: Vec<AgeGroup>,
    pub geographic_distribution: Vec<GeoData>,
}

#[derive(Debug, Serialize)]
pub struct AgeGroup {
    pub range: String,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct GeoData {
    pub country: String,
    pub buyers: u32,
    pub revenue: f64,
}

#[derive(Debug, Serialize)]
pub struct TrendAnalysis {
    pub daily_metrics: Vec<DailyMetric>,
    pub peak_hours: Vec<u32>,
    pub seasonal_patterns: Vec<SeasonalPattern>,
}

#[derive(Debug, Serialize)]
pub struct DailyMetric {
    pub date: String,
    pub sales: u32,
    pub revenue: f64,
    pub unique_visitors: u32,
    pub conversion_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct SeasonalPattern {
    pub month: String,
    pub multiplier: f64,
}

#[derive(Debug, Serialize)]
pub struct ProjectionData {
    pub estimated_final_sales: u32,
    pub projected_revenue: f64,
    pub completion_likelihood: f64,
    pub optimal_pricing: OptimalPricing,
}

#[derive(Debug, Serialize)]
pub struct OptimalPricing {
    pub suggested_price: f64,
    pub demand_elasticity: f64,
    pub price_sensitivity: f64,
}

fn generate_mock_daily_metrics() -> Vec<DailyMetric> {
    let mut metrics = Vec::new();
    for i in 0..14 {
        let date = (Utc::now() - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
        metrics.push(DailyMetric {
            date,
            sales: 25 + (i as u32 * 3),
            revenue: (25.0 + (i as f64 * 3.0)) * 25.0,
            unique_visitors: 450 + (i as u32 * 25),
            conversion_rate: 5.5 + (i as f64 * 0.3),
        });
    }
    metrics
} 