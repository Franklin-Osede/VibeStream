use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, RwLock},
    time::{interval, Instant},
};
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::event_schema::{DomainEventWrapper, EventPayload};
use crate::shared::domain::errors::AppError;

/// Real-time stream processor for VibeStream analytics
/// 
/// Processes millions of events per second to generate:
/// - Real-time revenue analytics
/// - User engagement metrics  
/// - Fraud detection alerts
/// - Market sentiment analysis
/// - Artist performance metrics
#[derive(Debug)]
pub struct StreamProcessor {
    config: StreamProcessorConfig,
    analytics_store: Arc<RwLock<AnalyticsStore>>,
    fraud_detector: Arc<FraudDetector>,
    revenue_calculator: Arc<RevenueCalculator>,
    market_analyzer: Arc<MarketAnalyzer>,
    event_sender: mpsc::Sender<ProcessedEvent>,
}

impl StreamProcessor {
    pub async fn new(
        config: StreamProcessorConfig,
        event_sender: mpsc::Sender<ProcessedEvent>,
    ) -> Result<Self, AppError> {
        info!("🚀 Initializing VibeStream Stream Processor");

        Ok(Self {
            config,
            analytics_store: Arc::new(RwLock::new(AnalyticsStore::new())),
            fraud_detector: Arc::new(FraudDetector::new()),
            revenue_calculator: Arc::new(RevenueCalculator::new()),
            market_analyzer: Arc::new(MarketAnalyzer::new()),
            event_sender,
        })
    }

    /// Process incoming domain events in real-time
    pub async fn process_event(&self, event: DomainEventWrapper) -> Result<(), AppError> {
        let start_time = Instant::now();
        
        debug!("⚡ Processing event {} of type {}", 
               event.metadata.event_id, event.metadata.event_type);

        match &event.payload {
            // Listen Session Analytics
            EventPayload::ListenSessionCompleted(payload) => {
                self.process_listen_session_completed(payload).await?;
            }
            
            // Revenue Analytics
            EventPayload::SharesPurchased(payload) => {
                self.process_shares_purchased(payload).await?;
            }
            
            EventPayload::RevenueDistributed(payload) => {
                self.process_revenue_distributed(payload).await?;
            }
            
            // User Behavior Analytics
            EventPayload::UserRegistered(payload) => {
                self.process_user_registered(payload).await?;
            }
            
            // Music Analytics
            EventPayload::SongUploaded(payload) => {
                self.process_song_uploaded(payload).await?;
            }
            
            // Campaign Analytics
            EventPayload::CampaignCreated(payload) => {
                self.process_campaign_created(payload).await?;
            }
            
            // System monitoring
            EventPayload::SystemHealthCheck(payload) => {
                self.process_system_health_check(payload).await?;
            }
            
            _ => {
                debug!("🔄 Event type {} not processed by stream processor", event.metadata.event_type);
            }
        }

        let processing_time = start_time.elapsed();
        self.record_processing_metrics(&event.metadata.event_type, processing_time).await;

        Ok(())
    }

    /// Process listen session completion for real-time analytics
    async fn process_listen_session_completed(
        &self,
        payload: &super::event_schema::ListenSessionCompletedPayload,
    ) -> Result<(), AppError> {
        // Real-time revenue calculation
        let estimated_revenue = self.revenue_calculator
            .calculate_listen_revenue(payload.quality_score, payload.listen_duration_seconds)
            .await;

        // Fraud detection
        let fraud_score = self.fraud_detector
            .analyze_listen_session(
                payload.user_id,
                payload.song_id,
                payload.listen_duration_seconds,
                &payload.zk_proof_hash,
            )
            .await?;

        // Update analytics store
        {
            let mut store = self.analytics_store.write().await;
            
            // User engagement metrics
            store.update_user_engagement(payload.user_id, payload.listen_duration_seconds);
            
            // Song popularity metrics
            store.update_song_metrics(payload.song_id, estimated_revenue, payload.quality_score);
            
            // Real-time counters
            store.increment_daily_listens();
            store.add_daily_revenue(estimated_revenue);
        }

        // Generate alerts if needed
        if fraud_score > self.config.fraud_threshold {
            self.send_fraud_alert(payload.user_id, payload.session_id, fraud_score).await?;
        }

        // Send processed event for further analytics
        let processed_event = ProcessedEvent::ListenAnalytics {
            session_id: payload.session_id,
            user_id: payload.user_id,
            song_id: payload.song_id,
            estimated_revenue,
            fraud_score,
            quality_score: payload.quality_score,
            processed_at: Utc::now(),
        };

        self.event_sender.send(processed_event).await.map_err(|_| {
            AppError::InternalError("Failed to send processed event".to_string())
        })?;

        Ok(())
    }

