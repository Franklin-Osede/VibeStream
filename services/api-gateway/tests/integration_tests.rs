use api_gateway::MessageQueue;
use vibestream_types::{
    Blockchain, ApiMessage, ServiceResponse, VibeStreamError, WalletAddress,
};

#[tokio::test]
async fn test_message_queue_connection() {
    // Test básico de conexión a Redis
    let result = MessageQueue::new("redis://127.0.0.1:6379").await;
    
    match result {
        Ok(_) => println!("✅ Conexión a Redis exitosa"),
        Err(e) => {
            println!("⚠️  Redis no disponible: {}", e);
            // No fallar la prueba si Redis no está disponible en desarrollo
            return;
        }
    }
}

#[tokio::test]
async fn test_service_request_serialization() {
    // Test de serialización de mensajes
    let request = ApiMessage::ProcessTransaction {
        blockchain: Blockchain::Ethereum,
        from: "0x123".to_string(),
        to: "0x456".to_string(),
        amount: 1000,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: ApiMessage = serde_json::from_str(&serialized).unwrap();

    match (request, deserialized) {
        (
            ApiMessage::ProcessTransaction { blockchain: b1, from: f1, to: t1, amount: a1 },
            ApiMessage::ProcessTransaction { blockchain: b2, from: f2, to: t2, amount: a2 },
        ) => {
            assert_eq!(format!("{:?}", b1), format!("{:?}", b2));
            assert_eq!(f1, f2);
            assert_eq!(t1, t2);
            assert_eq!(a1, a2);
        }
        _ => panic!("Serialización/deserialización falló"),
    }

    println!("✅ Serialización de mensajes funciona correctamente");
}

#[tokio::test]
async fn test_error_handling() {
    // Test del sistema de errores
    let error = VibeStreamError::Validation { message: "Test error".to_string() };
    let serialized = serde_json::to_string(&error).unwrap();
    let deserialized: VibeStreamError = serde_json::from_str(&serialized).unwrap();

    match (error, deserialized) {
        (VibeStreamError::Validation { message: msg1 }, VibeStreamError::Validation { message: msg2 }) => {
            assert_eq!(msg1, msg2);
        }
        _ => panic!("Error serialization failed"),
    }

    println!("✅ Sistema de errores funciona correctamente");
}

#[tokio::test]
async fn test_service_response_types() {
    // Test de diferentes tipos de respuesta
    let responses = vec![
        ServiceResponse::Error("Connection failed".to_string()),
        ServiceResponse::ZkVerification(true),
    ];

    for response in responses {
        let serialized = serde_json::to_string(&response).unwrap();
        let _deserialized: ServiceResponse = serde_json::from_str(&serialized).unwrap();
    }

    println!("✅ Tipos de respuesta funcionan correctamente");
} 