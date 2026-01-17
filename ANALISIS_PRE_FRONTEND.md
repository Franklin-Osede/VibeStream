# 🚨 Análisis Pre-Frontend: Estado del Backend y Contratos

> **Fecha**: Enero 2026
> **Objetivo**: Determinar qué falta antes de iniciar el desarrollo del Frontend.

## 📊 Resumen Ejecutivo

El backend está funcional para un **MVP básico (Users + Music)**, pero **incompleto** para las funcionalidades avanzadas (Pagos, Rewards, Contratos).

- **Backend Core**: ~55% funcional.
- **Contratos**: Existentes pero **NO integrados**.
- **Seguridad**: Básica implementada, falta unificación y hardening.

---

## 1. 🔙 Backend (Lo que falta)

### 🔴 Crítico (Bloqueante para Frontend completo)

- **Payments Integration**: La estructura existe pero los gateways (Stripe, PayPal) son **mocks**. El frontend no podrá procesar pagos reales.
  - _Acción_: Implementar `StripeGateway` real.
- **Listen Rewards**: Todo el módulo es **mock**. No hay lógica real de recompensas por escuchar música.
- **Testing**: Aunque `testcontainers` está configurado, la cobertura funcional es baja. Hay riesgo de bugs al integrar el frontend.

### 🟡 Importante

- **Campañas y Fan Ventures**: Totalmente mocks.
- **Notificaciones**: No implementado.

---

## 2. 📜 Contratos y Blockchain (Estado Real)

Aquí está el mayor _gap_:

- **Smart Contracts**: Existe `ProofOfInnovation.sol` en `contracts/proof/`, y circuitos ZK en `backend/circuits/proof_of_listen.circom`.
- **Integración (CRÍTICO)**: El crate `backend/ethereum-integration` existe pero **TIENE EL CÓDIGO FUENTE VACÍO** (solo `Cargo.toml`).
  - _Consecuencia_: El backend **NO** puede hablar con la blockchain actualmente.
  - _Acción_: Se debe escribir el código de interacción (ethers-rs) en `ethereum-integration`.

---

## 3. 🔌 APIs (Contratos de Interfaz)

- **Documentación (OpenAPI)**: Solo `Users` y `Music` están documentados.
  - _Falta_: Payments, Campaigns, Listen Rewards, Fan Ventures.
  - _Riesgo_: El desarrollador frontend tendrá que "adivinar" los endpoints o leer código Rust.
- **Estado de Endpoints**:
  - `POST /users/*` (Auth): ✅ Estable
  - `GET /music/*` (Songs/Albums): ✅ Estable
  - `GET /music/trending`: ❌ Mock (Devuelve TODO)
  - `POST /payments/*`: ⚠️ Beta (Lógica real, pasarela fake)

---

## 4. 🔒 Seguridad

- **Autenticación**:
  - ✅ JWT implementado.
  - ⚠️ **Inconsistencia**: Algunos módulos usan un middleware antiguo en lugar del `jwt_auth_middleware` unificado. Se debe estandarizar.
- **Validación**:
  - ✅ Crate `validator` instalado.
  - ⚠️ Se debe auditar que **todos** los DTOs de entrada (structs) tengan las etiquetas `#[validate(...)]` apropiadas.
- **Faltantes**:
  - Rate Limiting (Protección contra DDoS/Spam).
  - Circuit Breakers (Resiliencia).
  - Security Headers (Helmet equivalente).

---

## 🎯 Recomendación: Roadmap Pre-Frontend

Antes de empezar "seriamente" con el frontend, te recomiendo cerrar estos puntos para no tener que volver atrás:

1.  **Integración Blockchain**: Implementar `ethereum-integration` (al menos la lectura de contratos).
2.  **Unificar Auth Middleware**: Asegurar que todas las rutas protegidas usen el mismo mecanismo.
3.  **Completar OpenAPI**: Documentar al menos los endpoints "Beta" de pagos para que el frontend pueda maquetar la UI de pagos.
4.  **Decisión de Alcance**: Si el frontend va a incluir "Rewards" o "Pagos" en la primera versión, el backend necesita implementar eso YA. Si es solo un reproductor de música, el estado actual es aceptable.
