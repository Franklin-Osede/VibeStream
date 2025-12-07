# 📋 CONTRATO API VIBESTREAM

> **Versión**: 1.0.0  
> **Fecha**: Diciembre 2024  
> **Estado**: En desarrollo - Pre-Frontend  
> **Base URL**: `http://localhost:3000/api/v1`

---

## 🎯 PROPÓSITO

Este documento define el **contrato estable** entre backend y frontend. Solo los endpoints marcados como **"STABLE"** están listos para consumo del frontend.

---

## 📊 ESTADO DE ENDPOINTS

| Estado | Significado | Acción Frontend |
|--------|-------------|-----------------|
| ✅ **STABLE** | Listo para producción, implementado completamente | Usar en desarrollo |
| ⚠️ **BETA** | Funcional pero puede cambiar, implementado parcialmente | Usar con precaución |
| ❌ **MOCK** | Retorna datos mock/TODO, no implementado | **NO usar** |
| 🚧 **DEPRECATED** | Será removido en futuras versiones | Migrar a alternativa |

---

## 👤 USERS CONTEXT (`/api/v1/users`)

### ✅ STABLE - Autenticación

| Método | Endpoint | Estado | Descripción |
|--------|----------|--------|-------------|
| POST | `/register` | ✅ STABLE | Registrar nuevo usuario |
| POST | `/login` | ✅ STABLE | Autenticación con JWT |
| POST | `/refresh` | ✅ STABLE | Renovar access token |

**Request/Response Examples**:
```json
// POST /api/v1/users/register
{
  "email": "user@example.com",
  "username": "user123",
  "password": "securepass123",
  "confirm_password": "securepass123",
  "display_name": "Usuario Demo",
  "bio": "Amante de la música",
  "terms_accepted": true
}

// Response
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "username": "user123",
    "email": "user@example.com",
    "token": "jwt_access_token",
    "refresh_token": "jwt_refresh_token",
    "expires_in": 3600
  }
}
```

### ⚠️ BETA - Perfil y Social

| Método | Endpoint | Estado | Notas |
|--------|----------|--------|-------|
| GET | `/:user_id` | ⚠️ BETA | Algunos campos mock (tier, role, is_verified) |
| PUT | `/:user_id` | ⚠️ BETA | Funcional pero validación incompleta |
| GET | `/:user_id/followers` | ✅ STABLE | Usa repositorio real |
| GET | `/:user_id/following` | ✅ STABLE | Usa repositorio real |
| POST | `/:user_id/follow` | ⚠️ BETA | Usa UUID mock en lugar de JWT (pendiente fix) |

### ❌ MOCK - Analytics y Admin

| Método | Endpoint | Estado | Notas |
|--------|----------|--------|-------|
| GET | `/:user_id/stats` | ❌ MOCK | Retorna datos mock |
| GET | `/analytics` | ❌ MOCK | Retorna datos mock |
| POST | `/:user_id/change-password` | ❌ MOCK | Retorna éxito pero no cambia contraseña |
| POST | `/:user_id/link-wallet` | ❌ MOCK | Retorna éxito pero no vincula wallet |
| DELETE | `/:user_id` | ❌ MOCK | Retorna éxito pero no elimina usuario |

---

## 🎵 MUSIC CONTEXT (`/api/v1/music`)

### ✅ STABLE - CRUD Básico

**Estado Actualizado (Diciembre 2024)**: El gateway `music_gateway.rs` ahora usa controladores reales conectados a PostgreSQL para los endpoints principales.