    /// Process shares purchase for market analytics
    async fn process_shares_purchased(
        &self,
        payload: &super::event_schema::SharesPurchasedPayload,
    ) -> Result<(), AppError> {
        // Market sentiment analysis
        let market_impact = self.market_analyzer
            .analyze_purchase_impact(payload.song_id, payload.purchase_price, payload.ownership_percentage)
            .await;

        // Update analytics
        {
            let mut store = self.analytics_store.write().await;
            store.update_market_metrics(payload.song_id, payload.purchase_price, market_impact.sentiment);
            store.add_investment_volume(payload.purchase_price);
        }

        // Generate market alerts
        if market_impact.volatility > self.config.market_volatility_threshold {
            self.send_market_alert(payload.song_id, market_impact).await?;
        }

        let processed_event = ProcessedEvent::MarketAnalytics {
            contract_id: payload.contract_id,
            song_id: payload.song_id,
            purchase_price: payload.purchase_price,
            market_sentiment: market_impact.sentiment,
            volatility: market_impact.volatility,
            processed_at: Utc::now(),
        };

        self.event_sender.send(processed_event).await.map_err(|_| {
            AppError::InternalError("Failed to send market analytics".to_string())
        })?;

        Ok(())
    }

    /// Process revenue distribution for financial analytics
    async fn process_revenue_distributed(
        &self,
        payload: &super::event_schema::RevenueDistributedPayload,
    ) -> Result<(), AppError> {
        {
            let mut store = self.analytics_store.write().await;
            store.add_distributed_revenue(payload.total_distributed);
            store.update_contract_performance(payload.contract_id, payload.total_distributed);
        }

        let processed_event = ProcessedEvent::RevenueAnalytics {
            contract_id: payload.contract_id,
            song_id: payload.song_id,
            total_distributed: payload.total_distributed,
            shareholder_count: payload.shareholder_count,
            distribution_efficiency: payload.total_distributed / payload.total_revenue,
            processed_at: Utc::now(),
        };

        self.event_sender.send(processed_event).await.map_err(|_| {
            AppError::InternalError("Failed to send revenue analytics".to_string())
        })?;

        Ok(())
    }

    async fn process_user_registered(
        &self,
        payload: &super::event_schema::UserRegisteredPayload,
    ) -> Result<(), AppError> {
        {
            let mut store = self.analytics_store.write().await;
            store.increment_user_registrations(&payload.user_type);
        }
        Ok(())
    }

    async fn process_song_uploaded(
        &self,
        payload: &super::event_schema::SongUploadedPayload,
    ) -> Result<(), AppError> {
        {
            let mut store = self.analytics_store.write().await;
            store.increment_songs_uploaded(&payload.genre);
        }
        Ok(())
    }

    async fn process_campaign_created(
        &self,
        payload: &super::event_schema::CampaignCreatedPayload,
    ) -> Result<(), AppError> {
        {
            let mut store = self.analytics_store.write().await;
            store.add_campaign_target_revenue(payload.target_revenue);
        }
        Ok(())
    }

    async fn process_system_health_check(
        &self,
        payload: &super::event_schema::SystemHealthCheckPayload,
    ) -> Result<(), AppError> {
        {
            let mut store = self.analytics_store.write().await;
            store.update_service_health(&payload.service, &payload.status, payload.response_time_ms);
        }
        Ok(())
    }

    async fn send_fraud_alert(&self, user_id: Uuid, session_id: Uuid, fraud_score: f64) -> Result<(), AppError> {
        warn!("🚨 FRAUD ALERT: User {} session {} has fraud score {:.2}", user_id, session_id, fraud_score);
        
        let alert = ProcessedEvent::FraudAlert {
            user_id,
            session_id,
            fraud_score,
            alert_type: "HIGH_FRAUD_SCORE".to_string(),
            detected_at: Utc::now(),
        };
        
        self.event_sender.send(alert).await.map_err(|_| {
            AppError::InternalError("Failed to send fraud alert".to_string())
        })?;
        
        Ok(())
    }

