use anyhow::Result;
use tracing::info;
use tracing_subscriber;
use zk_service::{ZkService, ZkServiceConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("zk_service=debug,info")
        .init();

    info!("🚀 Starting ZK Service...");

    // Load configuration from environment or use defaults
    let config = ZkServiceConfig {
        circuits_dir: env::var("CIRCUITS_DIR")
            .unwrap_or_else(|_| "../../backend/circuits".to_string()),
        cache_dir: env::var("ZK_CACHE_DIR")
            .unwrap_or_else(|_| "/tmp/zk_cache".to_string()),
        redis_url: env::var("REDIS_URL").ok(),
        server_port: env::var("ZK_SERVICE_PORT")
            .unwrap_or_else(|_| "8003".to_string())
            .parse()
            .unwrap_or(8003),
    };

    info!("📁 Circuits directory: {}", config.circuits_dir);
    info!("💾 Cache directory: {}", config.cache_dir);
    info!("🔴 Redis URL: {:?}", config.redis_url);
    info!("🌐 Server port: {}", config.server_port);

    // Create and start the ZK service
    let zk_service = ZkService::new(config).await?;
    
    info!("✅ ZK Service initialized successfully");
    info!("🎯 Service is ready to process ZK proof requests");

    // Run the worker (this will start the HTTP server)
    zk_service.run_worker().await?;
    
    Ok(())
} 