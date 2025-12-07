# 🔍 ANÁLISIS EXHAUSTIVO DEL BACKEND - ESTRATEGIA FINAL PRE-FRONTEND

> **Fecha**: Diciembre 2024  
> **Objetivo**: Determinar qué falta antes de iniciar desarrollo frontend  
> **Metodología**: TDD + DDD + Best Practices  
> **Estado Frontend**: No existe aún (ventaja: podemos definir contrato primero)

---

## 📊 RESUMEN EJECUTIVO

### Estado Actual del Backend

| Componente | Completitud | Estado Funcional | Listo para Frontend |
|------------|-------------|------------------|---------------------|
| **Gateway Unificado** | 90% | ✅ Funcional | ⚠️ Parcial |
| **Autenticación JWT** | 70% | ⚠️ Incompleto | ❌ No |
| **User Context** | 60% | ⚠️ Con mocks | ❌ No |
| **Music Context** | 50% | ⚠️ Con mocks | ❌ No |
| **Payment Context** | 40% | ⚠️ Sin integraciones | ❌ No |
| **OpenAPI Spec** | 40% | ⚠️ Incompleto | ❌ No |
| **Tests** | 15% | ⚠️ Solo fan_loyalty | ❌ No |
| **Migraciones/Seeds** | 80% | ✅ Funcional | ✅ Sí |
| **Configuración** | 70% | ⚠️ Parcial | ⚠️ Parcial |

**Veredicto**: ❌ **NO está listo para frontend**. Necesita **4-6 semanas** de trabajo enfocado.

---

## 🔬 ANÁLISIS DETALLADO POR COMPONENTE

### 1. GATEWAY UNIFICADO ✅ (90% Completo)

**Estado Actual**:
- ✅ `main_unified.rs` levanta en puerto 3000
- ✅ Enrutamiento por path: `/api/v1/users/*`, `/api/v1/music/*`, etc.
- ✅ CORS configurado (pero abierto a `Any`)
- ✅ Health checks implementados
- ⚠️ Routers antiguos multi-puerto aún existen pero están deprecados

**Problemas Identificados**:
1. **CORS demasiado permisivo**: `allow_origin(Any)` - inseguro para producción
2. **Sin rate limiting**: Vulnerable a abuso
3. **Logging básico**: Sin structured logging ni request IDs

**Qué Falta**:
- [ ] CORS restrictivo por dominio
- [ ] Rate limiting básico (tower-http)
- [ ] Structured logging con tracing
- [ ] Request ID middleware
- [ ] Métricas básicas (Prometheus opcional)

**Tiempo Estimado**: 1 día

---

### 2. AUTENTICACIÓN JWT ⚠️ (70% Completo)

**Estado Actual**:
- ✅ Middleware JWT implementado (`jwt_auth_middleware`)
- ✅ Extractor `AuthenticatedUser` funcional
- ✅ `JwtService` genera y valida tokens
- ✅ Refresh token implementado
- ⚠️ Secret hardcodeado con fallback inseguro
- ⚠️ Sin revocación de tokens
- ⚠️ No todos los handlers extraen `user_id` del JWT

**Problemas Críticos**:

#### 2.1. Secret Hardcodeado
```rust
// ACTUAL (línea 36 de middleware.rs):
let jwt_secret = std::env::var("JWT_SECRET")
    .unwrap_or_else(|_| "default_secret_change_in_production".to_string());
```
**Problema**: Fallback inseguro si no se configura `JWT_SECRET`

#### 2.2. Handlers con UUIDs Mock
```rust
// ACTUAL (línea 609 de user_controller.rs):
let follower_id = Uuid::new_v4(); // Mock for now

// DEBERÍA SER:
let AuthenticatedUser { user_id, .. } = extract_authenticated_user(request)?;
```

**Endpoints Afectados**:
- `follow_user` - Usa UUID mock
- `get_user_stats` - Algunos campos mock
- `get_user_analytics` - Datos mock
- `change_password` - No valida usuario actual
- `link_wallet` - No verifica firma

