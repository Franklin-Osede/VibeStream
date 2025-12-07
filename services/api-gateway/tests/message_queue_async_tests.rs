// =============================================================================
// MESSAGE QUEUE ASYNC MIGRATION TESTS - TDD APPROACH
// =============================================================================
// 
// RED PHASE: Tests que verifican que MessageQueue usa conexiones async
// GREEN PHASE: Migrar MessageQueue a redis::aio
// 
// Usa testcontainers para levantar Redis automáticamente

use api_gateway::services::MessageQueue;
use tokio::time::{timeout, Duration};

// Importar testcontainers setup
use crate::testcontainers_setup::TestContainersSetup;

// =============================================================================
// TEST 1: MessageQueue debe usar conexiones async (no bloquear)
// =============================================================================

#[tokio::test]
async fn test_message_queue_uses_async_connections() {
    // Arrange: Setup testcontainers (solo Redis necesario)
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    
    let redis_url = setup.get_redis_url();
    let message_queue = MessageQueue::new(&redis_url)
        .await
        .expect("Failed to create MessageQueue");

    // Act: Ejecutar ping con timeout (si bloquea, el timeout fallará)
    let ping_result = timeout(
        Duration::from_secs(2),
        message_queue.ping()
    ).await;

    // Assert: Debe completar sin timeout (no bloquea)
    assert!(
        ping_result.is_ok(),
        "MessageQueue ping should complete without blocking (async)"
    );
    
    let ping_ok = ping_result.unwrap();
    assert!(
        ping_ok.is_ok(),
        "MessageQueue ping should succeed"
    );
}

// =============================================================================
// TEST 2: send_message debe ser completamente async
// =============================================================================

#[tokio::test]
async fn test_send_message_is_async() {
    // Arrange: Setup testcontainers (solo Redis necesario)
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    
    let redis_url = setup.get_redis_url();
    let message_queue = MessageQueue::new(&redis_url)
        .await
        .expect("Failed to create MessageQueue");

    // Act: Enviar mensaje con timeout
    let send_result = timeout(
        Duration::from_secs(2),
        message_queue.send_message("test_queue", "test_message")
    ).await;

    // Assert: Debe completar sin timeout
    assert!(
        send_result.is_ok(),
        "MessageQueue send_message should complete without blocking (async)"
    );
    
    let send_ok = send_result.unwrap();
    assert!(
        send_ok.is_ok(),
        "MessageQueue send_message should succeed"
    );
}

// =============================================================================
// TEST 3: Múltiples operaciones concurrentes deben funcionar
// =============================================================================

#[tokio::test]
async fn test_concurrent_operations() {
    // Arrange: Setup testcontainers (solo Redis necesario)
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    
    let redis_url = setup.get_redis_url();
    let message_queue = MessageQueue::new(&redis_url)
        .await
        .expect("Failed to create MessageQueue");

    // Act: Ejecutar múltiples operaciones concurrentes
    let mut handles = vec![];
    
    for i in 0..10 {
        let queue = message_queue.clone();
        let handle = tokio::spawn(async move {
            queue.send_message("concurrent_test", &format!("message_{}", i)).await
        });
        handles.push(handle);
    }

    // Esperar todas las operaciones
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task should complete"))
        .collect();

    // Assert: Todas deben completar exitosamente
    for result in results {
        assert!(
            result.is_ok(),
            "All concurrent operations should succeed"
        );
    }
}

// =============================================================================
// TEST 4: MessageQueue debe poder clonarse y usarse en múltiples tasks
// =============================================================================

#[tokio::test]
async fn test_message_queue_clone_and_share() {
    // Arrange: Setup testcontainers (solo Redis necesario)
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    
    let redis_url = setup.get_redis_url();
    let message_queue = MessageQueue::new(&redis_url)
        .await
        .expect("Failed to create MessageQueue");

    // Act: Clonar y usar en diferentes tasks
    let queue1 = message_queue.clone();
    let queue2 = message_queue.clone();

    let task1 = tokio::spawn(async move {
        queue1.ping().await
    });

    let task2 = tokio::spawn(async move {
        queue2.send_message("clone_test", "message").await
    });

    let (result1, result2) = tokio::join!(task1, task2);

    // Assert: Ambas tareas deben completar exitosamente
    assert!(
        result1.is_ok() && result1.unwrap().is_ok(),
        "Cloned queue should work in task 1"
    );
    
    assert!(
        result2.is_ok() && result2.unwrap().is_ok(),
        "Cloned queue should work in task 2"
    );
}

