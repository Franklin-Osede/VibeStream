# 🔐 FASE 3: PLAN DE ACCIÓN - Autenticación Sólida

> **Prioridad**: 🔴 CRÍTICA  
> **Tiempo Estimado**: 2-3 días  
> **Dependencias**: Ninguna (puede hacerse en paralelo con otras fases)

---

## 🎯 OBJETIVO

Eliminar datos mock en autenticación y garantizar identidad confiable para todos los handlers protegidos.

---

## 📋 TAREAS DETALLADAS

### Tarea 3.1: JWT_SECRET Obligatorio (30 min)

**Problema Actual**:
```rust
// middleware.rs línea 36-37
let jwt_secret = std::env::var("JWT_SECRET")
    .unwrap_or_else(|_| "default_secret_change_in_production".to_string());
```

**Solución**:
```rust
// Sin fallback inseguro
let jwt_secret = std::env::var("JWT_SECRET")
    .expect("JWT_SECRET must be set in environment variables");
```

**Archivos a Modificar**:
- `services/api-gateway/src/shared/infrastructure/auth/middleware.rs`
- `services/api-gateway/src/shared/infrastructure/auth/jwt_service.rs` (si tiene fallback)
- `services/api-gateway/src/bounded_contexts/user/presentation/controllers/user_controller.rs` (login, refresh)

**Validación**:
- [ ] Compilar sin `JWT_SECRET` debe fallar con mensaje claro
- [ ] Documentar en `.env.example` que es obligatorio

---

### Tarea 3.2: Verificar Extracción de user_id (1 hora)

**Estado Actual**:
- ✅ `follow_user` - Ya usa `AuthenticatedUser` correctamente
- ✅ `change_password` - Ya usa `AuthenticatedUser` correctamente
- ⚠️ Verificar otros handlers protegidos

**Handlers a Verificar**:
1. `get_user_profile` - ¿Extrae user_id para validar permisos?
2. `update_user_profile` - ¿Valida que solo puedes editar tu perfil?
3. `link_wallet` - ¿Usa AuthenticatedUser?
4. `delete_user` - ¿Valida permisos?
5. Handlers de Music (Fase 5)
6. Handlers de Payments (Fase 6)

**Acción**:
- Revisar cada handler protegido
- Asegurar que usan `AuthenticatedUser` extractor
- Validar permisos (solo puedes editar tu propio perfil)

---

### Tarea 3.3: Blacklist de Refresh Tokens (2-3 horas)

**Objetivo**: Invalidar refresh tokens al hacer logout o refresh.

**Implementación**:
```rust
// En Redis
// Key: refresh_token:{token_hash}
// Value: "revoked"
// TTL: tiempo de expiración del refresh token

pub struct RefreshTokenBlacklist {
    redis: redis::Client,
}

impl RefreshTokenBlacklist {
    pub async fn revoke(&self, token: &str) -> Result<(), Error> {
        let hash = sha256(token);
        let key = format!("refresh_token:{}", hash);
        // Guardar en Redis con TTL
    }
    
    pub async fn is_revoked(&self, token: &str) -> Result<bool, Error> {
        let hash = sha256(token);
        let key = format!("refresh_token:{}", hash);
        // Verificar en Redis
    }
}
```

**Integración**:
- En `refresh_token` handler: Verificar blacklist antes de generar nuevo token
- En `logout` handler (si existe): Agregar token a blacklist
- En `jwt_service.validate_refresh_token`: Verificar blacklist

**Archivos a Crear/Modificar**:
- `services/api-gateway/src/shared/infrastructure/auth/refresh_token_blacklist.rs` (nuevo)
- `services/api-gateway/src/shared/infrastructure/auth/jwt_service.rs` (modificar)
- `services/api-gateway/src/bounded_contexts/user/presentation/controllers/user_controller.rs` (refresh_token handler)

---

### Tarea 3.4: Middleware RBAC (2-3 horas)

**Objetivo**: Verificar roles (admin, artist, user) en endpoints protegidos.

**Implementación**:
```rust
pub async fn require_role<const ROLE: &'static str>(
    AuthenticatedUser { role, .. }: AuthenticatedUser,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if role != ROLE {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "message": format!("This endpoint requires {} role", ROLE)
            }))
        ));
    }
    Ok(())
}
```

**Uso**:
```rust
.route("/admin/users", get(get_all_users)
    .layer(middleware::from_fn(jwt_auth_middleware))
    .layer(middleware::from_fn(require_role::<"admin">))
```