    async fn send_market_alert(&self, song_id: Uuid, market_impact: MarketImpact) -> Result<(), AppError> {
        warn!("📈 MARKET ALERT: Song {} has high volatility {:.2}", song_id, market_impact.volatility);
        
        let alert = ProcessedEvent::MarketAlert {
            song_id,
            volatility: market_impact.volatility,
            sentiment: market_impact.sentiment,
            alert_type: "HIGH_VOLATILITY".to_string(),
            detected_at: Utc::now(),
        };
        
        self.event_sender.send(alert).await.map_err(|_| {
            AppError::InternalError("Failed to send market alert".to_string())
        })?;
        
        Ok(())
    }

    async fn record_processing_metrics(&self, event_type: &str, processing_time: Duration) {
        debug!("⚡ Processed {} in {:?}", event_type, processing_time);
        
        // In production, send to Prometheus/metrics system
        {
            let mut store = self.analytics_store.write().await;
            store.update_processing_metrics(event_type, processing_time);
        }
    }

    /// Get current analytics dashboard data
    pub async fn get_real_time_analytics(&self) -> RealTimeAnalytics {
        let store = self.analytics_store.read().await;
        store.get_current_analytics()
    }

    /// Start background analytics aggregation
    pub async fn start_analytics_aggregation(&self) -> tokio::task::JoinHandle<()> {
        let analytics_store = Arc::clone(&self.analytics_store);
        let event_sender = self.event_sender.clone();
        let aggregation_interval = self.config.analytics_aggregation_interval;

        tokio::spawn(async move {
            let mut interval = interval(aggregation_interval);
            
            loop {
                interval.tick().await;
                
                let analytics = {
                    let store = analytics_store.read().await;
                    store.get_current_analytics()
                };
                
                let aggregated_event = ProcessedEvent::AggregatedAnalytics {
                    analytics: analytics.clone(),
                    aggregated_at: Utc::now(),
                };
                
                if let Err(e) = event_sender.send(aggregated_event).await {
                    error!("Failed to send aggregated analytics: {}", e);
                }
                
                info!("📊 Real-time analytics: {} daily listens, ${:.2} revenue", 
                      analytics.daily_listens, analytics.daily_revenue);
            }
        })
    }
}

/// Stream processor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamProcessorConfig {
    pub fraud_threshold: f64,
    pub market_volatility_threshold: f64,
    pub analytics_aggregation_interval: Duration,
    pub max_processing_latency_ms: u64,
    pub enable_real_time_alerts: bool,
}

impl Default for StreamProcessorConfig {
    fn default() -> Self {
        Self {
            fraud_threshold: 0.8,
            market_volatility_threshold: 0.3,
            analytics_aggregation_interval: Duration::from_secs(60),
            max_processing_latency_ms: 100,
            enable_real_time_alerts: true,
        }
    }
}

/// Fraud detection engine
#[derive(Debug)]
pub struct FraudDetector {
    user_behavior_cache: RwLock<HashMap<Uuid, UserBehaviorProfile>>,
}

