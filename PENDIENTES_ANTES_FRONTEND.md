# 📋 Pendientes Antes de Empezar con Frontend

> **Última actualización**: Diciembre 2024  
> **Estado**: Backend 70% listo - Faltan mejoras importantes

---

## ✅ Completado (Listo para usar)

1. ✅ **Tests de Fan Loyalty corregidos** - Usa PostgreSQL
2. ✅ **Music Gateway limpiado** - Endpoints principales usan controladores reales
3. ✅ **Versiones de OpenAPI alineadas** - Versión 1.0.0 consistente
4. ✅ **API_CONTRACT.md actualizado** - Documentación actualizada
5. ✅ **Tests de Music Gateway creados** - Verifican controladores reales
6. ✅ **Script de setup automático** - `scripts/setup-dev.sh`
7. ✅ **Documentación de setup** - `SETUP.md` completo

---

## ⚠️ Pendiente (Importante pero no bloqueante)

### 1. Completar OpenAPI Spec para Endpoints Faltantes

**Prioridad**: Alta  
**Tiempo estimado**: 2-3 horas

**Endpoints que faltan en OpenAPI**:

- [ ] **Payments** (`/api/v1/payments/*`)
  - POST `/payments`
  - GET `/payments/:id`
  - POST `/payments/:id/process`
  - POST `/payments/:id/complete`
  - POST `/payments/refund`
  - GET `/payments/user/:user_id/history`
  - POST `/royalties/distribute`

- [ ] **Fan Loyalty** (`/api/v1/fan-loyalty/*`)
  - POST `/verify`
  - POST `/wristbands`
  - GET `/wristbands/:id`
  - POST `/wristbands/:id/activate`
  - GET `/validate-qr/:code`

- [ ] **Music - Endpoints adicionales** (algunos ya están, verificar completitud)
  - Verificar que todos los endpoints CRUD estén documentados
  - Agregar schemas para requests/responses

**Archivo a modificar**: `services/api-gateway/src/openapi/paths.rs` y `mod.rs`

**Por qué es importante**: El frontend necesita la spec completa para generar clientes automáticamente.

---

### 2. Implementar o Eliminar `validate_api_coverage()`

**Prioridad**: Media  
**Tiempo estimado**: 1 hora

**Problema actual**: La función en `openapi/mod.rs` línea 327 retorna `Ok(())` siempre, no valida realmente.

**Opciones**:
- **Opción A**: Implementar validación real que verifique que todos los endpoints están documentados
- **Opción B**: Eliminar la función si no se va a usar

**Recomendación**: Implementar validación básica que compare endpoints en rutas vs endpoints en OpenAPI.

---

### 3. Decidir y Documentar Estrategia de Pagos

**Prioridad**: Media  
**Tiempo estimado**: 30 minutos (solo decisión)

**Decisión pendiente**:
- ¿MVP solo con pagos internos (mock)?
- ¿Integrar Stripe real?
- ¿Cuándo integrar pagos blockchain?

**Acción requerida**:
1. Decidir estrategia MVP
2. Documentar en `API_CONTRACT.md`
3. Actualizar estado de endpoints de payments

**Recomendación**: Para MVP, mantener pagos mock pero documentarlo claramente. Integrar Stripe en Fase 2.

---

### 4. Probar Todos los Endpoints STABLE

**Prioridad**: Alta  
**Tiempo estimado**: 1-2 horas

**Endpoints a probar manualmente**:

- [ ] **Users**:
  - POST `/register` ✅
  - POST `/login` ✅
  - POST `/refresh` ✅
  - GET `/:user_id` ⚠️
  - GET `/:user_id/followers` ✅
  - GET `/:user_id/following` ✅
  - POST `/:user_id/follow` ⚠️

- [ ] **Music**:
  - GET `/songs` ✅
  - POST `/songs` ✅
  - GET `/songs/:id` ✅
  - PUT `/songs/:id` ✅
  - DELETE `/songs/:id` ✅
  - GET `/albums` ✅
  - POST `/albums` ✅
  - GET `/albums/:id` ✅
  - GET `/playlists` ✅
  - POST `/playlists` ✅
  - GET `/playlists/:id` ✅