**Qué Falta**:
- [ ] Mover `JWT_SECRET` a configuración obligatoria (sin fallback)
- [ ] Extraer `user_id` de JWT en TODOS los handlers protegidos
- [ ] Implementar blacklist de refresh tokens (Redis)
- [ ] Validar permisos (solo puedes editar tu propio perfil)
- [ ] Middleware RBAC para roles (admin, artist, user)
- [ ] Tests de autenticación (unit + integration)

**Tiempo Estimado**: 2-3 días

---

### 3. USER CONTEXT ⚠️ (60% Completo)

**Estado Actual**:
- ✅ Registro/login/refresh conectados a Postgres
- ✅ Repositorio `PostgresUserRepository` implementado
- ✅ Migración `020_user_follows_table.sql` existe
- ✅ Seed data en `021_seed_dev_data.sql`
- ⚠️ Muchos handlers devuelven datos mock
- ⚠️ Analytics y estadísticas son mocks
- ⚠️ Sin tests de aplicación/presentación

**Análisis de Handlers**:

#### 3.1. Handlers Funcionales ✅
- `register_user` - ✅ Funcional con Postgres
- `login_user` - ✅ Funcional con JWT real
- `refresh_token` - ✅ Funcional
- `get_user_profile` - ⚠️ Parcial (campos mock: tier, role, is_verified)
- `follow_user` - ⚠️ Usa UUID mock en lugar de JWT
- `get_user_followers` - ✅ Usa repositorio real
- `get_user_following` - ✅ Usa repositorio real

#### 3.2. Handlers con Mocks ❌
- `get_user_stats` - ❌ Todos los datos son mock
- `get_user_analytics` - ❌ Datos mock
- `change_password` - ⚠️ Retorna éxito pero no cambia contraseña
- `link_wallet` - ⚠️ Retorna éxito pero no vincula wallet
- `delete_user` - ⚠️ Retorna éxito pero no elimina usuario

**Qué Falta**:
- [ ] Completar `get_user_profile` con datos reales (tier, role, is_verified desde DB)
- [ ] Implementar `get_user_stats` con queries reales a `listen_sessions`, `user_followers`, etc.
- [ ] Implementar `change_password` con validación de contraseña actual
- [ ] Implementar `link_wallet` con verificación de firma blockchain
- [ ] Implementar `delete_user` (soft delete o hard delete)
- [ ] Tests unitarios de `UserApplicationService`
- [ ] Tests e2e de flujos completos (register → login → follow → stats)

**Tiempo Estimado**: 3-4 días

---

### 4. MUSIC CONTEXT ⚠️ (50% Completo)

**Estado Actual**:
- ✅ CRUD de songs/albums/playlists conectado a Postgres
- ✅ Repositorios implementados: `PostgresSongRepository`, `PostgresAlbumRepository`, `PostgresPlaylistRepository`
- ✅ Controllers reales: `SongController`, `AlbumController`, `PlaylistController`
- ⚠️ Gateway `music_gateway.rs` tiene handlers mock que no usan controllers
- ⚠️ Discovery/trending son TODOs
- ⚠️ Sin storage/streaming real

**Problema Crítico: Dualidad de Handlers**

Existen DOS conjuntos de handlers:

1. **Controllers Reales** (en `bounded_contexts/music/presentation/controllers/`):
   - ✅ `SongController::get_songs` - Usa repositorio real
   - ✅ `SongController::create_song` - Usa repositorio real
   - ✅ `SongController::update_song` - Usa repositorio real

2. **Handlers Mock** (en `gateways/music_gateway.rs`):
   - ❌ `get_songs()` - Retorna `{"songs": [], "total": 0, "message": "TODO"}`
   - ❌ `discover_songs()` - Retorna `{"message": "TODO"}`
   - ❌ `get_trending_songs()` - Retorna `{"message": "TODO"}`

**El gateway usa los handlers mock en lugar de los controllers reales.**

**Qué Falta**:
- [ ] **CRÍTICO**: Reemplazar handlers mock en `music_gateway.rs` con controllers reales
- [ ] Implementar `discover_songs` con algoritmo real (basado en listen_count, likes, fecha)
- [ ] Implementar `get_trending_songs` con queries reales
- [ ] Implementar `like_song` / `unlike_song` (tabla `song_likes` si no existe)
- [ ] Storage service (S3/IPFS/local) para archivos de audio
- [ ] Streaming service (URLs válidas aunque sean dummy)
- [ ] Tests e2e de CRUD completo
- [ ] Tests de discovery/trending

