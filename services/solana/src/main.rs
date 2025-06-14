use solana_service::run_solana_worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Solana Service Worker...");
    
    if let Err(e) = run_solana_worker().await {
        eprintln!("❌ Solana worker error: {}", e);
        return Err(e.into());
    }
    
    Ok(())
} 