use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use std::time::Duration;
use std::thread;
use anyhow::{Result, anyhow};
use log::{info, warn, error};

const MAX_RETRIES: u32 = 12;
const RETRY_DELAY: u64 = 10;
const MIN_BALANCE_FOR_TEST: u64 = 100_000; // 0.0001 SOL
const TRANSFER_AMOUNT: u64 = 50_000; // 0.00005 SOL

async fn wait_for_balance(client: &RpcClient, address: &str, min_balance: u64) -> Result<u64> {
    info!("Esperando fondos para la dirección: {}", address);
    
    for attempt in 1..=MAX_RETRIES {
        match client.get_balance(&Keypair::from_base58_string(address).pubkey()) {
            Ok(balance) => {
                if balance >= min_balance {
                    info!("✅ Fondos recibidos! Balance actual: {} SOL", 
                        balance as f64 / 1_000_000_000.0);
                    return Ok(balance);
                }
                warn!("⏳ Intento {}/{} - Balance actual: {} SOL", 
                    attempt, MAX_RETRIES, balance as f64 / 1_000_000_000.0);
            },
            Err(e) => {
                error!("❌ Error al verificar balance (intento {}/{}): {}", 
                    attempt, MAX_RETRIES, e);
            }
        }
        
        if attempt < MAX_RETRIES {
            thread::sleep(Duration::from_secs(RETRY_DELAY));
        }
    }
    
    Err(anyhow!("No se recibieron suficientes fondos después de {} intentos", MAX_RETRIES))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar cliente RPC
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed()
    );

    // Crear wallet para pruebas
    println!("🔑 Creando nueva wallet para recibir fondos...");
    let my_wallet = Keypair::new();
    println!("📫 Dirección de mi wallet: {}", my_wallet.pubkey());

    // Verificar balance inicial
    match client.get_balance(&my_wallet.pubkey()) {
        Ok(balance) => println!("\n💰 Balance inicial: {} SOL", 
            balance as f64 / 1_000_000_000.0),
        Err(e) => error!("❌ Error al verificar balance inicial: {}", e)
    }

    // Instrucciones para el usuario
    println!("\n🔄 Para probar una transferencia, necesitamos que envíes fondos a esta wallet.");
    println!("Por favor:");
    println!("1. Ve a https://solfaucet.com");
    println!("2. Pega esta dirección: {}", my_wallet.pubkey());
    println!("3. Selecciona 'Devnet'");
    println!("4. Click en 'Airdrop'");

    // Esperar por fondos
    match wait_for_balance(&client, &my_wallet.pubkey().to_string(), MIN_BALANCE_FOR_TEST).await {
        Ok(balance) => {
            // Crear segunda wallet y probar transferencia
            let recipient = Keypair::new();
            println!("\n🔄 Probando transferencia de {} SOL a una nueva wallet...",
                TRANSFER_AMOUNT as f64 / 1_000_000_000.0);
            println!("📫 Dirección destino: {}", recipient.pubkey());
            
            // Intentar transferencia
            match crate::wallet::WalletClient::new(my_wallet)
                .transfer(&recipient.pubkey().to_string(), TRANSFER_AMOUNT).await 
            {
                Ok(signature) => {
                    println!("✅ ¡Transferencia exitosa!");
                    println!("📝 Firma de la transacción: {}", signature);
                    
                    // Verificar balances finales
                    match client.get_balance(&recipient.pubkey()) {
                        Ok(final_balance) => {
                            println!("💰 Balance final del destinatario: {} SOL", 
                                final_balance as f64 / 1_000_000_000.0);
                            if final_balance >= TRANSFER_AMOUNT {
                                println!("✅ ¡Transferencia verificada!");
                            } else {
                                println!("⚠️ La transferencia se completó pero el balance no es el esperado.");
                                println!("    Esto puede tomar unos segundos más en la red Devnet.");
                            }
                        },
                        Err(e) => error!("❌ Error al verificar balance final: {}", e)
                    }
                },
                Err(e) => error!("❌ Error en la transferencia: {}", e)
            }
        },
        Err(e) => {
            println!("\n❌ {}", e);
            println!("Por favor, asegúrate de usar el faucet y ejecuta el programa nuevamente.");
        }
    }

    Ok(())
}

fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
} 