impl FraudDetector {
    pub fn new() -> Self {
        Self {
            user_behavior_cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn analyze_listen_session(
        &self,
        user_id: Uuid,
        song_id: Uuid,
        duration_seconds: u32,
        zk_proof_hash: &str,
    ) -> Result<f64, AppError> {
        let mut fraud_score: f64 = 0.0;
        
        // Check for rapid successive listens (bot behavior)
        {
            let mut cache = self.user_behavior_cache.write().await;
            let profile = cache.entry(user_id).or_insert_with(|| UserBehaviorProfile::new(user_id));
            
            profile.add_listen(song_id, duration_seconds);
            
            // Analyze patterns
            if profile.listens_last_hour > 100 {
                fraud_score += 0.5; // High volume suspicious
            }
            
            if profile.average_listen_duration < 10.0 {
                fraud_score += 0.3; // Very short listens suspicious
            }
            
            if profile.unique_songs_ratio < 0.1 {
                fraud_score += 0.4; // Listening to same song repeatedly
            }
        }
        
        // ZK proof validation (simplified)
        if zk_proof_hash.len() < 32 {
            fraud_score += 0.6; // Invalid proof format
        }
        
        // Duration analysis
        if duration_seconds < 10 || duration_seconds > 600 {
            fraud_score += 0.2; // Suspicious duration
        }
        
        Ok(fraud_score.min(1.0))
    }
}

/// Revenue calculation engine
#[derive(Debug)]
pub struct RevenueCalculator;

impl RevenueCalculator {
    pub fn new() -> Self {
        Self
    }

    pub async fn calculate_listen_revenue(&self, quality_score: f64, duration_seconds: u32) -> f64 {
        // Base revenue per listen
        let base_revenue = 0.001; // $0.001 per listen
        
        // Quality multiplier
        let quality_multiplier = quality_score;
        
        // Duration multiplier (optimal at 3-4 minutes)
        let duration_multiplier = if duration_seconds >= 30 {
            let minutes = duration_seconds as f64 / 60.0;
            (minutes / 3.5).min(1.0)
        } else {
            0.1 // Very short listens get minimal revenue
        };
        
        base_revenue * quality_multiplier * duration_multiplier
    }
}

/// Market analysis engine
#[derive(Debug)]
pub struct MarketAnalyzer {
    price_history: RwLock<HashMap<Uuid, Vec<PricePoint>>>,
}

impl MarketAnalyzer {
    pub fn new() -> Self {
        Self {
            price_history: RwLock::new(HashMap::new()),
        }
    }

    pub async fn analyze_purchase_impact(
        &self,
        song_id: Uuid,
        purchase_price: f64,
        ownership_percentage: f64,
    ) -> MarketImpact {
        let mut history = self.price_history.write().await;
        let prices = history.entry(song_id).or_insert_with(Vec::new);
        
        prices.push(PricePoint {
            price: purchase_price,
            ownership_percentage,
            timestamp: Utc::now(),
        });
        
        // Keep only last 100 data points
        if prices.len() > 100 {
            prices.remove(0);
        }
        
        // Calculate volatility and sentiment
        let volatility = if prices.len() > 1 {
            let price_changes: Vec<f64> = prices.windows(2)
                .map(|window| (window[1].price - window[0].price) / window[0].price)
                .collect();
            
            let variance = price_changes.iter()
                .map(|&change| change * change)
                .sum::<f64>() / price_changes.len() as f64;
            
            variance.sqrt()
        } else {
            0.0
        };
        
        let sentiment = if prices.len() >= 2 {
            let recent_trend = prices[prices.len() - 1].price - prices[prices.len() - 2].price;
            if recent_trend > 0.0 { 0.7 } else { 0.3 }
        } else {
            0.5
        };
        
        MarketImpact {
            volatility,
            sentiment,
            price_trend: if sentiment > 0.5 { "bullish".to_string() } else { "bearish".to_string() },
        }
    }
}

/// Analytics data store
#[derive(Debug)]
pub struct AnalyticsStore {
    // Real-time counters
    pub daily_listens: u64,
    pub daily_revenue: f64,
    pub daily_investments: f64,
    pub daily_distributed_revenue: f64,
    
    // User metrics
    pub user_registrations: HashMap<String, u64>, // user_type -> count
    pub active_users_last_hour: u64,
    
    // Music metrics
    pub songs_uploaded: HashMap<String, u64>, // genre -> count
    pub top_songs: Vec<(Uuid, u64)>, // song_id, listen_count
    
    // Market metrics
    pub total_market_cap: f64,
    pub average_song_price: f64,
    
    // System metrics
    pub service_health: HashMap<String, ServiceHealth>,
    pub processing_metrics: HashMap<String, ProcessingMetrics>,
    
    // Performance metrics
    pub contracts_performance: HashMap<Uuid, ContractPerformance>,
}

impl AnalyticsStore {
    pub fn new() -> Self {
        Self {
            daily_listens: 0,
            daily_revenue: 0.0,
            daily_investments: 0.0,
            daily_distributed_revenue: 0.0,
            user_registrations: HashMap::new(),
            active_users_last_hour: 0,
            songs_uploaded: HashMap::new(),
            top_songs: Vec::new(),
            total_market_cap: 0.0,
            average_song_price: 0.0,
            service_health: HashMap::new(),
            processing_metrics: HashMap::new(),
            contracts_performance: HashMap::new(),
        }
    }

    pub fn increment_daily_listens(&mut self) {
        self.daily_listens += 1;
    }

    pub fn add_daily_revenue(&mut self, amount: f64) {
        self.daily_revenue += amount;
    }

    pub fn add_investment_volume(&mut self, amount: f64) {
        self.daily_investments += amount;
    }

    pub fn add_distributed_revenue(&mut self, amount: f64) {
        self.daily_distributed_revenue += amount;
    }

    pub fn increment_user_registrations(&mut self, user_type: &str) {
        *self.user_registrations.entry(user_type.to_string()).or_insert(0) += 1;
    }

    pub fn increment_songs_uploaded(&mut self, genre: &str) {
        *self.songs_uploaded.entry(genre.to_string()).or_insert(0) += 1;
    }

    pub fn add_campaign_target_revenue(&mut self, amount: f64) {
        // Update total potential market cap
        self.total_market_cap += amount;
    }

    pub fn update_user_engagement(&mut self, _user_id: Uuid, _duration: u32) {
        self.active_users_last_hour += 1;
    }

    pub fn update_song_metrics(&mut self, song_id: Uuid, revenue: f64, _quality: f64) {
        // Update top songs list
        if let Some(pos) = self.top_songs.iter().position(|(id, _)| *id == song_id) {
            self.top_songs[pos].1 += 1;
        } else {
            self.top_songs.push((song_id, 1));
        }
        
        // Sort and keep top 10
        self.top_songs.sort_by(|a, b| b.1.cmp(&a.1));
        self.top_songs.truncate(10);
    }

    pub fn update_market_metrics(&mut self, _song_id: Uuid, price: f64, _sentiment: f64) {
        // Update average price (simplified)
        self.average_song_price = (self.average_song_price + price) / 2.0;
    }

    pub fn update_contract_performance(&mut self, contract_id: Uuid, distributed_amount: f64) {
        let performance = self.contracts_performance.entry(contract_id).or_insert_with(|| ContractPerformance {
            total_distributed: 0.0,
            total_listens: 0,
            efficiency_score: 0.0,
        });
        
        performance.total_distributed += distributed_amount;
        performance.efficiency_score = performance.total_distributed / (performance.total_listens as f64 + 1.0);
    }

    pub fn update_service_health(&mut self, service: &str, status: &str, response_time: u64) {
        self.service_health.insert(service.to_string(), ServiceHealth {
            status: status.to_string(),
            last_check: Utc::now(),
            response_time_ms: response_time,
        });
    }

    pub fn update_processing_metrics(&mut self, event_type: &str, duration: Duration) {
        let metrics = self.processing_metrics.entry(event_type.to_string()).or_insert_with(|| ProcessingMetrics {
            total_processed: 0,
            average_latency_ms: 0.0,
            errors: 0,
        });
        
        metrics.total_processed += 1;
        metrics.average_latency_ms = (metrics.average_latency_ms + duration.as_millis() as f64) / 2.0;
    }

    pub fn get_current_analytics(&self) -> RealTimeAnalytics {
        RealTimeAnalytics {
            daily_listens: self.daily_listens,
            daily_revenue: self.daily_revenue,
            daily_investments: self.daily_investments,
            daily_distributed_revenue: self.daily_distributed_revenue,
            active_users: self.active_users_last_hour,
            total_market_cap: self.total_market_cap,
            average_song_price: self.average_song_price,
            top_songs: self.top_songs.clone(),
            service_health: self.service_health.clone(),
            timestamp: Utc::now(),
        }
    }
}

// Supporting types
#[derive(Debug, Clone)]
pub struct UserBehaviorProfile {
    pub user_id: Uuid,
    pub listens_last_hour: u32,
    pub total_listens: u64,
    pub unique_songs: std::collections::HashSet<Uuid>,
    pub average_listen_duration: f64,
    pub unique_songs_ratio: f64,
    pub last_listen: DateTime<Utc>,
}

impl UserBehaviorProfile {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            listens_last_hour: 0,
            total_listens: 0,
            unique_songs: std::collections::HashSet::new(),
            average_listen_duration: 0.0,
            unique_songs_ratio: 1.0,
            last_listen: Utc::now(),
        }
    }

