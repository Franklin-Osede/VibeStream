use anyhow::Result;
use tracing::info;
use tracing_subscriber;
use zk_service::ZkService;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("zk_service=debug,info")
        .init();

    info!("Starting ZK Service...");

    // Create and start the ZK service
    let zk_service = ZkService::new();
    
    info!("ZK Service started successfully on port 8003");
    info!("Service is ready to process ZK proof requests");

    // Run the worker
    zk_service.run_worker().await?;
    
    Ok(())
} 