**Endpoints que Necesitan RBAC**:
- `/api/v1/users/analytics` - Requiere `admin`
- `/api/v1/music/admin/*` - Requiere `admin`
- `/api/v1/payments/admin/*` - Requiere `admin`

---

### Tarea 3.5: Tests de Autenticación (2-3 horas)

**Tests Unitarios**:
```rust
#[tokio::test]
async fn test_jwt_secret_required() {
    // Sin JWT_SECRET debe fallar
}

#[tokio::test]
async fn test_refresh_token_blacklist() {
    // Token revocado no debe funcionar
}
```

**Tests de Integración**:
```rust
#[tokio::test]
async fn test_protected_endpoint_without_token() {
    // Debe retornar 401
}

#[tokio::test]
async fn test_protected_endpoint_with_invalid_token() {
    // Debe retornar 401
}

#[tokio::test]
async fn test_protected_endpoint_with_valid_token() {
    // Debe funcionar
}

#[tokio::test]
async fn test_refresh_token_rotation() {
    // Refresh debe invalidar token anterior
}
```

**Archivos a Crear**:
- `services/api-gateway/tests/auth/jwt_secret_tests.rs`
- `services/api-gateway/tests/auth/refresh_token_blacklist_tests.rs`
- `services/api-gateway/tests/auth/rbac_tests.rs`
- `services/api-gateway/tests/auth/integration_tests.rs`

---

## 📊 CHECKLIST DE IMPLEMENTACIÓN

### JWT_SECRET Obligatorio
- [ ] Eliminar fallback inseguro en `middleware.rs`
- [ ] Eliminar fallback en `jwt_service.rs` (si existe)
- [ ] Eliminar fallback en `user_controller.rs` (login, refresh)
- [ ] Actualizar `.env.example` con comentario obligatorio
- [ ] Test: Compilar sin JWT_SECRET debe fallar

### Extracción de user_id
- [ ] Verificar `get_user_profile` valida permisos
- [ ] Verificar `update_user_profile` valida permisos
- [ ] Verificar `link_wallet` usa AuthenticatedUser
- [ ] Verificar `delete_user` valida permisos
- [ ] Documentar handlers que ya están correctos

### Blacklist de Refresh Tokens
- [ ] Crear `RefreshTokenBlacklist` struct
- [ ] Implementar `revoke()` método
- [ ] Implementar `is_revoked()` método
- [ ] Integrar en `refresh_token` handler
- [ ] Integrar en `jwt_service.validate_refresh_token()`
- [ ] Tests unitarios de blacklist
- [ ] Tests de integración

### Middleware RBAC
- [ ] Crear `require_role` middleware
- [ ] Aplicar a endpoints admin
- [ ] Tests de RBAC

### Tests
- [ ] Tests unitarios de JWT_SECRET
- [ ] Tests unitarios de blacklist
- [ ] Tests de integración de autenticación
- [ ] Tests de RBAC
- [ ] Cobertura > 80%

---

## 🚀 ORDEN DE EJECUCIÓN RECOMENDADO

1. **Tarea 3.1** (JWT_SECRET) - Más rápido, impacto inmediato
2. **Tarea 3.2** (Verificar user_id) - Revisión rápida
3. **Tarea 3.5** (Tests básicos) - Validar que todo funciona
4. **Tarea 3.3** (Blacklist) - Más complejo, requiere Redis
5. **Tarea 3.4** (RBAC) - Opcional, puede hacerse después

---

## 📝 NOTAS

1. **JWT_SECRET**: Este cambio puede romper desarrollo local si no está configurado. Documentar claramente.

2. **Blacklist**: Requiere Redis funcionando. Considerar fallback graceful si Redis no está disponible (solo en desarrollo).

3. **RBAC**: Puede implementarse incrementalmente, no todos los endpoints necesitan roles de inmediato.

4. **Tests**: Usar testcontainers para Redis en tests de integración.

---

## ✅ CRITERIOS DE ÉXITO

- [ ] JWT_SECRET es obligatorio (sin fallback)
- [ ] Todos los handlers protegidos extraen user_id de JWT
- [ ] Blacklist de refresh tokens implementada
- [ ] Tests de autenticación pasando
- [ ] Documentación actualizada

---

**Siguiente Fase**: Fase 4 (Users) o Fase 5 (Music) - Depende de prioridades


