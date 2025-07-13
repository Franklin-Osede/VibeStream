use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use super::super::domain::value_objects::*;

/// Quality Adaptation Engine - Adapts video quality based on network conditions
pub struct QualityAdaptationEngine {
    session_qualities: Arc<RwLock<HashMap<String, SessionQualityState>>>,
    adaptation_config: QualityAdaptationConfig,
    quality_metrics: Arc<RwLock<HashMap<String, QualityMetrics>>>,
}

impl QualityAdaptationEngine {
    pub fn new() -> Self {
        println!("🎯 Initializing Quality Adaptation Engine");
        
        Self {
            session_qualities: Arc::new(RwLock::new(HashMap::new())),
            adaptation_config: QualityAdaptationConfig::default(),
            quality_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Update quality for a session
    pub async fn update_quality(&self, session_id: &str, new_quality: VideoQuality) {
        println!("🔄 Updating quality to {:?} for session {}", new_quality, session_id);
        
        let quality_state = SessionQualityState {
            current_quality: new_quality,
            last_change: chrono::Utc::now(),
            change_count: 0,
        };

        {
            let mut qualities = self.session_qualities.write().await;
            if let Some(existing) = qualities.get_mut(session_id) {
                existing.current_quality = new_quality;
                existing.last_change = chrono::Utc::now();
                existing.change_count += 1;
            } else {
                qualities.insert(session_id.to_string(), quality_state);
            }
        }
    }

    /// Adapt quality based on buffer level and network conditions
    pub async fn adapt_quality(&self, session_id: &str, buffer_level: f64) -> Option<VideoQuality> {
        let current_quality = self.get_current_quality(session_id).await;
        let metrics = self.get_quality_metrics(session_id).await;
        
        let adapted_quality = self.calculate_optimal_quality(
            current_quality,
            buffer_level,
            &metrics,
        ).await;

        if adapted_quality != current_quality {
            println!("🎯 Quality adaptation: {:?} -> {:?} for session {} (buffer: {:.2})", 
                     current_quality, adapted_quality, session_id, buffer_level);
            
            self.update_quality(session_id, adapted_quality).await;
            Some(adapted_quality)
        } else {
            None
        }
    }

    /// Calculate optimal quality based on conditions
    async fn calculate_optimal_quality(
        &self,
        current_quality: VideoQuality,
        buffer_level: f64,
        metrics: &QualityMetrics,
    ) -> VideoQuality {
        // Buffer-based adaptation
        if buffer_level < self.adaptation_config.min_buffer_threshold {
            return self.downgrade_quality(current_quality);
        }

        if buffer_level > self.adaptation_config.max_buffer_threshold {
            return self.upgrade_quality(current_quality);
        }

        // Bandwidth-based adaptation
        if metrics.average_bandwidth_mbps < self.get_quality_bandwidth_requirement(current_quality) {
            return self.downgrade_quality(current_quality);
        }

        if metrics.average_bandwidth_mbps > self.get_quality_bandwidth_requirement(self.upgrade_quality(current_quality)) * 1.5 {
            return self.upgrade_quality(current_quality);
        }

        // Latency-based adaptation
        if metrics.average_latency_ms > self.adaptation_config.max_latency_ms {
            return self.downgrade_quality(current_quality);
        }

        current_quality
    }

    /// Downgrade quality
    fn downgrade_quality(&self, current_quality: VideoQuality) -> VideoQuality {
        match current_quality {
            VideoQuality::UltraHD => VideoQuality::High,
            VideoQuality::High => VideoQuality::Medium,
            VideoQuality::Medium => VideoQuality::Low,
            VideoQuality::Low => VideoQuality::Low, // Can't go lower
            VideoQuality::Auto => VideoQuality::Medium,
        }
    }

    /// Upgrade quality
    fn upgrade_quality(&self, current_quality: VideoQuality) -> VideoQuality {
        match current_quality {
            VideoQuality::Low => VideoQuality::Medium,
            VideoQuality::Medium => VideoQuality::High,
            VideoQuality::High => VideoQuality::UltraHD,
            VideoQuality::UltraHD => VideoQuality::UltraHD, // Can't go higher
            VideoQuality::Auto => VideoQuality::High,
        }
    }

    /// Get bandwidth requirement for quality
    fn get_quality_bandwidth_requirement(&self, quality: VideoQuality) -> f64 {
        match quality {
            VideoQuality::Low => 1.0,      // 1 Mbps
            VideoQuality::Medium => 2.5,   // 2.5 Mbps
            VideoQuality::High => 5.0,     // 5 Mbps
            VideoQuality::UltraHD => 15.0, // 15 Mbps
            VideoQuality::Auto => 2.5,     // Default to medium
        }
    }

    /// Get current quality for session
    async fn get_current_quality(&self, session_id: &str) -> VideoQuality {
        let qualities = self.session_qualities.read().await;
        qualities.get(session_id)
            .map(|state| state.current_quality.clone())
            .unwrap_or(VideoQuality::Medium)
    }

    /// Get quality metrics for session
    async fn get_quality_metrics(&self, session_id: &str) -> QualityMetrics {
        let metrics = self.quality_metrics.read().await;
        metrics.get(session_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Update quality metrics
    pub async fn update_metrics(&self, session_id: &str, metrics: QualityMetrics) {
        let mut quality_metrics = self.quality_metrics.write().await;
        quality_metrics.insert(session_id.to_string(), metrics);
    }

    /// Get available qualities for session
    pub async fn get_available_qualities(&self, session_id: &str) -> Vec<VideoQuality> {
        let current_quality = self.get_current_quality(session_id).await;
        let metrics = self.get_quality_metrics(session_id).await;
        
        let mut available = vec![VideoQuality::Low, VideoQuality::Medium];
        
        if metrics.average_bandwidth_mbps >= 5.0 {
            available.push(VideoQuality::High);
        }
        
        if metrics.average_bandwidth_mbps >= 15.0 {
            available.push(VideoQuality::UltraHD);
        }
        
        available
    }

    /// Get adaptation statistics
    pub async fn get_adaptation_stats(&self, session_id: &str) -> Option<AdaptationStats> {
        let qualities = self.session_qualities.read().await;
        qualities.get(session_id).map(|state| AdaptationStats {
            current_quality: state.current_quality.clone(),
            change_count: state.change_count,
            last_change: state.last_change,
        })
    }

    /// Reset quality for session
    pub async fn reset_quality(&self, session_id: &str) {
        let mut qualities = self.session_qualities.write().await;
        qualities.remove(session_id);
        
        let mut metrics = self.quality_metrics.write().await;
        metrics.remove(session_id);
    }

    /// Get adaptation configuration
    pub fn get_config(&self) -> &QualityAdaptationConfig {
        &self.adaptation_config
    }

    /// Update adaptation configuration
    pub fn update_config(&mut self, config: QualityAdaptationConfig) {
        self.adaptation_config = config;
    }
}

/// Session Quality State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionQualityState {
    pub current_quality: VideoQuality,
    pub last_change: chrono::DateTime<chrono::Utc>,
    pub change_count: u32,
}

/// Quality Metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub average_bandwidth_mbps: f64,
    pub average_latency_ms: u32,
    pub packet_loss_percent: f64,
    pub buffer_level: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Adaptation Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationStats {
    pub current_quality: VideoQuality,
    pub change_count: u32,
    pub last_change: chrono::DateTime<chrono::Utc>,
}

/// Quality Adaptation Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAdaptationConfig {
    pub min_buffer_threshold: f64,
    pub max_buffer_threshold: f64,
    pub max_latency_ms: u32,
    pub adaptation_interval_ms: u64,
    pub enable_bandwidth_adaptation: bool,
    pub enable_latency_adaptation: bool,
    pub enable_buffer_adaptation: bool,
}

impl Default for QualityAdaptationConfig {
    fn default() -> Self {
        Self {
            min_buffer_threshold: 0.2,  // 20% buffer minimum
            max_buffer_threshold: 0.8,  // 80% buffer maximum
            max_latency_ms: 200,        // 200ms max latency
            adaptation_interval_ms: 5000, // 5 seconds
            enable_bandwidth_adaptation: true,
            enable_latency_adaptation: true,
            enable_buffer_adaptation: true,
        }
    }
} 