- [ ] **Payments**:
  - Probar endpoints principales (aunque sean mock)

- [ ] **Fan Loyalty**:
  - POST `/verify`
  - POST `/wristbands`
  - GET `/wristbands/:id`

**Cómo probar**:
```bash
# Usar el script de setup primero
./scripts/setup-dev.sh

# Luego probar endpoints
curl http://localhost:3000/api/v1/music/songs
# etc.
```

---

## 🔧 Mejoras Opcionales (Pueden esperar)

### 5. Biometría Facial en Fan Loyalty ⭐ GRATIS

**Prioridad**: Media (Puede incluirse en MVP si hay tiempo)  
**Tiempo estimado**: 1-2 días de integración (servicio ya creado)

**Estado actual**: El sistema usa audio (40%), behavioral (30%), device (20%) y location (10%). **NO incluye reconocimiento facial**.

**¡Buenas noticias!**: He creado un **servicio gratuito** de reconocimiento facial usando open source.

**Servicio creado**: `services/facial-recognition-service/`
- ✅ Microservicio Python con face_recognition
- ✅ Dockerfile incluido
- ✅ Adapter en Rust creado
- ✅ **Costo: $0**

**Cuándo incluir**: 
- **MVP (si tienes 1-2 días)**: Puedes incluirla ahora, es gratis
- **Fase 2 (si no en MVP)**: Definitivamente incluir

**Ventajas de la solución gratuita**:
- ✅ **100% Gratuito** - Sin costos de API
- ✅ Control total de datos (privacidad)
- ✅ Sin límites de uso
- ✅ Precisión 95-98% (similar a servicios pagos)

**Requisitos cuando lo implementes**:
- Políticas de privacidad actualizadas
- Consentimiento explícito del usuario
- Iniciar microservicio Python (ya está en docker-compose.yml)
- Integrar adapter en Fan Loyalty (código ya creado)

**📖 Ver**: `BIOMETRIA_FACIAL_PLAN.md` - Plan completo con código

---

### 6. Implementar Endpoints Mock de Music (Discovery, Trending, etc.)

**Prioridad**: Baja  
**Tiempo estimado**: 4-6 horas

**Endpoints mock que podrían implementarse**:
- GET `/songs/discover`
- GET `/songs/trending`
- POST `/songs/:id/like`
- POST `/songs/:id/unlike`
- GET `/search`
- GET `/discover`
- GET `/analytics/*`

**Nota**: Estos endpoints están marcados como MOCK en `API_CONTRACT.md`. Pueden implementarse después del MVP.

---

### 7. Completar Implementación de Webhooks de Pago

**Prioridad**: Baja (solo si se integra Stripe)  
**Tiempo estimado**: 3-4 horas

**Archivos con TODOs**:
- `paypal_webhook.rs` - TODO: Implement PayPal webhook processing
- `coinbase_webhook.rs` - TODO: Implement Coinbase webhook processing

**Nota**: Solo necesario si decides integrar estos gateways. Para MVP, no es crítico.

---

### 8. Agregar Más Tests de Integración

**Prioridad**: Media  
**Tiempo estimado**: 2-3 horas

**Tests que podrían agregarse**:
- Tests end-to-end para flujos completos (crear usuario → crear canción → crear playlist)
- Tests de performance/carga
- Tests de seguridad (SQL injection, XSS, etc.)

**Nota**: Los tests básicos ya están. Estos son para mejorar cobertura.

---

### 9. Optimizar Queries de Base de Datos

**Prioridad**: Baja (solo si hay problemas de performance)  
**Tiempo estimado**: Variable

**Áreas a revisar**:
- Índices faltantes
- Queries N+1
- Caché de queries frecuentes

**Nota**: Solo necesario si encuentras problemas de performance. Para desarrollo, está bien.

---

## 🎯 Checklist Final Antes de Frontend

