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
pub struct StartListenSessionRequest {
    pub song_id: Uuid,
    pub device_info: DeviceInfo,
    pub user_tier: String, // "free", "premium", "vip"
    pub boost_multiplier: Option<f64>,
    pub location: Option<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_type: String, // "mobile", "desktop", "smart_speaker"
    pub platform: String,    // "ios", "android", "web", "alexa"
    pub app_version: String,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub country: String,
    pub city: Option<String>,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartListenSessionResponse {
    pub session_id: Uuid,
    pub song_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub expected_reward: f64,
    pub boost_multiplier: f64,
    pub quality_threshold: f64,
    pub minimum_duration_seconds: u32,
    pub zk_challenge: String,
    pub session_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteListenSessionRequest {
    pub session_id: Uuid,
    pub listen_duration_seconds: u32,
    pub actual_quality_score: f64,
    pub completion_percentage: f64,
    pub zk_proof: String,
    pub engagement_metrics: EngagementMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub skips: u32,
    pub repeats: u32,
    pub volume_changes: u32,
    pub pause_count: u32,
    pub peak_volume: f64,
    pub interaction_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteListenSessionResponse {
    pub session_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub final_reward: f64,
    pub quality_score: f64,
    pub verification_status: String,
    pub reward_breakdown: RewardBreakdown,
    pub next_tier_progress: TierProgress,
    pub achievement_unlocked: Option<Achievement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardBreakdown {
    pub base_reward: f64,
    pub quality_bonus: f64,
    pub tier_bonus: f64,
    pub boost_bonus: f64,
    pub loyalty_bonus: f64,
    pub total_reward: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TierProgress {
    pub current_tier: String,
    pub points_earned: u32,
    pub points_to_next_tier: u32,
    pub next_tier: String,
    pub progress_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub reward_points: u32,
    pub rarity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRewardsResponse {
    pub user_id: Uuid,
    pub total_rewards_earned: f64,
    pub current_balance: f64,
    pub lifetime_listens: u32,
    pub current_tier: String,
    pub tier_benefits: TierBenefits,
    pub recent_sessions: Vec<SessionSummary>,
    pub achievements: Vec<Achievement>,
    pub statistics: UserStatistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TierBenefits {
    pub tier_name: String,
    pub reward_multiplier: f64,
    pub bonus_features: Vec<String>,
    pub exclusive_content: bool,
    pub priority_support: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub duration_seconds: u32,
    pub reward_earned: f64,
    pub quality_score: f64,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatistics {
    pub daily_average_listens: f64,
    pub favorite_genres: Vec<String>,
    pub listening_streak_days: u32,
    pub total_listening_time_hours: f64,
    pub avg_quality_score: f64,
    pub preferred_listening_times: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributeRewardsRequest {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_reward_pool: f64,
    pub distribution_type: String, // "daily", "weekly", "monthly", "special"
    pub bonus_multipliers: Option<BonusMultipliers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BonusMultipliers {
    pub new_user_bonus: f64,
    pub loyalty_bonus: f64,
    pub quality_bonus: f64,
    pub volume_bonus: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributeRewardsResponse {
    pub distribution_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_distributed: f64,
    pub users_rewarded: u32,
    pub sessions_processed: u32,
    pub average_reward_per_user: f64,
    pub top_earners: Vec<TopEarner>,
    pub distribution_stats: DistributionStats,
    pub processed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopEarner {
    pub user_id: String,
    pub username: String,
    pub total_earned: f64,
    pub sessions_completed: u32,
    pub avg_quality_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionStats {
    pub total_listening_hours: f64,
    pub unique_songs_played: u32,
    pub quality_distribution: QualityDistribution,
    pub tier_distribution: TierDistribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub excellent: u32, // 90-100%
    pub good: u32,      // 70-89%
    pub average: u32,   // 50-69%
    pub poor: u32,      // <50%
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TierDistribution {
    pub free: u32,
    pub premium: u32,
    pub vip: u32,
}

#[derive(Debug, Deserialize)]
pub struct RewardsQuery {
    pub user_id: Option<Uuid>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub min_reward: Option<f64>,
    pub tier: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// ====== HANDLERS ======

/// POST /api/v1/listen/sessions - Start new listening session
pub async fn start_listen_session(
    State(_state): State<AppState>,
    claims: Claims,
    Json(request): Json<StartListenSessionRequest>,
) -> Result<ResponseJson<StartListenSessionResponse>, StatusCode> {
    let session_id = Uuid::new_v4();
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Calculate expected reward based on user tier and boost multiplier
    let base_reward = match request.user_tier.as_str() {
        "free" => 0.01,
        "premium" => 0.02,
        "vip" => 0.05,
        _ => 0.01,
    };
    
    let boost_multiplier = request.boost_multiplier.unwrap_or(1.0);
    let expected_reward = base_reward * boost_multiplier;
    
    // Generate ZK challenge for proof-of-listen
    let zk_challenge = format!("challenge_{:x}", Uuid::new_v4().as_u128());
    let session_token = format!("session_{:x}", session_id.as_u128());

    let response = StartListenSessionResponse {
        session_id,
        song_id: request.song_id,
        user_id,
        started_at: Utc::now(),
        expected_reward,
        boost_multiplier,
        quality_threshold: 0.7, // 70% quality threshold
        minimum_duration_seconds: 30, // Minimum 30 seconds
        zk_challenge,
        session_token,
    };

    tracing::info!("🎧 Listen session {} started by user {} for song {}", 
                  session_id, user_id, request.song_id);
    Ok(ResponseJson(response))
}

/// PUT /api/v1/listen/sessions/{id}/complete - Complete listening session
pub async fn complete_listen_session(
    State(_state): State<AppState>,
    Path(session_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<CompleteListenSessionRequest>,
) -> Result<ResponseJson<CompleteListenSessionResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Verify ZK proof (mock verification)
    let verification_status = if request.zk_proof.len() > 10 {
        "verified"
    } else {
        "failed"
    };
    
    if verification_status == "failed" {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Calculate reward breakdown
    let base_reward = 0.02;
    let quality_bonus = if request.actual_quality_score > 0.8 { 0.01 } else { 0.0 };
    let tier_bonus = 0.005; // Premium tier bonus
    let boost_bonus = 0.01; // From NFT boost
    let loyalty_bonus = 0.002; // Loyalty program
    let total_reward = base_reward + quality_bonus + tier_bonus + boost_bonus + loyalty_bonus;
    
    let reward_breakdown = RewardBreakdown {
        base_reward,
        quality_bonus,
        tier_bonus,
        boost_bonus,
        loyalty_bonus,
        total_reward,
    };
    
    let tier_progress = TierProgress {
        current_tier: "premium".to_string(),
        points_earned: 15,
        points_to_next_tier: 85,
        next_tier: "vip".to_string(),
        progress_percentage: 15.0,
    };
    
    // Check for achievement unlock
    let achievement = if request.listen_duration_seconds > 240 {
        Some(Achievement {
            id: "full_listen_master".to_string(),
            name: "Full Listen Master".to_string(),
            description: "Listened to a full song without skipping".to_string(),
            reward_points: 50,
            rarity: "common".to_string(),
        })
    } else {
        None
    };

    let response = CompleteListenSessionResponse {
        session_id,
        completed_at: Utc::now(),
        final_reward: total_reward,
        quality_score: request.actual_quality_score,
        verification_status: verification_status.to_string(),
        reward_breakdown,
        tier_progress,
        achievement_unlocked: achievement,
    };

    tracing::info!("✅ Listen session {} completed by user {}, reward: {}", 
                  session_id, user_id, total_reward);
    Ok(ResponseJson(response))
}

/// GET /api/v1/listen/users/{id}/rewards - Get user's reward summary
pub async fn get_user_rewards(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<UserRewardsResponse>, StatusCode> {
    // Verify user can access this data
    if claims.sub != user_id.to_string() && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Mock user data
    let recent_sessions = vec![
        SessionSummary {
            session_id: Uuid::new_v4(),
            song_title: "Summer Vibes".to_string(),
            artist_name: "DJ Awesome".to_string(),
            duration_seconds: 245,
            reward_earned: 0.045,
            quality_score: 0.87,
            completed_at: Utc::now() - chrono::Duration::hours(2),
        },
        SessionSummary {
            session_id: Uuid::new_v4(),
            song_title: "Deep Beats".to_string(),
            artist_name: "Bass Master".to_string(),
            duration_seconds: 189,
            reward_earned: 0.032,
            quality_score: 0.73,
            completed_at: Utc::now() - chrono::Duration::hours(5),
        },
    ];

    let achievements = vec![
        Achievement {
            id: "first_listen".to_string(),
            name: "First Listen".to_string(),
            description: "Completed your first listening session".to_string(),
            reward_points: 10,
            rarity: "common".to_string(),
        },
        Achievement {
            id: "quality_listener".to_string(),
            name: "Quality Listener".to_string(),
            description: "Maintained 80%+ quality score for 10 sessions".to_string(),
            reward_points: 100,
            rarity: "rare".to_string(),
        },
    ];

    let tier_benefits = TierBenefits {
        tier_name: "Premium".to_string(),
        reward_multiplier: 1.5,
        bonus_features: vec![
            "Ad-free listening".to_string(),
            "High-quality audio".to_string(),
            "Offline downloads".to_string(),
        ],
        exclusive_content: true,
        priority_support: true,
    };

    let statistics = UserStatistics {
        daily_average_listens: 12.5,
        favorite_genres: vec!["Electronic".to_string(), "Hip Hop".to_string()],
        listening_streak_days: 15,
        total_listening_time_hours: 127.3,
        avg_quality_score: 0.82,
        preferred_listening_times: vec!["Evening".to_string(), "Morning".to_string()],
    };

    let response = UserRewardsResponse {
        user_id,
        total_rewards_earned: 15.67,
        current_balance: 12.34,
        lifetime_listens: 1247,
        current_tier: "premium".to_string(),
        tier_benefits,
        recent_sessions,
        achievements,
        statistics,
    };

    tracing::info!("📊 User rewards requested for {}", user_id);
    Ok(ResponseJson(response))
}

/// POST /api/v1/listen/rewards/distribute - Distribute rewards for a period
pub async fn distribute_rewards(
    State(_state): State<AppState>,
    claims: Claims,
    Json(request): Json<DistributeRewardsRequest>,
) -> Result<ResponseJson<DistributeRewardsResponse>, StatusCode> {
    // Only admins can distribute rewards
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let distribution_id = Uuid::new_v4();
    let users_rewarded = 1250u32;
    let sessions_processed = 15670u32;
    let total_distributed = request.total_reward_pool * 0.95; // 5% held for platform
    let average_reward_per_user = total_distributed / users_rewarded as f64;

    let top_earners = vec![
        TopEarner {
            user_id: Uuid::new_v4().to_string(),
            username: "MusicLover123".to_string(),
            total_earned: 2.45,
            sessions_completed: 87,
            avg_quality_score: 0.94,
        },
        TopEarner {
            user_id: Uuid::new_v4().to_string(),
            username: "BeatSeeker".to_string(),
            total_earned: 2.12,
            sessions_completed: 76,
            avg_quality_score: 0.89,
        },
    ];

    let distribution_stats = DistributionStats {
        total_listening_hours: 4567.8,
        unique_songs_played: 2834,
        quality_distribution: QualityDistribution {
            excellent: 3245,
            good: 8760,
            average: 3012,
            poor: 653,
        },
        tier_distribution: TierDistribution {
            free: 8950,
            premium: 5820,
            vip: 900,
        },
    };

    let response = DistributeRewardsResponse {
        distribution_id,
        period_start: request.period_start,
        period_end: request.period_end,
        total_distributed,
        users_rewarded,
        sessions_processed,
        average_reward_per_user,
        top_earners,
        distribution_stats,
        processed_at: Utc::now(),
    };

    tracing::info!("💰 Rewards distributed: {} to {} users", 
                  total_distributed, users_rewarded);
    Ok(ResponseJson(response))
}

/// GET /api/v1/listen/analytics - Get listening analytics
pub async fn get_listen_analytics(
    State(_state): State<AppState>,
    Query(params): Query<RewardsQuery>,
    claims: Claims,
) -> Result<ResponseJson<AnalyticsResponse>, StatusCode> {
    // Only admins can view system analytics
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let response = AnalyticsResponse {
        total_sessions: 156780,
        total_rewards_distributed: 4567.89,
        active_users: 12450,
        average_session_duration: 187.5,
        quality_score_average: 0.78,
        top_performing_songs: vec![
            SongPerformance {
                song_id: Uuid::new_v4(),
                title: "Summer Vibes".to_string(),
                artist: "DJ Awesome".to_string(),
                total_listens: 2450,
                total_rewards: 89.67,
                avg_quality: 0.85,
            },
        ],
        tier_breakdown: TierDistribution {
            free: 8950,
            premium: 3200,
            vip: 300,
        },
        fraud_detection: FraudDetectionStats {
            suspicious_sessions: 45,
            blocked_sessions: 12,
            fraud_rate: 0.029,
        },
    };

    tracing::info!("📈 Listen analytics requested with params: {:?}", params);
    Ok(ResponseJson(response))
}

#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub total_sessions: u32,
    pub total_rewards_distributed: f64,
    pub active_users: u32,
    pub average_session_duration: f64,
    pub quality_score_average: f64,
    pub top_performing_songs: Vec<SongPerformance>,
    pub tier_breakdown: TierDistribution,
    pub fraud_detection: FraudDetectionStats,
}

#[derive(Debug, Serialize)]
pub struct SongPerformance {
    pub song_id: Uuid,
    pub title: String,
    pub artist: String,
    pub total_listens: u32,
    pub total_rewards: f64,
    pub avg_quality: f64,
}

#[derive(Debug, Serialize)]
pub struct FraudDetectionStats {
    pub suspicious_sessions: u32,
    pub blocked_sessions: u32,
    pub fraud_rate: f64,
} 