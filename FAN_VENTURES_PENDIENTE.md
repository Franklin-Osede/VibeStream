# 📋 Fan Ventures - Lo que Falta por Hacer

## 🎯 Resumen Ejecutivo

**Estado Actual**: ✅ Funcional para casos básicos (crear, ver, invertir, portfolio)
**Prioridad**: Mejoras incrementales y funcionalidades avanzadas

---

## 🔴 Prioridad Alta (Funcionalidad Core)

### 1. Métodos del Repositorio Faltantes

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/postgres_repository.rs`

#### Métodos Críticos (Usados por Controllers)

- [ ] **`update_venture()`** - Actualizar venture existente
  - **Estado**: Solo retorna `Ok(())` sin implementar
  - **Uso**: Necesario para editar ventures
  - **Complejidad**: Media

- [ ] **`delete_venture()`** - Eliminar venture
  - **Estado**: Solo retorna `Ok(())` sin implementar
  - **Uso**: Necesario para cancelar ventures
  - **Complejidad**: Baja (soft delete recomendado)

- [ ] **`get_ventures_by_artist()`** - Obtener ventures de un artista
  - **Estado**: Retorna `vec![]` sin implementar
  - **Uso**: Dashboard del artista
  - **Complejidad**: Baja

#### Métodos de Tiers

- [ ] **`update_venture_tier()`** - Actualizar tier
  - **Estado**: Solo retorna `Ok(())` sin implementar
  - **Complejidad**: Media

- [ ] **`delete_venture_tier()`** - Eliminar tier
  - **Estado**: Solo retorna `Ok(())` sin implementar
  - **Complejidad**: Baja

#### Métodos de Benefits

- [ ] **`update_venture_benefit()`** - Actualizar benefit
  - **Estado**: Parcialmente implementado (línea 875)
  - **Complejidad**: Media

- [ ] **`delete_venture_benefit()`** - Eliminar benefit
  - **Estado**: Parcialmente implementado (línea 880)
  - **Complejidad**: Baja

#### Carga de Benefits

- [ ] **Cargar benefits al obtener venture**
  - **Estado**: `benefits: vec![]` hardcodeado (línea 253)
  - **Uso**: Necesario para mostrar beneficios en detalles
  - **Complejidad**: Media

---

### 2. Integración Completa con Pagos

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/presentation/venture_handlers.rs`

**Estado Actual**: 
- ✅ Funding se actualiza directamente
- ⚠️ No hay creación automática de pago

**Para Completar**:

- [ ] **Crear `InitiatePaymentCommand` cuando se invierte**
  - Línea 388: `// TODO: Create payment using PaymentCommandHandler`
  - **Complejidad**: Media-Alta
  - **Requisitos**:
    - Agregar tipo de propósito "VentureInvestment" al sistema de pagos
    - Obtener `PaymentCommandHandler` del AppState
    - Crear comando con metadata del venture

- [ ] **Actualizar funding solo cuando pago se confirma**
  - Actualmente se actualiza inmediatamente
  - **Complejidad**: Alta (requiere event handling)
  - **Requisitos**:
    - Escuchar eventos de pago completado
    - Actualizar `FanInvestment.status` a "Active"
    - Actualizar `venture.current_funding`

- [ ] **Manejar fallos de pago**
  - Revertir inversión si pago falla
  - **Complejidad**: Media

---

### 3. Handlers y Endpoints Faltantes

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/presentation/`

#### Endpoints Básicos Faltantes

- [ ] **`GET /api/v1/fan-ventures`** - Listar ventures
  - **Estado**: Existe en controllers pero no en nuevos handlers
  - **Complejidad**: Baja
  - **Nota**: Usar `list_open_ventures()` del repositorio

- [ ] **`PUT /api/v1/fan-ventures/:id`** - Actualizar venture
  - **Estado**: No existe
  - **Complejidad**: Media
  - **Requisitos**: Implementar `update_venture()` primero

- [ ] **`DELETE /api/v1/fan-ventures/:id`** - Eliminar venture
  - **Estado**: No existe
  - **Complejidad**: Baja
  - **Requisitos**: Implementar `delete_venture()` primero

- [ ] **`GET /api/v1/fan-ventures/artists/:id/ventures`** - Ventures de artista
  - **Estado**: No existe
  - **Complejidad**: Baja
  - **Requisitos**: Implementar `get_ventures_by_artist()` primero

#### Endpoints Avanzados

- [ ] **`POST /api/v1/fan-ventures/:id/activate`** - Activar venture
  - **Estado**: Comentado en controllers (línea 29)
  - **Complejidad**: Baja

- [ ] **`POST /api/v1/fan-ventures/:id/deactivate`** - Desactivar venture
  - **Estado**: Comentado en controllers (línea 30)
  - **Complejidad**: Baja

- [ ] **`POST /api/v1/fan-ventures/:id/distribute-revenue`** - Distribuir revenue
  - **Estado**: No existe
  - **Complejidad**: Alta
  - **Requisitos**: Integración con sistema de pagos

---

## 🟡 Prioridad Media (Mejoras y Optimizaciones)

### 4. Tests de Handlers

**Archivo**: `services/api-gateway/tests/fan_ventures_integration_test.rs`

**Estado Actual**: ✅ Tests de repositorio completos

**Faltantes**:

- [ ] **Tests de handlers HTTP**
  - Test de `create_venture` handler
  - Test de `get_venture_details` handler
  - Test de `invest_in_venture` handler
  - Test de `get_user_portfolio` handler
  - **Complejidad**: Media
  - **Requisitos**: Setup de Axum test client

- [ ] **Tests de validaciones**
  - Test de límites de inversión
  - Test de venture cerrado
  - Test de usuario no autorizado
  - **Complejidad**: Baja

- [ ] **Tests end-to-end**
  - Flujo completo: crear → invertir → verificar funding
  - Auto-activación de venture
  - Auto-cierre cuando alcanza goal
  - **Complejidad**: Media

---

### 5. Analytics y Reporting

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/postgres_repository.rs`