    pub fn add_listen(&mut self, song_id: Uuid, duration: u32) {
        self.total_listens += 1;
        self.unique_songs.insert(song_id);
        self.average_listen_duration = (self.average_listen_duration + duration as f64) / 2.0;
        self.unique_songs_ratio = self.unique_songs.len() as f64 / self.total_listens as f64;
        
        // Reset hourly counter if more than an hour has passed
        let now = Utc::now();
        if (now - self.last_listen).num_minutes() > 60 {
            self.listens_last_hour = 0;
        }
        
        self.listens_last_hour += 1;
        self.last_listen = now;
    }
}

#[derive(Debug, Clone)]
pub struct MarketImpact {
    pub volatility: f64,
    pub sentiment: f64,
    pub price_trend: String,
}

#[derive(Debug, Clone)]
pub struct PricePoint {
    pub price: f64,
    pub ownership_percentage: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeAnalytics {
    pub daily_listens: u64,
    pub daily_revenue: f64,
    pub daily_investments: f64,
    pub daily_distributed_revenue: f64,
    pub active_users: u64,
    pub total_market_cap: f64,
    pub average_song_price: f64,
    pub top_songs: Vec<(Uuid, u64)>,
    pub service_health: HashMap<String, ServiceHealth>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: String,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessingMetrics {
    pub total_processed: u64,
    pub average_latency_ms: f64,
    pub errors: u64,
}

#[derive(Debug, Clone)]
pub struct ContractPerformance {
    pub total_distributed: f64,
    pub total_listens: u64,
    pub efficiency_score: f64,
}

/// Processed events for downstream consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ProcessedEvent {
    ListenAnalytics {
        session_id: Uuid,
        user_id: Uuid,
        song_id: Uuid,
        estimated_revenue: f64,
        fraud_score: f64,
        quality_score: f64,
        processed_at: DateTime<Utc>,
    },
    MarketAnalytics {
        contract_id: Uuid,
        song_id: Uuid,
        purchase_price: f64,
        market_sentiment: f64,
        volatility: f64,
        processed_at: DateTime<Utc>,
    },
    RevenueAnalytics {
        contract_id: Uuid,
        song_id: Uuid,
        total_distributed: f64,
        shareholder_count: u32,
        distribution_efficiency: f64,
        processed_at: DateTime<Utc>,
    },
    FraudAlert {
        user_id: Uuid,
        session_id: Uuid,
        fraud_score: f64,
        alert_type: String,
        detected_at: DateTime<Utc>,
    },
    MarketAlert {
        song_id: Uuid,
        volatility: f64,
        sentiment: f64,
        alert_type: String,
        detected_at: DateTime<Utc>,
    },
    AggregatedAnalytics {
        analytics: RealTimeAnalytics,
        aggregated_at: DateTime<Utc>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fraud_detector() {
        let detector = FraudDetector::new();
        let user_id = Uuid::new_v4();
        let song_id = Uuid::new_v4();
        
        let fraud_score = detector.analyze_listen_session(
            user_id,
            song_id,
            30,
            "valid_zk_proof_hash_example_32chars",
        ).await.unwrap();
        
        assert!(fraud_score >= 0.0 && fraud_score <= 1.0);
    }

    #[tokio::test]
    async fn test_revenue_calculator() {
        let calculator = RevenueCalculator::new();
        
        let revenue = calculator.calculate_listen_revenue(0.95, 180).await;
        assert!(revenue > 0.0);
        
        // Short listen should get less revenue
        let short_revenue = calculator.calculate_listen_revenue(0.95, 10).await;
        assert!(short_revenue < revenue);
    }

    #[test]
    fn test_analytics_store() {
        let mut store = AnalyticsStore::new();
        
        store.increment_daily_listens();
        store.add_daily_revenue(0.001);
        
        assert_eq!(store.daily_listens, 1);
        assert!((store.daily_revenue - 0.001).abs() < f64::EPSILON);
    }
} 