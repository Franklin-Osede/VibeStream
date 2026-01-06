# 🚀 Plan de Acción Backend - Próximos Pasos

## 📋 Resumen Ejecutivo

**Decisión**: NO crear más contratos inteligentes. La arquitectura híbrida actual (Web2 para lógica compleja, Web3 para casos específicos) es la correcta.

**Decisión Importante**: NO implementar Fractional Ownership (demasiado complejo, no escalable). En su lugar, enfocarse en **Fan Ventures** (Artist Ventures) que es más simple y escalable.

**Prioridad**: Completar la implementación de Fan Ventures antes de agregar nuevas funcionalidades.

---

## 🎯 Objetivos Principales

1. ✅ **Completar Fan Ventures** - Sistema de inversión de fans en proyectos de artistas
2. ✅ **Mejorar Cliente Blockchain** - Agregar soporte ERC20/ERC721 (opcional)
3. ✅ **Tests de Integración** - Validar flujos end-to-end
4. ✅ **Documentación OpenAPI** - Completar documentación de endpoints

---

## 📅 Fase 1: Fan Ventures (Prioridad Alta)

### ¿Qué es Fan Ventures?

**Fan Ventures** es un sistema donde:
- **Artistas** crean "ventures" (proyectos) con diferentes tiers de inversión
- **Fans** invierten en estos ventures
- **Fans** reciben beneficios según su tier (contenido exclusivo, merch, experiencias, etc.)
- **Opcionalmente** pueden recibir revenue share si el venture lo incluye

**Ventajas sobre Fractional Ownership**:
- ✅ Más simple: No hay trading de shares, no hay marketplace complejo
- ✅ Más escalable: Beneficios claros, no requiere blockchain
- ✅ Más flexible: Artistas pueden crear diferentes tipos de ventures
- ✅ Menos riesgo legal: No implica propiedad fraccionada de IP

### Estado Actual
- ✅ Base de datos: Tablas completas (`artist_ventures`, `fan_investments`, `venture_tiers`, `venture_benefits`)
- ⚠️ Repositorio: Muchos métodos con `// TODO: Implementar cuando la base de datos esté disponible`
- ⚠️ Handlers: Algunos usan placeholders que retornan datos mock
- ❌ OpenAPI: Endpoints no documentados

### Tareas

#### 1.1 Completar Repositorio Postgres
**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/postgres_repository.rs`

Métodos a implementar:
- [ ] `get_ventures_by_category()` - Buscar ventures por categoría
- [ ] `get_ventures_by_status()` - Buscar ventures por estado
- [ ] `search_ventures()` - Búsqueda de ventures
- [ ] `get_fan_investments()` - Obtener inversiones de un fan
- [ ] `get_venture_count()` - Contador de ventures
- [ ] `get_venture_revenue()` - Revenue de un venture
- [ ] `create_revenue_distribution()` - Crear distribución de revenue
- [ ] `get_venture_distributions()` - Obtener distribuciones

**Estimación**: 2-3 días

#### 1.2 Reemplazar Placeholders en Handlers
**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/presentation/handlers.rs`

Handlers a completar:
- [ ] `create_ownership_contract()` → Cambiar a `create_venture()`
- [ ] `purchase_shares()` → Cambiar a `invest_in_venture()`
- [ ] `get_contract_details()` → Cambiar a `get_venture_details()`
- [ ] `get_user_portfolio()` - Implementar con datos reales
- [ ] `distribute_revenue()` - Conectar con sistema de pagos

**Nota**: Los handlers actuales usan terminología de "ownership contracts" y "shares", pero deberían usar "ventures" e "investments".

**Estimación**: 2-3 días

#### 1.3 Documentar en OpenAPI
**Archivo**: `services/api-gateway/src/openapi/paths.rs`

Endpoints a documentar:
- [ ] `POST /api/v1/fan-ventures` - Crear venture
- [ ] `GET /api/v1/fan-ventures` - Listar ventures
- [ ] `GET /api/v1/fan-ventures/:id` - Detalles de venture
- [ ] `POST /api/v1/fan-ventures/:id/invest` - Invertir en venture
- [ ] `GET /api/v1/fan-ventures/users/:id/portfolio` - Portfolio de usuario
- [ ] `POST /api/v1/fan-ventures/:id/distribute-revenue` - Distribuir revenue

**Estimación**: 1 día

