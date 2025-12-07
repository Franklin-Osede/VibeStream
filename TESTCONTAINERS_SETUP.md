# Configuración de Testcontainers

> **Fecha**: Diciembre 2024  
> **Estado**: ✅ Configurado

---

## ✅ Implementación Completada

### 1. Dependencias Agregadas

**Archivo**: `services/api-gateway/Cargo.toml`

```toml
[dev-dependencies]
# Testcontainers para tests de integración
testcontainers = "0.15"
testcontainers-modules = { version = "0.1.0-beta.1", features = ["postgres", "redis"] }
```

### 2. Módulo de Setup Creado

**Archivo**: `services/api-gateway/tests/testcontainers_setup.rs`

**Características**:
- ✅ Configuración automática de PostgreSQL y Redis
- ✅ Helpers para esperar a que servicios estén listos
- ✅ Ejecución automática de migraciones
- ✅ Configuración de variables de entorno
- ✅ Helper para crear AppState con testcontainers

### 3. Tests Actualizados

**Archivo**: `services/api-gateway/tests/music_gateway_auth_tests.rs`

**Cambios**:
- ✅ Removido `#[ignore]` de todos los tests
- ✅ Agregado helper `setup_test_environment()` que usa testcontainers
- ✅ Todos los tests ahora usan testcontainers automáticamente

---

## 📋 Uso de Testcontainers

### Helper Principal

```rust
use crate::testcontainers_setup::TestContainersSetup;

#[tokio::test]
async fn test_example() {
    // Setup testcontainers
    let setup = TestContainersSetup::new();
    setup.setup_env();
    
    // Esperar a que los servicios estén listos
    setup.wait_for_postgres().await.expect("PostgreSQL debe estar listo");
    setup.wait_for_redis().await.expect("Redis debe estar listo");
    
    // Ejecutar migraciones
    setup.run_migrations().await.expect("Migraciones deben ejecutarse");
    
    // Crear AppState
    let app_state = AppState::new(
        &setup.get_postgres_url(),
        &setup.get_redis_url(),
    ).await.expect("Failed to create AppState");
    
    // Ejecutar test...
    
    // Cleanup automático (los containers se destruyen al salir del scope)
}
```

### Helper Simplificado

```rust
use crate::testcontainers_setup::setup_test_environment;

#[tokio::test]
async fn test_example() {
    // Setup completo en una línea
    let (_setup, _app_state, app) = setup_test_environment().await;
    
    // Ejecutar test...
}
```

---

## 🔧 Funcionalidades

### TestContainersSetup

**Métodos principales**:

1. **`new()`**: Crea nueva configuración e inicia containers
2. **`get_postgres_url()`**: Obtiene URL de conexión a PostgreSQL
3. **`get_redis_url()`**: Obtiene URL de conexión a Redis
4. **`wait_for_postgres()`**: Espera a que PostgreSQL esté listo
5. **`wait_for_redis()`**: Espera a que Redis esté listo
6. **`run_migrations()`**: Ejecuta migraciones automáticamente
7. **`setup_env()`**: Configura variables de entorno

### Características

- ✅ **Aislamiento**: Cada test tiene sus propios containers
- ✅ **Automatización**: No requiere servicios externos
- ✅ **Limpieza**: Containers se destruyen automáticamente
- ✅ **Reproducibilidad**: Mismo entorno en cada ejecución
- ✅ **Velocidad**: Containers se inician rápidamente

---

## 🚀 Ejecutar Tests

### Todos los Tests

```bash
cd services/api-gateway
cargo test
```

### Tests Específicos

```bash
# Tests de autenticación de Music Gateway
cargo test music_gateway_auth_tests

# Test específico
cargo test test_get_songs_public_route
```

### Con Output Detallado

```bash
cargo test -- --nocapture
```

---

## 📊 Ventajas de Testcontainers

### Antes (Sin Testcontainers)

❌ Requería servicios externos corriendo  
❌ Tests marcados con `#[ignore]`  
❌ Configuración manual de PostgreSQL y Redis  
❌ Posibles conflictos entre tests  
❌ Difícil de ejecutar en CI/CD  

### Después (Con Testcontainers)

✅ Tests ejecutables sin servicios externos  
✅ Tests siempre activos (sin `#[ignore]`)  
✅ Configuración automática  
✅ Aislamiento completo entre tests  
✅ Fácil de ejecutar en CI/CD  

---

## 🔍 Detalles Técnicos

### Versiones de Containers

- **PostgreSQL**: `15-alpine`
- **Redis**: `7-alpine`

### Configuración

- **PostgreSQL**:
  - Usuario: `postgres`
  - Password: `postgres`
  - Base de datos: `postgres`
  - Puerto: `5432` (mapeado dinámicamente)

- **Redis**:
  - Sin autenticación
  - Puerto: `6379` (mapeado dinámicamente)

### Timeouts

- **PostgreSQL**: 30 intentos × 500ms = 15 segundos máximo
- **Redis**: 30 intentos × 500ms = 15 segundos máximo

---

## 🐛 Troubleshooting

### Error: Docker no está corriendo

```bash
# Iniciar Docker
# macOS: Abrir Docker Desktop
# Linux: sudo systemctl start docker
```

### Error: Puerto ya en uso

Los puertos se mapean dinámicamente, así que no debería haber conflictos. Si ocurre, verifica que no haya containers huérfanos:

```bash
docker ps -a
docker rm -f $(docker ps -aq)
```

### Error: Migraciones no encontradas

Asegúrate de que el directorio `migrations` exista en la raíz del proyecto o ajusta las rutas en `run_migrations()`.

---

## 📝 Próximos Pasos

1. **Extender a otros tests**:
   - Actualizar otros tests de integración para usar testcontainers
   - Remover `#[ignore]` de tests que usen servicios

2. **Optimización**:
   - Considerar reutilizar containers entre tests (si es seguro)
   - Agregar pooling de conexiones para mejor performance

3. **CI/CD**:
   - Configurar GitHub Actions para usar testcontainers
   - Agregar tests de integración al pipeline

---

> **Última actualización**: Diciembre 2024

