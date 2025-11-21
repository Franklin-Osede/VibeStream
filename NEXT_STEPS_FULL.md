# VibeStream Backend – Detailed Next Steps

> Última revisión: _(actualiza la fecha al compartir)_  
> Contexto analizado: `services/api-gateway`, documentación adjunta y bounded contexts activos

---

## 1. Snapshot de Estado

- **Multi-gateway Axum** (`services/api-gateway/src/main.rs`): 9 routers corren en paralelo; falta observabilidad y supervisión de puertos.
- **`AppState` compartido** (`shared/infrastructure/app_state.rs`): abstrae Redis + PostgreSQL + EventBus, pero el bus actual sigue in-memory.
- **Autenticación/OAuth** (`oauth/real_providers.rs`, `user_controller.rs`): proveedores reales listos, controladores aún no usan `JwtService` ni `PasswordService`.
- **Fan Loyalty** (`gateways/fan_loyalty_gateway.rs`, `bounded_contexts/fan_loyalty/...`): routers reales con DTOs extensos; handlers siguen en modo TDD pendiente.
- **Notificaciones** (`notifications/infrastructure/postgres_repository.rs`): repositorio existe pero hay errores evidentes (campos duplicados, metadata opcional sin migración).
- **Documentación** (`openapi/mod.rs`): define esquemas pero no registra `paths`, impidiendo generar SDKs.
- **Servicios compartidos** (`services.rs`): Redis usa conexiones bloqueantes; ideal migrar a `redis::aio` y health checks unificados.

---

## 2. Prioridad Semana 0‑2 (Blocking para Front/QA)

1. **Cerrar capa Auth**  
   - ✅ `JwtService` + `PasswordService` cableados en `user_controller.rs` (register/login implementados)
   - ✅ Middleware de protección aplicado a rutas protegidas
   - ⚠️ Pendiente: Refresh token endpoints y token rotation
2. **Gateways de Pago reales**  
   - Implementar Stripe/Coinbase/PayPal en `payment/infrastructure/gateways`.  
   - Configurar webhooks y colas Redis para reconciliación.  
3. **Event Bus Persistente**  
   - Reemplazar `InMemoryEventBus` por Redis Streams/Kafka.  
   - Registrar handlers reales para User, Music, Campaign, Listen Reward, Fan Ventures.  
4. **OpenAPI completo**  
   - ✅ Paths definidos en `openapi/paths.rs` (register, login, user profile, songs, campaigns)
   - ✅ Schemas definidos (User, LoginRequest, ApiResponse, etc.)
   - ⚠️ Pendiente: Arreglar errores de compilación en OpenAPI y servir Swagger/Redoc reales
   - ⚠️ Pendiente: Automatizar generación de clientes Angular (`ng-openapi-gen`)  
5. **Testing base**  
   - ✅ Tests de integración creados: `register_login_integration_tests.rs` (5 tests), `message_queue_async_tests.rs` (4 tests)
   - ✅ Endpoints de register/login implementados con JwtService y PasswordService
   - ⚠️ **Estado actual**: Tests marcados con `#[ignore]` hasta configurar Postgres/Redis
   - ✅ Fixtures creados: `tests/fixtures.rs` y `tests/README_FIXTURES.md`
   - **Requisitos**: Ver `services/api-gateway/tests/README.md` y `tests/README_FIXTURES.md`
   - **Próximo paso**: Configurar servicios en CI o implementar testcontainers para habilitar tests automáticamente  

---

## 3. Roadmap por Bounded Context

### User Context
- Diseñar aggregate `User` + value objects (roles, tiers, wallets).  
- Completar capa application (commands, queries, handlers) y servicios de dominio (notificaciones, analytics).  
- Implementar RBAC + políticas (`axum::middleware`) y endpoints de perfil/artista.  
- Integrar OAuth providers (Google/Apple) con `AuthService`.  

### Payment Context
- Terminar gateways reales + clients firmes (Stripe Payment Intents, Coinbase Commerce, PayPal Orders).  
- Crear `PaymentOrchestrator` con sagas para reconciliar campañas, listen rewards y fractional ownership.  
- Añadir antifraude (reglas heurísticas + hook ML externo) y auditoría detallada en PostgreSQL.  
- Testing E2E con simuladores y verificación de webhooks idempotentes.

### Music Context
- Infraestructura de subida: storage service (S3/IPFS), procesamiento y validación de metadata.  
- Streaming service (HLS/DASH) + DRM ligero + rate limiting.  
- Analytics de reproducción y revenue share; exponer endpoints de estadísticas.  
- Search + discovery (Elastic/Lite) y recomendaciones iniciales.  

### Listen Reward Context
- Integración ZK real (Circom/Arkworks) para `VerifyProof`.  
- Pool dinámico de recompensas con funding desde Payment context.  
- Anti-gaming (detección de bots, limits) y dashboards de streaks.  