**Métodos Faltantes**:

- [ ] **`get_venture_analytics()`** - Analytics de venture
  - **Estado**: Retorna `json!({})` vacío (línea 922)
  - **Complejidad**: Alta
  - **Datos a incluir**:
    - Total de inversores
    - Revenue acumulado
    - Tasa de conversión
    - Distribución de inversiones por tier

- [ ] **`get_fan_investment_history()`** - Historial de inversiones
  - **Estado**: Retorna `vec![]` (línea 927)
  - **Complejidad**: Baja

- [ ] **`get_top_performing_ventures()`** - Top ventures
  - **Estado**: Retorna `vec![]` (línea 932)
  - **Complejidad**: Media
  - **Criterios**: Por funding, por número de inversores, por ROI

- [ ] **`get_venture_performance_metrics()`** - Métricas de performance
  - **Estado**: Retorna `json!({})` vacío (línea 937)
  - **Complejidad**: Alta

---

### 6. Revenue Distribution

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/postgres_repository.rs`

**Métodos Faltantes**:

- [ ] **`create_revenue_distribution()`** - Crear distribución
  - **Estado**: Retorna `Ok(())` sin implementar (línea 947)
  - **Complejidad**: Alta
  - **Requisitos**: Integración con sistema de pagos

- [ ] **`get_venture_distributions()`** - Obtener distribuciones
  - **Estado**: Retorna `vec![]` (línea 953)
  - **Complejidad**: Media

---

### 7. Benefit Delivery

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/postgres_repository.rs`

**Métodos Faltantes**:

- [ ] **`create_benefit_delivery()`** - Crear entrega de beneficio
  - **Estado**: Retorna `Ok(())` sin implementar (línea 969)
  - **Complejidad**: Media

- [ ] **`get_benefit_delivery()`** - Obtener entrega
  - **Estado**: Retorna `None` (línea 975)
  - **Complejidad**: Baja

- [ ] **`update_benefit_delivery()`** - Actualizar entrega
  - **Estado**: Retorna `Ok(())` sin implementar (línea 981)
  - **Complejidad**: Media

- [ ] **`get_fan_deliveries()`** - Entregas de un fan
  - **Estado**: Retorna `vec![]` (línea 987)
  - **Complejidad**: Baja

- [ ] **`get_venture_deliveries()`** - Entregas de un venture
  - **Estado**: Retorna `vec![]` (línea 993)
  - **Complejidad**: Baja

---