**Tiempo Estimado**: 4-5 días

---

### 5. PAYMENT CONTEXT ⚠️ (40% Completo)

**Estado Actual**:
- ✅ Estructura DDD completa (domain, application, infrastructure)
- ✅ Repositorios: `PostgresPaymentRepository`, `PostgresRoyaltyRepository`, `PostgresWalletRepository`
- ✅ Controller `PaymentController` con rutas definidas
- ⚠️ Gateways externos (Stripe, PayPal, Coinbase) son mocks
- ⚠️ Sin integración real con procesadores
- ⚠️ Webhooks no procesan eventos reales

**Análisis de Implementación**:

#### 5.1. Gateways Externos
```rust
// stripe_gateway.rs línea 234:
// For test environment, return mock success
```
**Estado**: Estructura existe, pero lógica es mock

#### 5.2. Controller
- ✅ Rutas definidas correctamente
- ⚠️ Handlers probablemente tienen lógica parcial
- ❌ Sin tests de flujos completos

**Qué Falta**:
- [ ] **Decidir MVP**: ¿Solo pagos internos primero? ¿O integración real?
- [ ] Si MVP interno: Completar handlers con repos Postgres (sin gateways externos)
- [ ] Si integración real: Implementar Stripe Payment Intents real
- [ ] Webhooks handlers reales (idempotentes)
- [ ] Tests unitarios con repos in-memory/mock
- [ ] Tests e2e de flujo happy path + errores

**Tiempo Estimado**: 3-5 días (depende de alcance)

---

### 6. OPENAPI SPEC ⚠️ (40% Completo)

**Estado Actual**:
- ✅ Estructura base con `utoipa`
- ✅ Schemas definidos (User, Song, Album, Playlist, etc.)
- ✅ Paths documentados para users y music básico
- ⚠️ Cobertura parcial (solo algunos endpoints)
- ⚠️ No se valida contra rutas reales
- ⚠️ No se genera cliente TS automáticamente

**Problemas**:
1. **Endpoints faltantes**: Muchos endpoints no están documentados
2. **Validación**: No hay validación automática de que OpenAPI coincida con rutas reales
3. **Generación de cliente**: No hay pipeline para generar cliente TypeScript

**Qué Falta**:
- [ ] Documentar TODOS los endpoints activos (users, music, payments)
- [ ] Validación automática en CI (comparar OpenAPI con rutas reales)
- [ ] Generación de cliente TypeScript en build
- [ ] Endpoint de validación en runtime
- [ ] Swagger UI funcionando correctamente

**Tiempo Estimado**: 2-3 días

---

### 7. TESTING ⚠️ (15% Completo)

**Estado Actual**:
- ✅ Tests de `fan_loyalty` (completos)
- ✅ Estructura de tests (helpers, fixtures, testcontainers_setup)
- ⚠️ Tests de users/music/payments están `#[ignore]` o no existen
- ⚠️ Sin tests unitarios de servicios
- ⚠️ Sin tests e2e completos

**Tests Existentes**:
- `tests/fan_loyalty/tests/` - ✅ Funcionales
- `tests/register_login_integration_tests.rs` - ⚠️ Marcados con `#[ignore]`
- `tests/user_gateway_integration_tests.rs` - ⚠️ Parciales
- `tests/music_gateway_auth_tests.rs` - ⚠️ Parciales

**Qué Falta**:
- [ ] Habilitar tests con testcontainers (Postgres + Redis)
- [ ] Tests unitarios de `UserApplicationService`
- [ ] Tests unitarios de `SongController` (con repos mock)
- [ ] Tests e2e de flujos completos:
  - Register → Login → Follow → Stats
  - Create Song → List Songs → Discovery
  - Initiate Payment → Process → Complete
- [ ] Tests de autenticación (JWT válido/inválido, refresh, etc.)
- [ ] Pipeline CI que ejecute tests automáticamente