| Método | Endpoint | Estado | Notas |
|--------|----------|--------|-------|
| GET | `/songs` | ✅ STABLE | Usa `SongController::get_songs` con PostgreSQL |
| POST | `/songs` | ✅ STABLE | Usa `SongController::create_song` con PostgreSQL |
| GET | `/songs/:id` | ✅ STABLE | Usa `SongController::get_song` con PostgreSQL |
| PUT | `/songs/:id` | ✅ STABLE | Usa `SongController::update_song` con PostgreSQL |
| DELETE | `/songs/:id` | ✅ STABLE | Usa `SongController::delete_song` con PostgreSQL |
| GET | `/albums` | ✅ STABLE | Usa `AlbumController::get_albums` con PostgreSQL |
| POST | `/albums` | ✅ STABLE | Usa `AlbumController::create_album` con PostgreSQL |
| GET | `/albums/:id` | ✅ STABLE | Usa `AlbumController::get_album` con PostgreSQL |
| PUT | `/albums/:id` | ✅ STABLE | Usa `AlbumController::update_album` con PostgreSQL |
| DELETE | `/albums/:id` | ✅ STABLE | Usa `AlbumController::delete_album` con PostgreSQL |
| GET | `/playlists` | ✅ STABLE | Usa `PlaylistController::get_playlists` con PostgreSQL |
| POST | `/playlists` | ✅ STABLE | Usa `PlaylistController::create_playlist` con PostgreSQL |
| GET | `/playlists/:id` | ✅ STABLE | Usa `PlaylistController::get_playlist` con PostgreSQL |
| POST | `/playlists/:id/songs` | ✅ STABLE | Usa `PlaylistController::add_song_to_playlist` con PostgreSQL |
| DELETE | `/playlists/:id/songs/:song_id` | ✅ STABLE | Usa `PlaylistController::remove_song_from_playlist` con PostgreSQL |
| GET | `/artists/:id` | ✅ STABLE | Usa `ArtistController::get_artist` con PostgreSQL |
| GET | `/artists/:id/albums` | ✅ STABLE | Usa `ArtistController::get_artist_albums` con PostgreSQL |

### ❌ MOCK - Discovery, Trending y Analytics

| Método | Endpoint | Estado | Notas |
|--------|----------|--------|-------|
| GET | `/songs/discover` | ❌ MOCK | Retorna `{"message": "TODO"}` - Pendiente implementación |
| GET | `/songs/trending` | ❌ MOCK | Retorna `{"message": "TODO"}` - Pendiente implementación |
| POST | `/songs/:id/like` | ❌ MOCK | No implementado - Pendiente |
| POST | `/songs/:id/unlike` | ❌ MOCK | No implementado - Pendiente |
| GET | `/search` | ❌ MOCK | Búsqueda no implementada |
| GET | `/discover` | ❌ MOCK | Descubrimiento no implementado |
| GET | `/analytics/*` | ❌ MOCK | Analytics no implementado |
| GET | `/admin/*` | ❌ MOCK | Endpoints de admin no implementados |

**Nota**: Los endpoints mock se mantienen temporalmente hasta que se implementen los controladores correspondientes. Los endpoints principales (CRUD) están completamente funcionales.

---

## 💰 PAYMENTS CONTEXT (`/api/v1/payments`)

### ⚠️ BETA - Estructura Completa

**Estado**: Controllers y repositorios implementados, pero integraciones externas (Stripe, PayPal) son mock.

| Método | Endpoint | Estado | Notas |
|--------|----------|--------|-------|
| POST | `/payments` | ⚠️ BETA | Controller real, gateways externos mock |
| POST | `/payments/:id/process` | ⚠️ BETA | Controller real, gateways externos mock |
| POST | `/payments/:id/complete` | ⚠️ BETA | Controller real |
| POST | `/payments/refund` | ⚠️ BETA | Controller real |
| GET | `/payments/:id` | ⚠️ BETA | Controller real |
| GET | `/payments/user/:user_id/history` | ⚠️ BETA | Controller real |
| POST | `/royalties/distribute` | ⚠️ BETA | Controller real |
| GET | `/wallets` | ⚠️ BETA | Controller real |

**Decisión Pendiente**: ¿MVP solo pagos internos o integración real con Stripe?

---

## 🎯 CAMPAIGNS CONTEXT (`/api/v1/campaigns`)

### ❌ MOCK - No Listo

**Estado**: Todos los handlers retornan `{"message": "TODO: Implement with real service"}`

| Método | Endpoint | Estado |
|--------|----------|--------|
| * | `/campaigns/*` | ❌ MOCK |
| * | `/nfts/*` | ❌ MOCK |
| * | `/analytics/*` | ❌ MOCK |

**Acción**: Deshabilitar o feature-flag hasta que esté implementado.

---

## 🎧 LISTEN REWARDS CONTEXT (`/api/v1/listen-rewards`)

### ❌ MOCK - No Listo

**Estado**: Placeholder, no implementado.

| Método | Endpoint | Estado |
|--------|----------|--------|
| * | `/sessions/*` | ❌ MOCK |
| * | `/proofs/*` | ❌ MOCK |
| * | `/rewards/*` | ❌ MOCK |