### 8. Recommendations y Preferences

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/postgres_repository.rs`

**Métodos Faltantes**:

- [ ] **`get_venture_recommendations()`** - Recomendaciones para fan
  - **Estado**: Retorna `vec![]` (línea 999)
  - **Complejidad**: Alta
  - **Algoritmo**: Basado en preferencias, categorías, artistas seguidos

- [ ] **`save_fan_preferences()`** - Guardar preferencias
  - **Estado**: Retorna `Ok(())` sin implementar (línea 1005)
  - **Complejidad**: Baja

- [ ] **`get_fan_preferences()`** - Obtener preferencias
  - **Estado**: Retorna `None` (línea 1011)
  - **Complejidad**: Baja

---

## 🟢 Prioridad Baja (Limpieza y Mejoras)

### 9. Limpieza de Código Legacy

**Archivos a Revisar**:

- [ ] **`ownership_routes.rs`** - Eliminar o deprecar
  - **Estado**: Existe pero no se usa
  - **Acción**: Eliminar o marcar como deprecated

- [ ] **`handlers.rs`** - Eliminar handlers antiguos
  - **Estado**: Contiene handlers de "contracts" (legacy)
  - **Acción**: Eliminar o migrar a nuevos handlers

- [ ] **`controllers.rs`** - Actualizar o consolidar
  - **Estado**: Tiene controllers funcionales pero con terminología antigua
  - **Acción**: Decidir si usar controllers o nuevos handlers

- [ ] **Archivos `.backup`** - Eliminar
  - **Estado**: `postgres_repository.rs.backup` existe
  - **Acción**: Eliminar si no se necesita

---

### 10. Integración de Rutas en Gateway

**Archivo**: `services/api-gateway/src/gateways/fan_ventures_gateway.rs`

**Estado Actual**: Usa controllers antiguos

**Acciones**:

- [ ] **Decidir sistema a usar**
  - Opción A: Usar controllers existentes (`controllers.rs`)
  - Opción B: Usar nuevos handlers (`venture_handlers.rs`)
  - Opción C: Consolidar ambos

- [ ] **Integrar nuevas rutas**
  - Si se decide usar nuevos handlers, integrar `venture_routes.rs`
  - Actualizar gateway para usar handlers nuevos

---

### 11. Documentación

**Faltantes**:

- [ ] **Documentar endpoints faltantes en OpenAPI**
  - `GET /api/v1/fan-ventures` (listar)
  - `PUT /api/v1/fan-ventures/:id` (actualizar)
  - `DELETE /api/v1/fan-ventures/:id` (eliminar)
  - `GET /api/v1/fan-ventures/artists/:id/ventures` (ventures de artista)

- [ ] **Actualizar README de Fan Ventures**
  - Documentar flujo completo
  - Ejemplos de uso
  - Guía de integración

---

### 12. Event Handling

**Archivo**: `services/api-gateway/src/bounded_contexts/fan_ventures/infrastructure/event_publisher.rs`

**Estado Actual**: Eventos deshabilitados por falta de tabla `event_outbox`

**Acciones**:

- [ ] **Crear tabla `event_outbox`** (si se necesita)
  - O usar sistema de eventos existente
  - **Complejidad**: Media

- [ ] **Habilitar publicación de eventos**
  - `VentureCreated`
  - `InvestmentMade`
  - `VentureActivated`
  - `VentureFunded`

---

## 📊 Resumen por Prioridad

### 🔴 Prioridad Alta (Crítico para Funcionalidad)

1. ✅ Repositorio básico - **COMPLETADO**
2. ✅ Handlers básicos - **COMPLETADO**
3. ⚠️ Métodos faltantes del repositorio (update, delete, get_by_artist)
4. ⚠️ Integración completa con pagos
5. ⚠️ Endpoints faltantes (listar, actualizar, eliminar)

### 🟡 Prioridad Media (Mejoras Importantes)

6. Tests de handlers
7. Analytics y reporting
8. Revenue distribution
9. Benefit delivery
10. Recommendations

### 🟢 Prioridad Baja (Limpieza)

11. Limpieza de código legacy
12. Integración de rutas
13. Documentación adicional
14. Event handling

---

## 🎯 Plan de Acción Recomendado

### Fase 1: Completar Funcionalidad Core (1-2 semanas)

1. Implementar `update_venture()`, `delete_venture()`, `get_ventures_by_artist()`
2. Agregar endpoints faltantes (listar, actualizar, eliminar)
3. Integrar pagos básicos (crear pago al invertir)

### Fase 2: Mejoras y Optimizaciones (2-3 semanas)

4. Tests de handlers
5. Analytics básicos
6. Benefit delivery básico

### Fase 3: Funcionalidades Avanzadas (3-4 semanas)

7. Revenue distribution completo
8. Recommendations
9. Analytics avanzados

### Fase 4: Limpieza y Documentación (1 semana)

10. Limpieza de código legacy
11. Documentación completa
12. Event handling

---

## 📝 Notas Técnicas

### Métodos del Repositorio por Implementar

**Total**: ~20 métodos
- **Críticos**: 3 (update, delete, get_by_artist)
- **Importantes**: 8 (tiers, benefits, analytics)
- **Avanzados**: 9 (delivery, recommendations, preferences)

### Endpoints por Implementar

**Total**: ~8 endpoints
- **Básicos**: 4 (listar, actualizar, eliminar, por artista)
- **Avanzados**: 4 (activar, distribuir revenue, analytics, etc.)

### Tests por Crear

**Total**: ~15 tests
- **Handlers**: 4
- **Validaciones**: 3
- **End-to-end**: 3
- **Analytics**: 5

---

**Última actualización**: 2024
**Estado**: Funcional para casos básicos, mejoras incrementales pendientes

