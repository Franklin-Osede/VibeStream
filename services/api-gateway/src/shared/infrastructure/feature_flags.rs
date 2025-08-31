//! Sistema de Feature Flags para controlar módulos habilitados
//! 
//! Permite habilitar/deshabilitar módulos sin romper la compilación
//! y facilita el desarrollo incremental.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuración de feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub listen_reward_enabled: bool,
    pub fan_ventures_enabled: bool,
    pub notifications_enabled: bool,
    pub music_enabled: bool,
    pub analytics_enabled: bool,
    pub search_enabled: bool,
    pub market_stats_enabled: bool,
    pub zk_integration_enabled: bool,
    pub blockchain_integration_enabled: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            // Módulos estables habilitados por defecto
            listen_reward_enabled: true,
            fan_ventures_enabled: true,
            notifications_enabled: false, // Deshabilitado por ahora
            music_enabled: true,
            
            // Funcionalidades avanzadas deshabilitadas por defecto
            analytics_enabled: false,
            search_enabled: false,
            market_stats_enabled: false,
            zk_integration_enabled: false,
            blockchain_integration_enabled: false,
        }
    }
}

impl FeatureFlags {
    /// Crear desde variables de entorno
    pub fn from_env() -> Self {
        Self {
            listen_reward_enabled: std::env::var("FEATURE_LISTEN_REWARD")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            
            fan_ventures_enabled: std::env::var("FEATURE_FAN_VENTURES")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            
            notifications_enabled: std::env::var("FEATURE_NOTIFICATIONS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            
            music_enabled: std::env::var("FEATURE_MUSIC")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            
            analytics_enabled: std::env::var("FEATURE_ANALYTICS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            
            search_enabled: std::env::var("FEATURE_SEARCH")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            
            market_stats_enabled: std::env::var("FEATURE_MARKET_STATS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            
            zk_integration_enabled: std::env::var("FEATURE_ZK_INTEGRATION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            
            blockchain_integration_enabled: std::env::var("FEATURE_BLOCKCHAIN_INTEGRATION")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
    
    /// Verificar si un módulo está habilitado
    pub fn is_enabled(&self, module: &str) -> bool {
        match module {
            "listen_reward" => self.listen_reward_enabled,
            "fan_ventures" => self.fan_ventures_enabled,
            "notifications" => self.notifications_enabled,
            "music" => self.music_enabled,
            "analytics" => self.analytics_enabled,
            "search" => self.search_enabled,
            "market_stats" => self.market_stats_enabled,
            "zk_integration" => self.zk_integration_enabled,
            "blockchain_integration" => self.blockchain_integration_enabled,
            _ => false,
        }
    }
    
    /// Obtener lista de módulos habilitados
    pub fn enabled_modules(&self) -> Vec<String> {
        let mut modules = Vec::new();
        
        if self.listen_reward_enabled { modules.push("listen_reward".to_string()); }
        if self.fan_ventures_enabled { modules.push("fan_ventures".to_string()); }
        if self.notifications_enabled { modules.push("notifications".to_string()); }
        if self.music_enabled { modules.push("music".to_string()); }
        if self.analytics_enabled { modules.push("analytics".to_string()); }
        if self.search_enabled { modules.push("search".to_string()); }
        if self.market_stats_enabled { modules.push("market_stats".to_string()); }
        if self.zk_integration_enabled { modules.push("zk_integration".to_string()); }
        if self.blockchain_integration_enabled { modules.push("blockchain_integration".to_string()); }
        
        modules
    }
    
    /// Obtener lista de módulos deshabilitados
    pub fn disabled_modules(&self) -> Vec<String> {
        let mut modules = Vec::new();
        
        if !self.listen_reward_enabled { modules.push("listen_reward".to_string()); }
        if !self.fan_ventures_enabled { modules.push("fan_ventures".to_string()); }
        if !self.notifications_enabled { modules.push("notifications".to_string()); }
        if !self.music_enabled { modules.push("music".to_string()); }
        if !self.analytics_enabled { modules.push("analytics".to_string()); }
        if !self.search_enabled { modules.push("search".to_string()); }
        if !self.market_stats_enabled { modules.push("market_stats".to_string()); }
        if !self.zk_integration_enabled { modules.push("zk_integration".to_string()); }
        if !self.blockchain_integration_enabled { modules.push("blockchain_integration".to_string()); }
        
        modules
    }
}

/// Manager de feature flags con soporte para cambios dinámicos
#[derive(Clone)]
pub struct FeatureFlagManager {
    flags: Arc<RwLock<FeatureFlags>>,
}

impl FeatureFlagManager {
    pub fn new(flags: FeatureFlags) -> Self {
        Self {
            flags: Arc::new(RwLock::new(flags)),
        }
    }
    
    pub fn default() -> Self {
        Self::new(FeatureFlags::default())
    }
    
    pub fn from_env() -> Self {
        Self::new(FeatureFlags::from_env())
    }
    
    /// Verificar si un módulo está habilitado
    pub async fn is_enabled(&self, module: &str) -> bool {
        let flags = self.flags.read().await;
        flags.is_enabled(module)
    }
    
    /// Actualizar feature flags dinámicamente
    pub async fn update_flags(&self, new_flags: FeatureFlags) {
        let mut flags = self.flags.write().await;
        *flags = new_flags;
    }
    
    /// Obtener configuración actual
    pub async fn get_flags(&self) -> FeatureFlags {
        let flags = self.flags.read().await;
        flags.clone()
    }
    
    /// Habilitar un módulo específico
    pub async fn enable_module(&self, module: &str) {
        let mut flags = self.flags.write().await;
        match module {
            "listen_reward" => flags.listen_reward_enabled = true,
            "fan_ventures" => flags.fan_ventures_enabled = true,
            "notifications" => flags.notifications_enabled = true,
            "music" => flags.music_enabled = true,
            "analytics" => flags.analytics_enabled = true,
            "search" => flags.search_enabled = true,
            "market_stats" => flags.market_stats_enabled = true,
            "zk_integration" => flags.zk_integration_enabled = true,
            "blockchain_integration" => flags.blockchain_integration_enabled = true,
            _ => {}
        }
    }
    
    /// Deshabilitar un módulo específico
    pub async fn disable_module(&self, module: &str) {
        let mut flags = self.flags.write().await;
        match module {
            "listen_reward" => flags.listen_reward_enabled = false,
            "fan_ventures" => flags.fan_ventures_enabled = false,
            "notifications" => flags.notifications_enabled = false,
            "music" => flags.music_enabled = false,
            "analytics" => flags.analytics_enabled = false,
            "search" => flags.search_enabled = false,
            "market_stats" => flags.market_stats_enabled = false,
            "zk_integration" => flags.zk_integration_enabled = false,
            "blockchain_integration" => flags.blockchain_integration_enabled = false,
            _ => {}
        }
    }
}

/// Macro para verificar feature flags en tiempo de compilación
#[macro_export]
macro_rules! feature_enabled {
    ($feature:expr) => {
        cfg!(feature = $feature) || std::env::var(concat!("FEATURE_", stringify!($feature)))
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false)
    };
}

/// Macro para ejecutar código condicionalmente basado en feature flags
#[macro_export]
macro_rules! if_feature {
    ($feature:expr, $code:block) => {
        if feature_enabled!($feature) {
            $code
        }
    };
}

// Tests unitarios
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature_flags_default() {
        let flags = FeatureFlags::default();
        assert!(flags.listen_reward_enabled);
        assert!(flags.fan_ventures_enabled);
        assert!(!flags.notifications_enabled);
        assert!(flags.music_enabled);
    }
    
    #[test]
    fn test_feature_flags_is_enabled() {
        let flags = FeatureFlags::default();
        assert!(flags.is_enabled("listen_reward"));
        assert!(flags.is_enabled("fan_ventures"));
        assert!(!flags.is_enabled("notifications"));
        assert!(!flags.is_enabled("unknown_module"));
    }
    
    #[test]
    fn test_feature_flags_enabled_modules() {
        let flags = FeatureFlags::default();
        let enabled = flags.enabled_modules();
        assert!(enabled.contains(&"listen_reward".to_string()));
        assert!(enabled.contains(&"fan_ventures".to_string()));
        assert!(enabled.contains(&"music".to_string()));
        assert!(!enabled.contains(&"notifications".to_string()));
    }
    
    #[tokio::test]
    async fn test_feature_flag_manager() {
        let manager = FeatureFlagManager::default();
        
        assert!(manager.is_enabled("listen_reward").await);
        assert!(manager.is_enabled("fan_ventures").await);
        assert!(!manager.is_enabled("notifications").await);
        
        // Habilitar notifications
        manager.enable_module("notifications").await;
        assert!(manager.is_enabled("notifications").await);
        
        // Deshabilitar listen_reward
        manager.disable_module("listen_reward").await;
        assert!(!manager.is_enabled("listen_reward").await);
    }
}









