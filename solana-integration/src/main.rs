use solana_integration::SolanaClient;
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() {
    // Inicializar el logger
    solana_logger::setup_with_default("info");

    // Crear un nuevo keypair para pruebas
    let keypair = Keypair::new();
    
    // Crear el cliente de Solana
    let _client = SolanaClient::new(keypair);
    
    println!("Servicio de Solana iniciado correctamente");
    
    // TODO: Aquí añadiremos la lógica del servidor
    tokio::signal::ctrl_c()
        .await
        .expect("Error al configurar Ctrl+C handler");
    println!("Servicio de Solana detenido");
} 