# 🚀 Guía de Setup Rápido - VibeStream Backend

> **Para principiantes**: Esta guía te ayudará a configurar el backend en menos de 10 minutos.

---

## ⚡ Setup Automático (Recomendado)

El método más rápido es usar el script de setup:

```bash
# Desde la raíz del proyecto
./scripts/setup-dev.sh
```

Este script automáticamente:
- ✅ Inicia PostgreSQL y Redis con Docker
- ✅ Ejecuta todas las migraciones
- ✅ Genera un JWT_SECRET seguro
- ✅ Crea el archivo `.env` con todas las configuraciones

**Después del script**, solo necesitas:

```bash
cd services/api-gateway
cargo run --bin api-gateway-unified
```

---

## 📋 Setup Manual (Paso a Paso)

Si prefieres hacerlo manualmente o el script falla:

### 1. Iniciar PostgreSQL y Redis

```bash
# Desde la raíz del proyecto
docker-compose up -d postgres redis

# Verificar que están corriendo
docker-compose ps
```

**Espera 30 segundos** para que PostgreSQL esté completamente listo.

### 2. Ejecutar Migraciones

```bash
cd services/api-gateway

# Configurar DATABASE_URL
export DATABASE_URL="postgresql://vibestream:vibestream@localhost:5433/vibestream"

# Ejecutar migraciones
sqlx migrate run
```

**Nota**: Si no tienes `sqlx-cli` instalado:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 3. Configurar JWT_SECRET

```bash
# Generar secreto seguro
openssl rand -base64 32

# Copiar el resultado y usarlo en el siguiente paso
```

### 4. Crear archivo .env

Crea `services/api-gateway/.env` con este contenido:

```bash
# Database
DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream
TEST_DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream_test

# Redis
REDIS_URL=redis://localhost:6379

# JWT (usa el secreto generado en el paso 3)
JWT_SECRET=TU_SECRETO_AQUI
JWT_ACCESS_TOKEN_EXPIRY=3600
JWT_REFRESH_TOKEN_EXPIRY=2592000

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Environment
ENVIRONMENT=development
RUST_LOG=info
```

### 5. Iniciar el Servidor

```bash
cd services/api-gateway
cargo run --bin api-gateway-unified
```

Deberías ver:
```
🚀 VibeStream Unified API Gateway iniciado:
   🌐 Base URL: http://127.0.0.1:3000
```

---

## ✅ Verificar que Todo Funciona

### 1. Health Check

```bash
curl http://localhost:3000/health
```

Debería retornar:
```json
{
  "status": "healthy",
  "service": "vibestream-unified-api-gateway",
  ...
}
```

### 2. Información de la API

```bash
curl http://localhost:3000/api/v1/info
```

### 3. Probar Registro de Usuario

```bash
curl -X POST http://localhost:3000/api/v1/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "securepass123",
    "confirm_password": "securepass123",
    "terms_accepted": true
  }'
```

### 4. Ver Documentación OpenAPI

Abre en tu navegador:
- **Swagger UI**: http://localhost:3000/swagger-ui
- **ReDoc**: http://localhost:3000/redoc
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

---

## 🐛 Solución de Problemas

### Error: "Connection refused" al iniciar el servidor

**Causa**: PostgreSQL no está corriendo o no está listo.

**Solución**:
```bash
# Verificar que PostgreSQL está corriendo
docker-compose ps

# Si no está corriendo, iniciarlo
docker-compose up -d postgres

# Esperar 30 segundos y verificar
docker-compose exec postgres pg_isready -U vibestream
```

### Error: "JWT_SECRET environment variable is required"

**Causa**: No has configurado JWT_SECRET en el archivo `.env`.

**Solución**:
1. Asegúrate de que el archivo `.env` existe en `services/api-gateway/`
2. Verifica que tiene la línea `JWT_SECRET=...`
3. Si usas el script de setup, esto se hace automáticamente

### Error: "error communicating with database" durante compilación

**Causa**: `sqlx` intenta validar queries en tiempo de compilación y necesita conexión a la DB.

**Solución**:
```bash
# Opción 1: Tener PostgreSQL corriendo durante compilación
docker-compose up -d postgres

# Opción 2: Compilar en modo offline (si ya compilaste antes)
cargo build --offline
```

### Error: "No such file or directory: migrations"

**Causa**: Estás ejecutando `sqlx migrate run` desde el directorio incorrecto.

**Solución**:
```bash
# Asegúrate de estar en el directorio correcto
cd services/api-gateway

# O especifica la ruta completa
sqlx migrate run --source ../../migrations
```

---

## 📚 Recursos Adicionales

- **API Contract**: Ver `API_CONTRACT.md` para detalles de endpoints
- **Análisis Backend**: Ver `ANALISIS_EXHAUSTIVO_BACKEND_COMPLETO.md` para análisis profundo
- **Esquema DB**: Ver `ESQUEMA_BASE_DATOS_RELACIONES.md` para estructura de base de datos

---

## 🎯 Próximos Pasos

Una vez que el servidor esté corriendo:

1. **Probar endpoints principales**:
   - Registro/Login de usuarios
   - CRUD de canciones
   - CRUD de álbumes y playlists

2. **Revisar documentación OpenAPI**:
   - Swagger UI tiene ejemplos interactivos

3. **Ejecutar tests**:
   ```bash
   cd services/api-gateway
   cargo test
   ```

4. **Empezar con el frontend**:
   - El backend está listo para ser consumido
   - Usa la especificación OpenAPI para generar clientes

---

## 💡 Tips

- **Desarrollo**: Usa `RUST_LOG=debug` para logs más detallados
- **Tests**: Los tests usan testcontainers automáticamente, no necesitas configurar nada
- **Hot Reload**: Considera usar `cargo watch` para recargar automáticamente:
  ```bash
  cargo install cargo-watch
  cargo watch -x 'run --bin api-gateway-unified'
  ```

---

> **¿Problemas?** Revisa los logs con `docker-compose logs postgres` o crea un issue en el repositorio.