**Tiempo Estimado**: 4-5 días

---

### 8. MIGRACIONES Y SEEDS ✅ (80% Completo)

**Estado Actual**:
- ✅ 22 migraciones SQL completas
- ✅ Seed data en `021_seed_dev_data.sql` (usuarios, followers, tier progress)
- ✅ Migraciones se ejecutan automáticamente al arrancar (si `RUN_MIGRATIONS=true`)
- ⚠️ Rutas relativas podrían fallar según cwd

**Qué Falta**:
- [ ] Validar que todas las migraciones ejecutan sin errores
- [ ] Ajustar rutas de migraciones a absolutas o parametrizables
- [ ] Seed data más completa (artistas, canciones, playlists)
- [ ] Script de limpieza de datos de prueba

**Tiempo Estimado**: 1 día

---

### 9. OTROS BOUNDED CONTEXTS ❌ (No Listos)

**Campaign**:
- ⚠️ Gateway mock (`campaign_gateway.rs`)
- ✅ Use case `activate_campaign.rs` existe pero es standalone (sin repos)
- ❌ No conectado a base de datos

**Listen Reward**:
- ⚠️ Gateway placeholder
- ❌ Sin implementación real

**Fan Ventures**:
- ⚠️ Gateway placeholder
- ⚠️ Eventos con `unimplemented!`

**Notifications**:
- ⚠️ Gateway placeholder
- ❌ Sin implementación real

**Fan Loyalty**:
- ✅ Tests completos
- ⚠️ Gateway funcional pero no integrado completamente

**Estrategia**: Deshabilitar o feature-flag estos contextos hasta que estén listos.

---

## 🎯 ESTRATEGIA PASO A PASO (TDD/DDD)

### PRINCIPIOS FUNDAMENTALES

1. **Contract-First**: Definir contrato API antes de implementar
2. **TDD**: Escribir tests antes de código
3. **DDD**: Respetar bounded contexts y agregados
4. **Incremental**: Completar un contexto antes de pasar al siguiente
5. **Testing Real**: Usar testcontainers, no mocks en tests e2e

---

### FASE 1: CONGELAR CONTRATO Y RUTAS ACTIVAS (1 día)

**Objetivo**: Reducir superficie inestable y definir qué se expondrá al frontend.

**Tareas**:

1. **Catalogar rutas "reales" a exponer**:
   - ✅ Users: `/register`, `/login`, `/refresh`, `/:user_id`, `/:user_id/follow`, `/:user_id/stats`
   - ✅ Music: `/songs` (CRUD), `/albums` (CRUD), `/playlists` (CRUD)
   - ⚠️ Payments: Decidir MVP (solo `/payments/initiate`, `/payments/:id/process`?)

2. **Deshabilitar/feature-flag rutas mock**:
   ```rust
   // En main_unified.rs
   #[cfg(feature = "enable_mock_routes")]
   .nest("/api/v1/mobile", mobile_routes)
   ```
   - Deshabilitar: `mobile_api_routes.rs`
   - Deshabilitar: `campaign_gateway` (mock)
   - Deshabilitar: `listen_reward_gateway` (placeholder)
   - Deshabilitar: `fan_ventures_gateway` (placeholder)
   - Deshabilitar: `notification_gateway` (placeholder)

3. **Documentar decisiones**:
   - Crear `API_CONTRACT.md` con lista de endpoints estables
   - Marcar endpoints como "stable", "beta", "deprecated"

**Justificación TDD/DDD**:
- **Contract-First**: Define qué se expondrá antes de codificar frontend
- **Reducción de superficie**: Evita que frontend consuma endpoints inestables
- **Claridad**: Documenta qué está listo y qué no

**Criterios de Éxito**:
- [ ] Lista clara de endpoints estables
- [ ] Rutas mock deshabilitadas o detrás de feature flags
- [ ] Documentación actualizada

---

### FASE 2: COMPLETAR OPENAPI Y VALIDACIÓN (2 días)

**Objetivo**: Contrato verificable y generador de cliente TS.

**Tareas**:

1. **Documentar endpoints activos en OpenAPI**:
   ```rust
   // openapi/paths.rs
   // Agregar documentación para TODOS los endpoints de users/music/payments
   ```

2. **Generar openapi.json en build**:
   ```rust
   // En build.rs o script
   // Generar openapi.json automáticamente
   ```

3. **Validación automática**:
   ```rust
   // Endpoint de validación
   GET /api/v1/openapi/validate
   // Compara OpenAPI spec con rutas reales
   ```

4. **Generar cliente TypeScript**:
   ```bash
   # En CI o script
   openapi-generator-cli generate \
       -i openapi.json \
       -g typescript-axios \
       -o ../frontend/src/api-client
   ```

**Justificación TDD/DDD**:
- **Contract-First**: El contrato es la fuente de verdad
- **Validación**: Asegura que OpenAPI coincide con implementación
- **Generación automática**: Evita desincronización manual

**Criterios de Éxito**:
- [ ] Todos los endpoints activos documentados
- [ ] `openapi.json` generado automáticamente
- [ ] Cliente TypeScript generado
- [ ] Validación pasa en CI

---

### FASE 3: AUTENTICACIÓN SÓLIDA Y CONSISTENTE (2-3 días)

**Objetivo**: Eliminar datos mock en auth y garantizar identidad confiable.

**Tareas**:

1. **Mover JWT_SECRET a configuración obligatoria**:
   ```rust
   // Sin fallback inseguro
   let jwt_secret = std::env::var("JWT_SECRET")
       .expect("JWT_SECRET must be set");
   ```

2. **Aplicar middleware a todas las rutas protegidas**:
   ```rust
   // Verificar que TODOS los gateways usen jwt_auth_middleware
   ```

3. **Extraer claims en TODOS los handlers protegidos**:
   ```rust
   // Reemplazar UUIDs mock con:
   let AuthenticatedUser { user_id, .. } = extract_authenticated_user(request)?;
   ```

4. **Implementar blacklist de refresh tokens** (Redis):
   ```rust
   // Al hacer refresh, invalidar token anterior
   // Al hacer logout, invalidar refresh token
   ```

5. **Tests de autenticación**:
   ```rust
   #[tokio::test]
   async fn test_jwt_validation() { ... }
   
   #[tokio::test]
   async fn test_refresh_token_rotation() { ... }
   ```

**Justificación TDD/DDD**:
- **Seguridad**: Sin auth sólida, todo lo demás es inseguro
- **TDD**: Tests primero aseguran comportamiento correcto
- **DDD**: Auth es cross-cutting concern, debe ser consistente

**Criterios de Éxito**:
- [ ] JWT_SECRET obligatorio (sin fallback)
- [ ] Todos los handlers protegidos extraen user_id de JWT
- [ ] Blacklist de refresh tokens implementada
- [ ] Tests de autenticación pasando

---

### FASE 4: USERS LISTO PARA CONSUMO (3-4 días)

**Objetivo**: Flujo core de onboarding/social estable con TDD.

**Tareas**:

1. **Completar handlers con datos reales**:
   - `get_user_profile`: Obtener tier/role/is_verified desde DB
   - `get_user_stats`: Queries reales a `listen_sessions`, `user_followers`, etc.
   - `change_password`: Validar contraseña actual + actualizar
   - `link_wallet`: Verificar firma blockchain + guardar
   - `delete_user`: Soft delete o hard delete

2. **Tests TDD**:
   ```rust
   // 1. Escribir test primero
   #[tokio::test]
   async fn test_get_user_profile_with_real_data() {
       // Arrange: Crear usuario en DB
       // Act: Llamar endpoint
       // Assert: Verificar datos reales
   }
   
   // 2. Implementar hasta que pase
   // 3. Refactorizar
   ```

3. **Tests e2e con testcontainers**:
   ```rust
   // Flujo completo: Register → Login → Follow → Stats
   #[tokio::test]
   async fn test_user_onboarding_flow() { ... }
   ```

**Justificación TDD/DDD**:
- **TDD**: Tests primero garantizan comportamiento correcto
- **DDD**: User es agregado raíz, debe estar completo
- **Incremental**: Completar un contexto antes de pasar al siguiente