**Acción**: Deshabilitar o feature-flag hasta que esté implementado.

---

## 💎 FAN VENTURES CONTEXT (`/api/v1/fan-ventures`)

### ❌ MOCK - No Listo

**Estado**: Placeholder, eventos con `unimplemented!`.

| Método | Endpoint | Estado |
|--------|----------|--------|
| * | `/ventures/*` | ❌ MOCK |
| * | `/investments/*` | ❌ MOCK |
| * | `/portfolios/*` | ❌ MOCK |

**Acción**: Deshabilitar o feature-flag hasta que esté implementado.

---

## 🔔 NOTIFICATIONS CONTEXT (`/api/v1/notifications`)

### ❌ MOCK - No Listo

**Estado**: Placeholder, no implementado.

| Método | Endpoint | Estado |
|--------|----------|--------|
| * | `/notifications/*` | ❌ MOCK |
| * | `/push/*` | ❌ MOCK |
| * | `/email/*` | ❌ MOCK |

**Acción**: Deshabilitar o feature-flag hasta que esté implementado.

---

## 🏆 FAN LOYALTY CONTEXT (`/api/v1/fan-loyalty`)

### ✅ STABLE - Tests Completos

**Estado**: Implementado con tests completos, pero no completamente integrado.

| Método | Endpoint | Estado | Notas |
|--------|----------|--------|-------|
| POST | `/verify` | ✅ STABLE | Tests completos |
| POST | `/wristbands` | ✅ STABLE | Tests completos |
| GET | `/wristbands/:id` | ✅ STABLE | Tests completos |

---

## 🔐 AUTENTICACIÓN

### Headers Requeridos

```http
Authorization: Bearer <jwt_access_token>
```

### Flujo de Autenticación

1. **Registro**: `POST /api/v1/users/register`
2. **Login**: `POST /api/v1/users/login` → Recibe `access_token` y `refresh_token`
3. **Usar Token**: Incluir en header `Authorization: Bearer <access_token>`
4. **Renovar Token**: `POST /api/v1/users/refresh` con `refresh_token`

### Endpoints Públicos (No Requieren Auth)

- `POST /api/v1/users/register`
- `POST /api/v1/users/login`
- `GET /health`
- `GET /api/v1/info`

### Endpoints Protegidos (Requieren Auth)

Todos los demás endpoints requieren JWT válido.

---

## 📝 NOTAS IMPORTANTES

### 1. Music Gateway - Problema Crítico

El gateway `music_gateway.rs` tiene handlers mock que retornan `{"message": "TODO"}`. Sin embargo, controllers reales existen en `bounded_contexts/music/presentation/controllers/` y están conectados a Postgres.

**Solución Pendiente**: Reemplazar handlers mock con controllers reales (Fase 5).

### 2. User Context - Datos Mock

Algunos handlers en `user_controller.rs` devuelven campos mock:
- `get_user_profile`: tier, role, is_verified son mock
- `get_user_stats`: Todos los datos son mock
- `follow_user`: Usa UUID mock en lugar de extraer de JWT

**Solución Pendiente**: Completar handlers con datos reales (Fase 4).

### 3. Payments - Integraciones Externas

Los gateways de Stripe, PayPal, Coinbase tienen estructura pero implementación mock.

**Decisión Pendiente**: ¿MVP solo pagos internos o integración real?

---

## 🚀 PRÓXIMOS PASOS

1. **Fase 1** (En progreso): Congelar contrato, deshabilitar rutas mock
2. **Fase 2**: Completar OpenAPI spec
3. **Fase 3**: Autenticación sólida (extraer user_id de JWT en todos los handlers)
4. **Fase 4**: Users listo (eliminar mocks)
5. **Fase 5**: Music funcional (reemplazar handlers mock por controllers reales)
6. **Fase 6**: Payments MVP (decidir alcance)

---

## 📞 CONTACTO

Para preguntas sobre el contrato API:
- Revisar: `ANALISIS_EXHAUSTIVO_FINAL_ESTRATEGIA.md`
- Issues: Crear issue en repositorio con tag `api-contract`

---

**Última Actualización**: Diciembre 2024  
**Próxima Revisión**: Después de Fase 5 (Music funcional)


