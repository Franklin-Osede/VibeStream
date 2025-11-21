# Test Fixtures y Configuración

## Requisitos Previos

### Docker Compose (Recomendado)

```bash
# Desde la raíz del proyecto
docker-compose up -d postgres redis
```

### Manual

1. **PostgreSQL**
   ```bash
   # Crear base de datos de test
   createdb -U vibestream vibestream_test
   
   # Aplicar migraciones
   cd services/api-gateway
   DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream_test sqlx migrate run
   ```

2. **Redis**
   ```bash
   # Redis debe estar corriendo en localhost:6379
   redis-server
   ```

## Ejecutar Tests con Fixtures

### Opción 1: Tests con Servicios Reales

```bash
# 1. Configurar variables de entorno
export TEST_DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream_test
export TEST_REDIS_URL=redis://localhost:6379/1
export TEST_JWT_SECRET=test_secret_key_for_testing_only

# 2. Ejecutar tests ignorados
cargo test -- --ignored
```

### Opción 2: Usar Script de Setup

```bash
# Crear script de setup (ver scripts/test-setup.sh)
./scripts/test-setup.sh

# Ejecutar tests
cargo test -- --ignored
```

### Opción 3: Testcontainers (Futuro)

Cuando se implemente testcontainers, los tests levantarán servicios automáticamente:

```rust
#[tokio::test]
async fn test_with_containers() {
    let postgres = Postgres::new("postgres:15").start().await;
    let redis = Redis::new("redis:7").start().await;
    
    // Tests aquí
}
```

## Limpieza de Datos

Los tests deben limpiar datos después de ejecutarse:

```rust
use crate::fixtures::{cleanup_test_data, cleanup_test_redis};

#[tokio::test]
async fn test_example() {
    // Setup
    let services = TestServices::new();
    services.check_services().await.unwrap();
    
    // Test code...
    
    // Cleanup
    cleanup_test_data(&services.postgres_url).await.unwrap();
    cleanup_test_redis(&services.redis_url).await.unwrap();
}
```

## Variables de Entorno

| Variable | Default | Descripción |
|----------|---------|-------------|
| `TEST_DATABASE_URL` | `postgresql://.../vibestream_test` | URL de PostgreSQL para tests |
| `TEST_REDIS_URL` | `redis://localhost:6379/1` | URL de Redis para tests (DB 1) |
| `TEST_JWT_SECRET` | `test_secret_key...` | Secret para JWT en tests |

## Troubleshooting

### Error: "Connection refused"

1. Verificar que servicios estén corriendo:
   ```bash
   docker ps  # Para Docker
   pg_isready -h localhost -p 5433  # Para PostgreSQL
   redis-cli ping  # Para Redis
   ```

2. Verificar variables de entorno:
   ```bash
   echo $TEST_DATABASE_URL
   echo $TEST_REDIS_URL
   ```

### Error: "Database does not exist"

```bash
createdb -U vibestream vibestream_test
cd services/api-gateway
DATABASE_URL=$TEST_DATABASE_URL sqlx migrate run
```

### Tests Lentos

- Usar Redis DB separada (DB 1) para tests
- Usar base de datos de test separada
- Limpiar datos entre tests