**Criterios de Éxito**:
- [ ] Todos los handlers devuelven datos reales (sin mocks)
- [ ] Tests unitarios de `UserApplicationService` pasando
- [ ] Tests e2e de flujos completos pasando
- [ ] Cobertura de tests > 80%

---

### FASE 5: MUSIC MÍNIMO FUNCIONAL (4-5 días)

**Objetivo**: Catálogo/descubrimiento estable para UI.

**Tareas**:

1. **CRÍTICO: Reemplazar handlers mock en music_gateway.rs**:
   ```rust
   // ACTUAL (mock):
   .route("/songs", get(get_songs))  // Handler mock
   
   // DEBERÍA SER:
   .route("/songs", get(SongController::get_songs))  // Controller real
   ```

2. **Implementar discovery/trending reales**:
   ```rust
   // discovery: Basado en listen_count, likes, fecha
   // trending: Últimas 24-48 horas, ordenado por listen_count
   ```

3. **Implementar like/unlike**:
   ```sql
   -- Crear tabla si no existe
   CREATE TABLE song_likes (
       id UUID PRIMARY KEY,
       song_id UUID REFERENCES songs(id),
       user_id UUID REFERENCES users(id),
       created_at TIMESTAMP DEFAULT NOW()
   );
   ```

4. **Storage/streaming mínimo**:
   - URLs dummy pero consistentes: `https://storage.vibestream.com/audio/{song_id}.mp3`
   - O usar IPFS gateway: `https://ipfs.io/ipfs/{ipfs_hash}`

5. **Tests e2e**:
   ```rust
   #[tokio::test]
   async fn test_song_crud_flow() {
       // Create → Read → Update → Delete
   }
   
   #[tokio::test]
   async fn test_discovery_algorithm() {
       // Verificar que discovery retorna canciones correctas
   }
   ```

**Justificación TDD/DDD**:
- **TDD**: Tests primero para discovery/trending
- **DDD**: Music es bounded context, debe estar completo
- **Incremental**: Completar antes de pasar a payments

**Criterios de Éxito**:
- [ ] Handlers mock reemplazados por controllers reales
- [ ] Discovery/trending implementados con queries reales
- [ ] Like/unlike funcional
- [ ] Tests e2e pasando
- [ ] URLs de streaming consistentes (aunque dummy)

---

### FASE 6: PAYMENTS MVP CONTROLADO (3-5 días)

**Objetivo**: Habilitar flujos de compra/inversión básicos.

**Tareas**:

1. **Decidir alcance MVP**:
   - Opción A: Solo pagos internos (sin gateways externos)
   - Opción B: Integración real con Stripe (más tiempo)

2. **Si Opción A (MVP interno)**:
   ```rust
   // Completar handlers con repos Postgres
   // Mockear gateways externos detrás de traits
   ```

3. **Si Opción B (Integración real)**:
   ```rust
   // Implementar Stripe Payment Intents
   // Webhooks handlers reales
   ```

4. **Tests**:
   ```rust
   // Tests unitarios con repos in-memory
   // Tests e2e con Postgres (happy path + errores)
   ```

**Justificación TDD/DDD**:
- **TDD**: Tests primero para flujos de pago
- **DDD**: Payment es bounded context complejo, MVP primero
- **Incremental**: Decidir alcance antes de implementar

**Criterios de Éxito**:
- [ ] Alcance MVP definido
- [ ] Handlers completos (internos o con Stripe)
- [ ] Tests unitarios + e2e pasando
- [ ] Flujo happy path funcional

---

### FASE 7: MIGRACIONES Y SEEDS REPRODUCIBLES (1 día)

**Objetivo**: Entornos consistentes y datos para pruebas.

**Tareas**:

1. **Validar migraciones**:
   ```bash
   sqlx migrate run
   # Verificar que no hay errores
   ```

2. **Ajustar rutas de migraciones**:
   ```rust
   // Usar ruta absoluta o parametrizable
   let migrations_path = env::var("MIGRATIONS_PATH")
       .unwrap_or_else(|_| "./migrations".to_string());
   ```

