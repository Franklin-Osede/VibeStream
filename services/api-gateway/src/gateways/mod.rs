// =============================================================================
// VIBESTREAM API GATEWAYS - ARQUITECTURA INDEPENDIENTE
// =============================================================================
// 
// Este módulo implementa gateways independientes para cada bounded context,
// permitiendo escalabilidad y desarrollo paralelo.

pub mod user_gateway;
pub mod music_gateway;
pub mod payment_gateway;
pub mod campaign_gateway;
pub mod listen_reward_gateway;
pub mod fan_ventures_gateway;
pub mod notification_gateway;

// Re-export para facilitar el uso
pub use user_gateway::create_user_gateway;
pub use music_gateway::create_music_gateway;
pub use payment_gateway::create_payment_gateway;
pub use campaign_gateway::create_campaign_gateway;
pub use listen_reward_gateway::create_listen_reward_gateway;
pub use fan_ventures_gateway::create_fan_ventures_gateway;
pub use notification_gateway::create_notification_gateway;

// =============================================================================
// GATEWAY FACTORY
// =============================================================================

use axum::Router;
use std::sync::Arc;
use crate::shared::infrastructure::app_state::AppState;

/// Factory para crear todos los gateways con configuración consistente
pub struct GatewayFactory;

impl GatewayFactory {
    /// Crear todos los gateways independientes
    pub async fn create_all_gateways(app_state: AppState) -> Result<Vec<(String, Router)>, Box<dyn std::error::Error>> {
        let gateways = vec![
            ("user".to_string(), create_user_gateway(app_state.clone()).await?),
            ("music".to_string(), create_music_gateway(app_state.clone()).await?),
            ("payment".to_string(), create_payment_gateway(app_state.clone()).await?),
            ("campaign".to_string(), create_campaign_gateway(app_state.clone()).await?),
            ("listen_reward".to_string(), create_listen_reward_gateway(app_state.clone()).await?),
            ("fan_ventures".to_string(), create_fan_ventures_gateway(app_state.clone()).await?),
            ("notification".to_string(), create_notification_gateway(app_state.clone()).await?),
        ];
        
        Ok(gateways)
    }
    
    /// Crear gateway específico por nombre
    pub async fn create_gateway_by_name(
        name: &str, 
        app_state: AppState
    ) -> Result<Router, Box<dyn std::error::Error>> {
        match name {
            "user" => create_user_gateway(app_state).await,
            "music" => create_music_gateway(app_state).await,
            "payment" => create_payment_gateway(app_state).await,
            "campaign" => create_campaign_gateway(app_state).await,
            "listen_reward" => create_listen_reward_gateway(app_state).await,
            "fan_ventures" => create_fan_ventures_gateway(app_state).await,
            "notification" => create_notification_gateway(app_state).await,
            _ => Err(format!("Gateway '{}' no encontrado", name).into()),
        }
    }
}

// =============================================================================
// GATEWAY CONFIGURATION
// =============================================================================

/// Configuración para cada gateway independiente
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub name: String,
    pub port: u16,
    pub host: String,
    pub cors_enabled: bool,
    pub rate_limiting_enabled: bool,
    pub health_check_enabled: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            port: 3000,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: false,
            health_check_enabled: true,
        }
    }
}

impl GatewayConfig {
    /// Crear configuración específica para cada gateway
    pub fn user_gateway() -> Self {
        Self {
            name: "user".to_string(),
            port: 3001,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: true,
            health_check_enabled: true,
        }
    }
    
    pub fn music_gateway() -> Self {
        Self {
            name: "music".to_string(),
            port: 3002,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: false,
            health_check_enabled: true,
        }
    }
    
    pub fn payment_gateway() -> Self {
        Self {
            name: "payment".to_string(),
            port: 3003,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: true,
            health_check_enabled: true,
        }
    }
    
    pub fn campaign_gateway() -> Self {
        Self {
            name: "campaign".to_string(),
            port: 3004,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: false,
            health_check_enabled: true,
        }
    }
    
    pub fn listen_reward_gateway() -> Self {
        Self {
            name: "listen_reward".to_string(),
            port: 3005,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: false,
            health_check_enabled: true,
        }
    }
    
    pub fn fan_ventures_gateway() -> Self {
        Self {
            name: "fan_ventures".to_string(),
            port: 3006,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: false,
            health_check_enabled: true,
        }
    }
    
    pub fn notification_gateway() -> Self {
        Self {
            name: "notification".to_string(),
            port: 3007,
            host: "127.0.0.1".to_string(),
            cors_enabled: true,
            rate_limiting_enabled: false,
            health_check_enabled: true,
        }
    }
}
