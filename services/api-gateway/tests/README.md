# API Gateway Integration Tests

Este directorio contiene los tests de integración para el API Gateway.

## Requisitos de Infraestructura

Los tests de integración requieren servicios externos en ejecución:

### Servicios Requeridos

1. **PostgreSQL**
   - Puerto: `5433` (por defecto)
   - Base de datos: `vibestream`
   - Usuario: `vibestream`
   - Password: `vibestream`
   - URL: `postgresql://vibestream:vibestream@localhost:5433/vibestream`

2. **Redis**
   - Puerto: `6379` (por defecto)
   - URL: `redis://localhost:6379`

### Variables de Entorno

Los tests usan las siguientes variables de entorno (con valores por defecto):

```bash
DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream
REDIS_URL=redis://localhost:6379
JWT_SECRET=default_secret_change_in_production
```

## Ejecutar Tests

### Todos los Tests

```bash
cd services/api-gateway
cargo test
```

### Tests Específicos

```bash
# Tests de MessageQueue async
cargo test message_queue_async_tests

# Tests de register/login
cargo test register_login_integration_tests

# Tests de auth middleware
cargo test auth_middleware_tests
```

### Ejecutar Tests Ignorados

Los tests marcados con `#[ignore]` requieren servicios activos:

```bash
# Ejecutar todos los tests, incluyendo los ignorados
cargo test -- --ignored

# Ejecutar un test específico ignorado
cargo test test_register_creates_user_and_returns_token -- --ignored
```

## Configuración con Docker

Para levantar los servicios necesarios:

```bash
# Desde la raíz del proyecto
docker-compose up -d postgres redis
```

O usar los scripts de desarrollo:

```bash
./scripts/dev-start.sh
```

## Tests por Categoría

### Tests de Integración End-to-End

- **`register_login_integration_tests.rs`**: Tests completos del flujo de autenticación
  - Registro de usuarios
  - Login con credenciales
  - Validación de errores
  - **Estado**: Requieren Postgres activo

### Tests de Middleware

- **`auth_middleware_tests.rs`**: Tests del middleware de autenticación JWT
  - Rutas protegidas vs públicas
  - Validación de tokens
  - **Estado**: Funcionan sin servicios externos

### Tests de MessageQueue

- **`message_queue_async_tests.rs`**: Tests de la cola de mensajes async
  - Verificación de conexiones async
  - Operaciones concurrentes
  - **Estado**: Requieren Redis activo

### Tests de OpenAPI

- **`openapi_integration_tests.rs`**: Tests de generación de documentación OpenAPI
  - Verificación de paths registrados
  - **Estado**: Funcionan sin servicios externos

## Troubleshooting

### Error: "Connection refused"

- Verificar que PostgreSQL y Redis estén corriendo
- Verificar que los puertos sean correctos
- Verificar variables de entorno

### Error: "Database does not exist"

- Crear la base de datos: `createdb -U vibestream vibestream`
- O ejecutar migraciones: `sqlx migrate run`

### Tests Fallan con Timeout

- Verificar que los servicios no estén sobrecargados
- Aumentar timeout en tests si es necesario
- Verificar logs de servicios

## CI/CD

En CI, los tests deben ejecutarse con servicios en contenedores:

```yaml
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: vibestream
      POSTGRES_USER: vibestream
      POSTGRES_PASSWORD: vibestream
  redis:
    image: redis:7-alpine
```

## Próximos Pasos

- [ ] Configurar testcontainers para tests automáticos
- [ ] Agregar fixtures/mocks para tests sin servicios
- [ ] Habilitar tests ignorados en CI
- [ ] Documentar estructura de respuestas esperadas