3. **Seed data completa**:
   ```sql
   -- Usuarios, artistas, canciones, playlists, etc.
   ```

4. **Script de limpieza**:
   ```sql
   -- Limpiar datos de prueba
   ```

**Justificación TDD/DDD**:
- **Reproducibilidad**: Tests necesitan datos consistentes
- **DDD**: Seeds respetan agregados y bounded contexts

**Criterios de Éxito**:
- [ ] Todas las migraciones ejecutan sin errores
- [ ] Seed data completa disponible
- [ ] Script de limpieza funcional

---

### FASE 8: LIMPIEZA DE MOCKS Y RUIDO (1 día)

**Objetivo**: Evitar que frontend consuma respuestas vacías.

**Tareas**:

1. **Eliminar o proteger rutas mock**:
   - Feature flags o deshabilitar completamente
   - Documentar en OpenAPI qué no está listo

2. **Limpiar código muerto**:
   - Eliminar handlers mock no usados
   - Eliminar routers antiguos multi-puerto (si no se usan)

**Justificación TDD/DDD**:
- **Claridad**: Evita confusión sobre qué está listo
- **Mantenibilidad**: Menos código = menos bugs

**Criterios de Éxito**:
- [ ] Rutas mock deshabilitadas o documentadas
- [ ] Código muerto eliminado
- [ ] OpenAPI actualizado

---

### FASE 9: OBSERVABILIDAD Y SEGURIDAD BÁSICA (1 día)

**Objetivo**: Operabilidad mínima para QA/staging.

**Tareas**:

1. **CORS restrictivo**:
   ```rust
   CorsLayer::new()
       .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
       .allow_methods([Method::GET, Method::POST, ...])
   ```

2. **Rate limiting básico**:
   ```rust
   // tower-http rate-limit
   .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
   ```

3. **Logging estructurado**:
   ```rust
   // tracing con request_id
   tracing::info!(request_id = %request_id, "Request received");
   ```

4. **Health checks por dependencia**:
   ```rust
   // Ya existe en AppState, exponer en endpoint
   GET /health
   {
     "database": "healthy",
     "redis": "healthy",
     "overall": "healthy"
   }
   ```

**Justificación TDD/DDD**:
- **Operabilidad**: Necesario para QA/staging
- **Seguridad**: CORS y rate limiting básicos

**Criterios de Éxito**:
- [ ] CORS restrictivo configurado
- [ ] Rate limiting básico funcionando
- [ ] Logging estructurado con request IDs
- [ ] Health checks expuestos

---

### FASE 10: CI + CALIDAD (1 día)

**Objetivo**: Evitar regresiones antes de que frontend llegue.

**Tareas**:

1. **Workflow CI**:
   ```yaml
   # .github/workflows/ci.yml
   - name: Run tests
     run: cargo test --all-features
   
   - name: Generate OpenAPI
     run: cargo run --bin generate-openapi
   
   - name: Publish OpenAPI artifact
     uses: actions/upload-artifact@v3
     with:
       name: openapi.json
       path: openapi.json
   ```

2. **Linting**:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

**Justificación TDD/DDD**:
- **Calidad**: CI asegura que no se rompa nada
- **Contract-First**: OpenAPI se publica como artefacto

**Criterios de Éxito**:
- [ ] CI ejecuta tests automáticamente
- [ ] OpenAPI se publica como artefacto
- [ ] Linting pasa

---

### FASE 11: HANDOVER PARA FRONTEND (1 día)

**Objetivo**: Que el equipo de frontend pueda empezar sin incertidumbre.

**Tareas**:

1. **Publicar openapi.json**:
   - En CI o manualmente
   - URL accesible: `https://api.vibestream.com/openapi.json`

2. **Generar cliente TypeScript**:
   ```bash
   openapi-generator-cli generate \
       -i openapi.json \
       -g typescript-axios \
       -o ../frontend/src/api-client
   ```

3. **Documentación**:
   - `API_CONTRACT.md` con lista de endpoints estables
   - Ejemplos de request/response
   - Guía de autenticación