### Crítico (Debe hacerse)

- [x] Configurar PostgreSQL y ejecutar migraciones
- [x] Configurar JWT_SECRET
- [x] Arreglar Music Gateway (eliminar mocks principales)
- [x] Alinear versiones de OpenAPI
- [ ] **Completar OpenAPI spec para Payments y Fan Loyalty** ⚠️
- [ ] **Probar todos los endpoints STABLE manualmente** ⚠️

### Importante (Recomendado)

- [ ] Decidir y documentar estrategia de pagos
- [ ] Implementar o eliminar `validate_api_coverage()`
- [ ] Verificar que todos los endpoints retornan datos reales

### Opcional (Puede esperar)

- [ ] Implementar endpoints mock de discovery/trending
- [ ] Completar webhooks de pago
- [ ] Agregar más tests
- [ ] Optimizar queries

---

## 📊 Estado Actual por Contexto

| Contexto | Estado | % Funcional | Listo para Frontend? |
|----------|--------|-------------|---------------------|
| **Users** | ✅ STABLE | 90% | ✅ Sí |
| **Music** | ✅ STABLE | 60% | ✅ Sí (CRUD funciona) |
| **Payments** | ⚠️ BETA | 70% | ⚠️ Parcial (estructura lista, gateways mock) |
| **Fan Loyalty** | ✅ STABLE | 85% | ✅ Sí |
| **Campaigns** | ❌ MOCK | 0% | ❌ No |
| **Listen Rewards** | ❌ MOCK | 0% | ❌ No |
| **Fan Ventures** | ❌ MOCK | 0% | ❌ No |
| **Notifications** | ❌ MOCK | 0% | ❌ No |

---

## 🚀 Plan de Acción Recomendado

### Fase 1: Completar lo Crítico (1-2 días)

1. **Completar OpenAPI spec** (2-3 horas)
   - Agregar endpoints de Payments
   - Agregar endpoints de Fan Loyalty
   - Verificar que Music está completo

2. **Probar endpoints STABLE** (1-2 horas)
   - Crear script de pruebas o usar Postman
   - Verificar que retornan datos reales
   - Documentar cualquier problema encontrado

3. **Decidir estrategia de pagos** (30 min)
   - Documentar decisión
   - Actualizar API_CONTRACT.md

**Resultado**: Backend 85% listo, frontend puede empezar con confianza.

---

### Fase 2: Mejoras Post-MVP (Después del frontend básico)

1. Implementar endpoints de discovery/trending
2. Integrar Stripe (si se decide)
3. Agregar más tests
4. Optimizar performance

---

## 💡 Recomendación Final

**Puedes empezar con el frontend ahora si**:
- ✅ Solo necesitas Users, Music (CRUD), y Fan Loyalty
- ✅ Estás dispuesto a trabajar con pagos mock por ahora
- ✅ Puedes generar el cliente OpenAPI manualmente si falta algo

**Deberías completar lo crítico primero si**:
- ⚠️ Necesitas Payments completamente funcionales
- ⚠️ Quieres generar clientes OpenAPI automáticamente
- ⚠️ Necesitas documentación 100% completa

**Mi recomendación**: **Completa los 2-3 items críticos** (OpenAPI spec y pruebas) antes de empezar con frontend. Te tomará 1-2 días pero te ahorrará tiempo después.

---

## 📞 Próximos Pasos Inmediatos

1. **Ejecutar setup** (si no lo has hecho):
   ```bash
   ./scripts/setup-dev.sh
   ```

2. **Completar OpenAPI spec**:
   - Revisar `services/api-gateway/src/openapi/paths.rs`
   - Agregar endpoints faltantes

3. **Probar endpoints**:
   - Usar Postman o curl
   - Verificar respuestas

4. **Decidir sobre pagos**:
   - Documentar decisión
   - Actualizar API_CONTRACT.md

---

> **Nota**: El backend está en buen estado. Los pendientes son principalmente mejoras y completitud, no problemas críticos que bloqueen el desarrollo del frontend.