#### 1.4 Conectar con Sistema de Pagos
**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/application/`

Integración:
- [ ] Al invertir en venture, crear pago automático
- [ ] Al distribuir revenue, usar sistema de pagos existente
- [ ] Validar que usuario tiene fondos suficientes

**Estimación**: 2 días

#### 1.5 Limpiar Código de Fractional Ownership
**Archivos a revisar**:
- [ ] `domain/aggregates.rs` - Tiene `OwnershipContractAggregate`, debería ser `VentureAggregate`
- [ ] `domain/repository.rs` - Tiene métodos de fractional ownership, debería ser solo ventures
- [ ] `presentation/ownership_routes.rs` - Renombrar a `venture_routes.rs`

**Estimación**: 1 día

**Total Fase 1**: 8-10 días

---

## 📅 Fase 2: Cliente Blockchain (Prioridad Media - Opcional)

### Estado Actual
- ✅ Transferencias nativas (ETH/MATIC) funcionan
- ❌ No soporta ERC20 tokens
- ❌ No soporta ERC721 NFTs

### Tareas

#### 2.1 Agregar Soporte ERC20
**Archivo**: `services/api-gateway/src/shared/infrastructure/clients/blockchain_client.rs`

Métodos a agregar:
- [ ] `transfer_erc20()` - Transferir tokens ERC20
- [ ] `approve_erc20()` - Aprobar gasto de tokens
- [ ] `balance_of_erc20()` - Consultar balance de tokens
- [ ] `allowance_erc20()` - Consultar allowance

**Estimación**: 2 días

#### 2.2 Agregar Soporte ERC721
**Archivo**: `services/api-gateway/src/shared/infrastructure/clients/blockchain_client.rs`

Métodos a agregar:
- [ ] `mint_erc721()` - Mintear NFT
- [ ] `transfer_erc721()` - Transferir NFT
- [ ] `owner_of_erc721()` - Consultar dueño de NFT
- [ ] `balance_of_erc721()` - Balance de NFTs

**Estimación**: 2 días

**Total Fase 2**: 4 días (opcional)

---

## 📅 Fase 3: Tests de Integración (Prioridad Alta)

### Tareas

#### 3.1 Tests de Pagos End-to-End
**Archivo**: `services/api-gateway/tests/integration/payments_test.rs`

Tests a crear:
- [ ] `test_payment_flow_initiate_process_complete()`
- [ ] `test_payment_refund_flow()`
- [ ] `test_payment_with_stripe_gateway()`
- [ ] `test_payment_with_coinbase_gateway()`
- [ ] `test_royalty_distribution()`

**Estimación**: 2 días

#### 3.2 Tests de Fan Ventures
**Archivo**: `services/api-gateway/tests/integration/fan_ventures_test.rs`

Tests a crear:
- [ ] `test_create_venture()`
- [ ] `test_invest_in_venture()`
- [ ] `test_get_venture_details()`
- [ ] `test_get_user_portfolio()`
- [ ] `test_distribute_revenue()`
- [ ] `test_venture_tiers_and_benefits()`

**Estimación**: 2 días

#### 3.3 Tests de Webhooks
**Archivo**: `services/api-gateway/tests/integration/webhooks_test.rs`

Tests a crear:
- [ ] `test_stripe_webhook_processing()`
- [ ] `test_paypal_webhook_processing()`
- [ ] `test_coinbase_webhook_processing()`
- [ ] `test_webhook_reconciliation()`

**Estimación**: 1 día

**Total Fase 3**: 5 días

---

## 📅 Fase 4: Mejoras y Optimizaciones (Prioridad Baja)

### Tareas

#### 4.1 Observabilidad
- [ ] Health checks por bounded context
- [ ] Métricas de performance
- [ ] Logging estructurado

**Estimación**: 2 días

#### 4.2 Validación de Fraude Real
- [ ] Reemplazar `MockFraudDetectionService` con implementación real
- [ ] Integrar con servicio de fraude externo (opcional)

**Estimación**: 3 días

#### 4.3 Notificaciones Reales
- [ ] Reemplazar `MockNotificationService` con implementación real
- [ ] Integrar con servicio de notificaciones (email, push, SMS)

**Estimación**: 2 días

**Total Fase 4**: 7 días (opcional)

---

## 🎯 Priorización Recomendada

### Semana 1-2: Fase 1 (Fan Ventures)
**Por qué**: Es la funcionalidad más visible y necesaria para el frontend.

**Orden de implementación**:
1. Completar repositorio Postgres
2. Reemplazar handlers con lógica real
3. Limpiar código de fractional ownership
4. Documentar en OpenAPI
5. Conectar con sistema de pagos

### Semana 3: Fase 3 (Tests)
**Por qué**: Validar que todo funciona antes de continuar.

### Semana 4: Fase 2 (Blockchain) - Opcional
**Por qué**: Solo si realmente necesitas pagos on-chain.

### Fase 4: Cuando tengas tiempo
**Por qué**: Mejoras incrementales, no bloquean funcionalidad.

---

## 📊 Métricas de Éxito

- [ ] Todos los endpoints de fan ventures funcionan
- [ ] Tests de integración pasan al 100%
- [ ] Documentación OpenAPI completa
- [ ] No hay código de fractional ownership en producción
- [ ] No hay TODOs críticos en código de producción
- [ ] Cliente blockchain soporta ERC20/ERC721 (si es necesario)

---

## 🚨 Diferencias Clave: Fan Ventures vs Fractional Ownership

### Fan Ventures (✅ Implementar)
- **Modelo**: Artista crea proyecto, fans invierten, reciben beneficios
- **Complejidad**: Baja - Sistema de inversión simple
- **Escalabilidad**: Alta - No requiere blockchain
- **Legal**: Bajo riesgo - No implica propiedad de IP
- **Ejemplo**: "Invierte $50 en mi nuevo álbum, recibe acceso exclusivo + merch"

### Fractional Ownership (❌ NO Implementar)
- **Modelo**: Artista vende shares de una canción, fans pueden tradear shares
- **Complejidad**: Alta - Requiere marketplace, trading, pricing dinámico
- **Escalabilidad**: Baja - Muchas transacciones, alto costo de gas
- **Legal**: Alto riesgo - Implica propiedad fraccionada de IP
- **Ejemplo**: "Compra 10% de esta canción, puedes vender tus shares después"

---

## 🧹 Limpieza de Código Necesaria

### Archivos con Código de Fractional Ownership a Limpiar

1. **`domain/aggregates.rs`**
   - Tiene `OwnershipContractAggregate` - Debería eliminarse o renombrarse
   - Tiene `OwnershipContract` - Debería eliminarse

2. **`domain/repository.rs`**
   - Tiene `OwnershipContractRepository` - Debería ser `VentureRepository`
   - Métodos relacionados con shares - Eliminar

3. **`presentation/ownership_routes.rs`**
   - Renombrar a `venture_routes.rs`
   - Cambiar terminología de "contracts" a "ventures"
   - Cambiar terminología de "shares" a "investments"

4. **`presentation/handlers.rs`**
   - `create_ownership_contract()` → `create_venture()`
   - `purchase_shares()` → `invest_in_venture()`
   - `get_contract_details()` → `get_venture_details()`

5. **Base de datos**
   - Las tablas `ownership_contracts`, `user_shares`, `share_transactions` pueden mantenerse para migración futura, pero no usarlas

---

## 📝 Notas

- **No crear más contratos**: La arquitectura híbrida es correcta
- **Enfocarse en Fan Ventures**: Más simple y escalable que Fractional Ownership
- **Limpiar código legacy**: Eliminar referencias a fractional ownership
- **Tests primero**: Escribir tests ayuda a entender los requisitos
- **Documentar mientras desarrollas**: No dejar documentación para el final

---

## ✅ Checklist de Completitud

### Fan Ventures
- [ ] Repositorio Postgres completo
- [ ] Handlers sin placeholders
- [ ] Endpoints documentados en OpenAPI
- [ ] Integrado con sistema de pagos
- [ ] Tests de integración pasando
- [ ] Código de fractional ownership eliminado/renombrado

### Cliente Blockchain
- [ ] Soporte ERC20 implementado (opcional)
- [ ] Soporte ERC721 implementado (opcional)
- [ ] Tests de blockchain client
- [ ] Documentación de uso

### Tests
- [ ] Tests de pagos end-to-end
- [ ] Tests de fan ventures
- [ ] Tests de webhooks
- [ ] Cobertura > 80%

### Documentación
- [ ] OpenAPI completo
- [ ] README actualizado
- [ ] Guías de uso creadas

---

**Última actualización**: 2024
**Estado**: En progreso
**Enfoque**: Fan Ventures (NO Fractional Ownership)