4. **`.env.example`**:
   ```env
   DATABASE_URL=...
   REDIS_URL=...
   JWT_SECRET=...
   ```

**Justificación TDD/DDD**:
- **Contract-First**: Frontend usa contrato, no adivina
- **Paralelización**: Frontend puede empezar mientras backend se completa

**Criterios de Éxito**:
- [ ] openapi.json publicado
- [ ] Cliente TypeScript generado
- [ ] Documentación completa
- [ ] `.env.example` actualizado

---

## 📅 CRONOGRAMA ESTIMADO

| Fase | Tareas | Tiempo | Dependencias |
|------|--------|--------|--------------|
| **Fase 1** | Congelar contrato | 1 día | - |
| **Fase 2** | OpenAPI completo | 2 días | Fase 1 |
| **Fase 3** | Autenticación sólida | 2-3 días | - |
| **Fase 4** | Users listo | 3-4 días | Fase 3 |
| **Fase 5** | Music funcional | 4-5 días | Fase 3 |
| **Fase 6** | Payments MVP | 3-5 días | Fase 3 |
| **Fase 7** | Migraciones/seeds | 1 día | - |
| **Fase 8** | Limpieza mocks | 1 día | Fases 4-6 |
| **Fase 9** | Observabilidad | 1 día | - |
| **Fase 10** | CI + calidad | 1 día | Fases 2-8 |
| **Fase 11** | Handover | 1 día | Fase 10 |

**Total**: **20-25 días** (4-5 semanas)

**Paralelización posible**:
- Fases 4, 5, 6 pueden hacerse en paralelo (diferentes bounded contexts)
- Fase 7 puede hacerse en paralelo con otras

**Tiempo realista**: **4-6 semanas** (considerando imprevistos)

---

## ✅ CHECKLIST FINAL PRE-FRONTEND

### Contrato API
- [ ] OpenAPI spec completo y validado
- [ ] Cliente TypeScript generado
- [ ] Documentación de endpoints estables
- [ ] Ejemplos de request/response

### Autenticación
- [ ] JWT_SECRET obligatorio (sin fallback)
- [ ] Todos los handlers extraen user_id de JWT
- [ ] Blacklist de refresh tokens
- [ ] Tests de autenticación pasando

### Users Context
- [ ] Handlers devuelven datos reales (sin mocks)
- [ ] Tests unitarios + e2e pasando
- [ ] Cobertura > 80%

### Music Context
- [ ] Handlers mock reemplazados por controllers reales
- [ ] Discovery/trending implementados
- [ ] Tests e2e pasando

### Payments Context
- [ ] MVP definido e implementado
- [ ] Tests pasando

### Testing
- [ ] Tests habilitados (no `#[ignore]`)
- [ ] Testcontainers configurado
- [ ] CI ejecuta tests automáticamente

### Infraestructura
- [ ] Migraciones validadas
- [ ] Seed data completa
- [ ] CORS restrictivo
- [ ] Rate limiting básico
- [ ] Logging estructurado
- [ ] Health checks expuestos

### Calidad
- [ ] Linting pasa
- [ ] CI configurado
- [ ] OpenAPI publicado como artefacto

---

## 🎯 CONCLUSIÓN

El análisis previo es **preciso**. El backend necesita **4-6 semanas** de trabajo enfocado antes de que el frontend pueda consumirlo efectivamente.

**Prioridades**:
1. **Contract-First**: OpenAPI completo
2. **Autenticación sólida**: Base de todo
3. **Users + Music**: Contextos core
4. **Payments MVP**: Decidir alcance
5. **Testing**: Garantizar calidad

**Ventaja**: Como el frontend aún no existe, podemos definir el contrato primero y trabajar en paralelo.

**Riesgo**: Si no se completa esta estrategia, el frontend se pegará a mocks y habrá que rehacer trabajo.

---

## 📚 REFERENCIAS

- Análisis previo: `ANALISIS_ESTADO_ACTUAL_PROXIMOS_PASOS.md`
- Backend gaps: `BACKEND_GAPS_ANALYSIS.md`
- Testing progress: `TESTING_PROGRESS.md`
- OpenAPI progress: `PROGRESO_OPENAPI.md`