### Fractional Ownership / Fan Ventures
- Marketplace secundario para shares (order book simple + fees).  
- Price discovery + oráculos (TWAP).  
- Automatizar distribución de ingresos y notificaciones a inversionistas.  

### Campaign Context
- Integrar NFT marketplace (OpenSea/Reservoir) y on-chain events.  
- Analytics (ROI, CPL, engagement) y social/referral endpoints.  
- Conectar pagos y listen reward para campañas patrocinadas.  

### Fan Loyalty Context
- Completar handlers TDD (`api_handlers.rs`) y wiring del container real.  
- Persistencia completa (verificación biométrica, wristbands, QR) + pruebas contractuales.  
- Integrar servicios externos reales (biométricos, mint NFTs, QR signer).  

### Notifications Context
- Arreglar repositorio Postgres (campos duplicados, metadata/updated_at) y crear migraciones.  
- Añadir canales (email, push, in-app) con colas, plantillas y preferencias de usuario.  
- WebSocket/WebPush gateway y deduplicación de mensajes.

---

## 4. Cross-Cutting & Plataforma

- **Eventing & Sagas**: implementar bus (Redis Streams/Kafka) + `Outbox` por contexto, definir sagas para pagos, rewards, campañas.  
- **Observabilidad**: tracing distribuido (`tracing-opentelemetry` + OTEL collector), métricas Prometheus/Grafana, alertas básicas.  
- **Health & Readiness**: endpoints uniformes con chequeos DB/Redis/event bus.  
- **Config/Secrets**: mover a `config` crate (per env) + `vault`/`aws ssm`.  
- **Security**: HTTPS terminación, CORS estricto, seguridad Axum (rate limiting, content security), secret rotation.  
- **Data & Migrations**: completar `sqlx::migrate`, versionar migraciones por contexto, scripts de seed para QA/demo.  
- **CI/CD**: workflows para lint/test/clippy, coverage y despliegue Docker (multi-stage).  

---

## 5. Testing & Calidad

| Capa | Próximos pasos |
|------|---------------|
| Unit | Cobertura para servicios de dominio (user, campaign, loyalty). |
| Integration | `sqlx::test` + contenedores para repositorios, endpoints clave. |
| Contract/API | Schema tests vs OpenAPI generada; incluir Postman/newman o Dredd. |
| E2E | Scenarios completos (registro → campaña → pago → recompensa). |
| Performance | Benchmarks para streaming/payment, pruebas de carga con k6/Vegeta. |
| Security | Tests OWASP, fuzzing de endpoints críticos, auditoría de permisos. |

---

## 6. Integración Front Angular (Global Styles Strategy)

1. **Design Tokens Globales**  
   - `styles.scss`: reset + variables CSS (`--color-primary`, spacing, tipografía).  
   - `global-theme.scss`: mixins Angular Material y theming (light/dark).  
2. **Servicios API Tipados**  
   - Generar clientes desde OpenAPI; mapear `apiEndpoints` por gateway (puertos 3001‑3008) o configurar proxy/BFF.  
   - `AuthInterceptor` que tome access/refresh tokens y gestione renovaciones.  
3. **State Management**  
   - NgRx/Signals para auth, campañas, rewards; cada slice habla con su bounded context.  
4. **Component Library**  
   - Componentes base (cards, stat badges, tables) usando estilos globales; prepara theming para fan loyalty dashboards.  
5. **Feature Modules**  
   - `music`, `campaigns`, `listen-rewards`, `fan-loyalty` con lazy loading y resolvers que llamen a los endpoints asignados.  

---

## 7. Línea de Tiempo Sugerida

1. **Sprint 1 (Sem 0‑2)**: Auth + Payment gateways + OpenAPI + pruebas base.  
2. **Sprint 2 (Sem 3‑4)**: Event bus persistente + User/Campaign refinados + primeras integraciones Angular.  
3. **Sprint 3 (Sem 5‑6)**: Music upload/streaming + Listen Reward ZK + observabilidad.  
4. **Sprint 4 (Sem 7‑8)**: Fractional marketplace + Campaign NFT + front dashboards + hardening seguridad.  

---

## 8. Checklist de Entrega

- [ ] Autenticación/Autorización completa y documentada.  
- [ ] Gateways de pago con webhooks + pruebas E2E.  
- [ ] Event bus durable + outbox + sagas.  
- [ ] Documentación OpenAPI + clientes Angular generados.  
- [ ] Testing (unit + integration + contract) con CI.  
- [ ] Observabilidad básica (metrics + tracing + health).  
- [ ] Infra segura (HTTPS, secrets, migrations versionadas).  
- [ ] Estrategia de estilos globales implementada en front.  

---

> Mantén este archivo vivo: actualiza al cierre de cada sprint para reflejar avance real y reordenar prioridades según bloqueos o descubrimientos.

