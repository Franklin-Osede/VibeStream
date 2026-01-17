# 📊 Análisis Completo del Esquema de Base de Datos - VibeStream

> **Fecha**: Diciembre 2024  
> **Estado**: Análisis exhaustivo de estructura, integridad y mejores prácticas

---

## 🎯 Resumen Ejecutivo

### ✅ Aspectos Positivos
- ✅ Todas las tablas tienen Primary Keys definidas (UUID)
- ✅ Estructura bien normalizada con relaciones claras
- ✅ Constraints de negocio (CHECK) bien definidos
- ✅ Índices en foreign keys y campos de búsqueda frecuente
- ✅ Triggers para actualización automática de timestamps
- ✅ Migración 019 agrega la mayoría de foreign keys faltantes

### ⚠️ Problemas Críticos Encontrados
1. **UUID v7 NO implementado**: Se usa `uuid_generate_v4()` y `gen_random_uuid()` (ambos v4)
2. **Inconsistencia ON DELETE SET NULL vs NOT NULL**: `payments.payer_id` y `payee_id` son NOT NULL pero FK tiene SET NULL
3. **DOUBLE PRECISION para dinero**: `artist_ventures.funding_goal` y `current_funding` usan DOUBLE (riesgo de precisión)
4. **Faltan UNIQUE constraints críticos**: `ownership_contracts.song_id`, `artists.user_id`, etc.
5. **Tipos inconsistentes**: `nft_wristbands.artist_id` es VARCHAR en vez de UUID

---

## 1. 🔑 Evaluación de Primary Keys (PK)

### Estado Actual
✅ **Bien implementado**: Todas las tablas tienen PK UUID

```sql
-- Ejemplo de implementación actual
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- ...
);
```

### Problema: UUID v7 NO está implementado

**Situación actual**:
- Migraciones usan `uuid_generate_v4()` (UUID v4 aleatorio)
- Algunas migraciones usan `gen_random_uuid()` (también v4)
- **NO se usa UUID v7** (ordenable por tiempo)

**Por qué importa UUID v7**:
- ✅ **Orden cronológico**: Los IDs se pueden ordenar por fecha de creación sin campo adicional
- ✅ **Mejor rendimiento en índices**: Los índices B-tree funcionan mejor con datos ordenados
- ✅ **Debugging más fácil**: Puedes ver cuándo se creó un registro solo por el ID
- ✅ **Menos fragmentación**: Los inserts secuenciales mejoran el rendimiento

**Qué cambiar**:
```sql
-- En lugar de:
DEFAULT uuid_generate_v4()

-- Debería ser (requiere extensión):
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- O mejor aún, usar una función personalizada para UUID v7
-- O usar la librería del backend para generar UUID v7
```

**Recomendación**: Si necesitas UUID v7, implementa una función o usa generación en el backend (Rust/Python tienen librerías para UUID v7).

---

## 2. 🔗 Evaluación de Foreign Keys (FK)

### Estado Actual
✅ **Bien**: La migración 019 agrega la mayoría de FKs faltantes  
⚠️ **Problema crítico**: Inconsistencia en `payments`

### Problema Crítico: Inconsistencia ON DELETE SET NULL vs NOT NULL

**Tabla `payments`**:
```sql
-- En la migración 008_payment_tables.sql:
payer_id UUID NOT NULL,  -- ❌ Es NOT NULL
payee_id UUID NOT NULL,  -- ❌ Es NOT NULL

-- Pero en 019_add_missing_foreign_keys.sql:
FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE SET NULL;  -- ❌ Intenta SET NULL
FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE SET NULL;  -- ❌ Intenta SET NULL
```

**Por qué es un problema**:
- Si intentas borrar un usuario, PostgreSQL intentará poner NULL en `payer_id` y `payee_id`
- Pero esas columnas son NOT NULL, entonces **la operación fallará**
- Esto crea una **inconsistencia lógica** en el esquema

**Solución**:
```sql
-- Opción 1: Si quieres mantener historial, hacer las columnas NULL:
ALTER TABLE payments 
    ALTER COLUMN payer_id DROP NOT NULL,
    ALTER COLUMN payee_id DROP NOT NULL;

-- Opción 2: Si NO quieres permitir NULL, usar RESTRICT:
ALTER TABLE payments 
    DROP CONSTRAINT fk_payments_payer_id,
    DROP CONSTRAINT fk_payments_payee_id;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payer_id 
    FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payee_id 
    FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE RESTRICT;
```

### Foreign Keys Faltantes (Verificadas)

Según el documento `ESQUEMA_BASE_DATOS_RELACIONES.md`, estas FKs están marcadas como faltantes pero **la migración 019 las agrega**:

✅ **Ya agregadas en migración 019**:
- `payments` → `users` (payer_id, payee_id, transaction_id)
- `royalty_distributions` → `songs`, `users`
- `revenue_sharing_distributions` → `ownership_contracts`, `songs`
- `shareholder_distributions` → `users`, `payments`
- `payment_batch_items` → `payments`
- `fraud_alerts` → `payments`, `users`
- `ownership_contracts` → `users` (artist_id)
- `share_transactions` → `ownership_contracts`, `users`
- `revenue_distributions` → `ownership_contracts`
- `campaigns` → `songs`, `users`
- `nft_purchases` → `campaigns`, `users`
- `campaign_analytics` → `campaigns`
- `listen_sessions` → `users`, `songs`, `artists`
- `user_reward_history` → `users`, `listen_sessions`, `reward_distributions`
- `notifications` → `users`
- `fan_verifications` → `users`
- `nft_wristbands` → `users`

**Conclusión**: La mayoría de FKs están implementadas. Solo falta corregir la inconsistencia de `payments`.

---

## 3. ✅ Evaluación de Constraints

### Unique Constraints

#### ✅ Bien implementados:
- `users.email` UNIQUE
- `users.username` UNIQUE
- `user_achievements(user_id, achievement_id)` UNIQUE
- `playlist_songs(playlist_id, song_id)` UNIQUE
- `user_shares(user_id, contract_id)` UNIQUE
- `shareholder_distributions(distribution_id, shareholder_id)` UNIQUE
- `payment_batch_items(batch_id, payment_id)` UNIQUE
- `campaign_analytics(campaign_id, date)` UNIQUE
- `fan_preferences(fan_id)` UNIQUE

#### ❌ Faltan UNIQUE críticos:

**1. `ownership_contracts.song_id` debería ser UNIQUE**
```sql
-- Regla de negocio: Una canción solo puede tener UN contrato de ownership
-- Actualmente NO está implementado
ALTER TABLE ownership_contracts 
    ADD CONSTRAINT uk_ownership_contracts_song_id UNIQUE(song_id);
```

**2. `artists.user_id` debería ser UNIQUE**
```sql
-- Regla de negocio: Un usuario solo puede tener UN artista
-- Actualmente NO está implementado (aunque se menciona en el documento)
ALTER TABLE artists 
    ADD CONSTRAINT uk_artists_user_id UNIQUE(user_id);
```

**3. `song_analytics.song_id` debería ser UNIQUE**
```sql
-- Si es relación 1:1, debería ser UNIQUE
ALTER TABLE song_analytics 
    ADD CONSTRAINT uk_song_analytics_song_id UNIQUE(song_id);
```

### Check Constraints

✅ **Bien implementados**:
- `payments.amount_value >= 0`
- `payments.amount_currency IN ('USD', 'ETH', 'SOL', 'USDC', 'VIBES')`
- `payments.status IN ('Pending', 'Processing', ...)`
- `payments.net_amount_value <= amount_value`
- `royalty_distributions.artist_share_percentage >= 0 AND <= 100`
- `listen_sessions.status IN ('active', 'completed', ...)`
- `listen_sessions.quality_score >= 0 AND <= 1`

**Recomendación**: Los CHECK constraints están bien, pero podrían expandirse para validar más reglas de negocio.

---

## 4. 💰 Problema Crítico: Precisión Monetaria

### DOUBLE PRECISION para Dinero

**Problema encontrado en `artist_ventures`**:
```sql
funding_goal DOUBLE PRECISION NOT NULL,  -- ❌ PROBLEMA
current_funding DOUBLE PRECISION DEFAULT 0.0,  -- ❌ PROBLEMA
min_investment DOUBLE PRECISION NOT NULL,  -- ❌ PROBLEMA
max_investment DOUBLE PRECISION,  -- ❌ PROBLEMA
```

**Por qué es un problema**:
- ❌ **Errores de redondeo**: DOUBLE PRECISION usa punto flotante binario, causando errores de precisión
- ❌ **Ejemplo**: `0.1 + 0.2 = 0.30000000000000004` (no exacto)
- ❌ **En dinero**: Puedes perder centavos o tener inconsistencias en cálculos
- ❌ **Comparaciones**: `current_funding = funding_goal` puede fallar por precisión

**Solución**:
```sql
-- Cambiar a DECIMAL con precisión adecuada
ALTER TABLE artist_ventures 
    ALTER COLUMN funding_goal TYPE DECIMAL(15,2),
    ALTER COLUMN current_funding TYPE DECIMAL(15,2),
    ALTER COLUMN min_investment TYPE DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2);
```

**Comparación**:
- ✅ **DECIMAL(15,2)**: Precisión exacta, perfecto para dinero
- ❌ **DOUBLE PRECISION**: Aproximación, puede tener errores

**Nota**: Otras tablas como `payments` ya usan `DECIMAL(15,6)` correctamente.

---

## 5. 🔄 Estrategia de Cascadas (ON DELETE)

### Análisis de Políticas

**CASCADE** (se borra todo lo relacionado):
- ✅ `artists` → `songs` (si borras artista, borras canciones)
- ✅ `users` → `artists` (si borras usuario, borras artista)
- ✅ `playlists` → `playlist_songs` (si borras playlist, borras relaciones)
- ✅ `ownership_contracts` → `user_shares` (si borras contrato, borras shares)

**RESTRICT/NO ACTION** (no permite borrar si hay referencias):
- ✅ `royalty_distributions` → `songs` (histórico, no se borra)
- ✅ `campaigns` → `songs` (histórico, no se borra)
- ✅ `nft_purchases` → `campaigns` (histórico, no se borra)

**SET NULL** (pone NULL si se borra la referencia):
- ⚠️ `payments` → `users` (problema: columnas son NOT NULL)
- ✅ `transactions` → `users` (correcto: user_id puede ser NULL)

**Recomendación**: La estrategia es razonable, pero hay que corregir `payments`.

---

## 6. 📊 Índices

### Estado Actual
✅ **Bien implementado**: Hay índices en:
- Foreign keys
- Campos de búsqueda frecuente (email, username, status, created_at)
- Índices compuestos para queries comunes
- Índices GIN para arrays (tags, favorite_categories)

**Ejemplo de índices bien implementados**:
```sql
CREATE INDEX idx_payments_payer_status_date ON payments(payer_id, status, created_at);
CREATE INDEX idx_songs_artist_id ON songs(artist_id);
CREATE INDEX idx_listen_sessions_user_started ON listen_sessions(user_id, started_at);
```

**Recomendación**: Los índices están bien. Solo asegúrate de que todas las FKs tengan índices (la migración 019 los agrega).

---

## 7. 🔍 Tipos de Datos Inconsistentes

### Problema: VARCHAR vs UUID

**Tabla `nft_wristbands`**:
```sql
artist_id VARCHAR(255) NOT NULL,  -- ❌ Debería ser UUID
concert_id VARCHAR(255) NOT NULL,  -- ⚠️ Podría ser UUID si hay tabla concerts
```

**Por qué es un problema**:
- ❌ No hay integridad referencial (no puede tener FK)
- ❌ No hay validación de que el artista exista
- ❌ Puede tener datos inválidos ("artista_123", "test", etc.)

**Solución**:
```sql
-- Si hay tabla concerts:
ALTER TABLE nft_wristbands 
    ALTER COLUMN artist_id TYPE UUID USING artist_id::UUID,
    ADD CONSTRAINT fk_nft_wristbands_artist_id 
        FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT;

-- Si NO hay tabla concerts, crear una o dejar VARCHAR con validación en app
```

---

## 8. 🎯 Reglas de Negocio vs Implementación

### Relaciones 1:1 que NO tienen UNIQUE

| Tabla | Campo | Regla de Negocio | Implementado? |
|-------|-------|------------------|---------------|
| `artists` | `user_id` | 1 usuario = 1 artista | ❌ Falta UNIQUE |
| `ownership_contracts` | `song_id` | 1 canción = 1 contrato | ❌ Falta UNIQUE |
| `song_analytics` | `song_id` | 1 canción = 1 analytics | ❌ Falta UNIQUE |
| `fan_preferences` | `fan_id` | 1 fan = 1 preferencias | ✅ UNIQUE implementado |

**Recomendación**: Agregar UNIQUE constraints para garantizar las reglas de negocio.

---

## 9. 📝 Otras Cosas Importantes a Evaluar

### 9.1. Borrado Lógico vs Físico

**Estado actual**:
- Algunas tablas usan `status = 'deleted'` (borrado lógico)
- Otras usan DELETE físico con CASCADE

**Recomendación**:
- **Históricos** (pagos, distribuciones, compras): Usar borrado lógico o RESTRICT
- **Contenido** (canciones, playlists): Puede ser DELETE físico con CASCADE
- **Considerar agregar `deleted_at TIMESTAMPTZ`** en tablas críticas

### 9.2. Auditoría y Trazabilidad

**Estado actual**:
- ✅ `created_at` y `updated_at` en la mayoría de tablas
- ✅ Triggers para `updated_at`
- ✅ Tablas de eventos (`payment_events`, `fan_loyalty_events`)
- ⚠️ No hay tabla de audit log general

**Recomendación**:
- Considerar tabla `audit_log` para cambios críticos
- O usar triggers para loggear cambios en tablas sensibles

### 9.3. Validación de Estados

**Estado actual**:
- ✅ CHECK constraints para estados válidos
- ✅ Enums definidos en constraints

**Recomendación**: Mantener consistencia en nombres de estados entre tablas relacionadas.

### 9.4. Escalabilidad

**Tablas que pueden crecer mucho**:
- `listen_sessions` (miles por día)
- `payments` (cientos por día)
- `payment_events` (event sourcing, puede crecer mucho)

**Recomendación**:
- Considerar particionamiento por fecha para tablas grandes
- Índices parciales para datos recientes
- Políticas de retención/archivado

### 9.5. Seguridad y Privacidad

**Datos sensibles**:
- `users.password_hash` ✅ (hash, no plain text)
- `users.wallet_address` ⚠️ (considerar encriptación si es necesario)
- `custodial_wallets.private_key_encrypted` ✅ (encriptado)

**Recomendación**: Revisar políticas de acceso y encriptación según requerimientos legales (GDPR, etc.).

---

## 10. 📋 Checklist de Cambios Prioritarios

### 🔴 Críticos (Hacer primero)

- [ ] **Corregir inconsistencia `payments`**: Cambiar `payer_id`/`payee_id` a NULL o cambiar FK a RESTRICT
- [ ] **Cambiar DOUBLE PRECISION a DECIMAL** en `artist_ventures` (funding_goal, current_funding, etc.)
- [ ] **Agregar UNIQUE constraint** en `ownership_contracts.song_id`
- [ ] **Agregar UNIQUE constraint** en `artists.user_id`
- [ ] **Decidir sobre UUID v7**: Si se necesita, implementar función o usar generación en backend

### 🟡 Importantes (Hacer después)

- [ ] **Corregir tipos** en `nft_wristbands.artist_id` (VARCHAR → UUID si es posible)
- [ ] **Agregar UNIQUE** en `song_analytics.song_id` si es 1:1
- [ ] **Revisar estrategia de borrado lógico** para tablas históricas
- [ ] **Agregar índices** en cualquier FK que falte (verificar migración 019)

### 🟢 Mejoras (Opcional)

- [ ] **Implementar tabla de audit_log** para cambios críticos
- [ ] **Considerar particionamiento** para tablas grandes (listen_sessions, payments)
- [ ] **Revisar políticas de retención** para eventos y logs
- [ ] **Documentar reglas de negocio** en comentarios SQL

---

## 11. 📝 Scripts de Corrección

### Script 1: Corregir payments (Opción RESTRICT)

```sql
-- Migration: Fix payments foreign keys consistency
-- Cambiar a RESTRICT en vez de SET NULL

ALTER TABLE payments 
    DROP CONSTRAINT IF EXISTS fk_payments_payer_id,
    DROP CONSTRAINT IF EXISTS fk_payments_payee_id;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payer_id 
    FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payee_id 
    FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE RESTRICT;
```

### Script 2: Cambiar DOUBLE PRECISION a DECIMAL

```sql
-- Migration: Fix monetary precision in artist_ventures

ALTER TABLE artist_ventures 
    ALTER COLUMN funding_goal TYPE DECIMAL(15,2),
    ALTER COLUMN current_funding TYPE DECIMAL(15,2),
    ALTER COLUMN min_investment TYPE DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2);

-- También en fan_investments
ALTER TABLE fan_investments 
    ALTER COLUMN investment_amount TYPE DECIMAL(15,2);

-- Y en venture_tiers
ALTER TABLE venture_tiers 
    ALTER COLUMN min_investment TYPE DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2);

-- Y en fan_preferences
ALTER TABLE fan_preferences 
    ALTER COLUMN min_investment TYPE DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2);
```

### Script 3: Agregar UNIQUE constraints faltantes

```sql
-- Migration: Add missing UNIQUE constraints

-- Una canción solo puede tener un contrato de ownership
ALTER TABLE ownership_contracts 
    ADD CONSTRAINT uk_ownership_contracts_song_id UNIQUE(song_id);

-- Un usuario solo puede tener un artista
ALTER TABLE artists 
    ADD CONSTRAINT uk_artists_user_id UNIQUE(user_id);

-- Una canción solo puede tener un analytics (si es 1:1)
ALTER TABLE song_analytics 
    ADD CONSTRAINT uk_song_analytics_song_id UNIQUE(song_id);
```

### Script 4: Corregir tipos en nft_wristbands (si es posible)

```sql
-- Migration: Fix nft_wristbands artist_id type
-- SOLO si todos los valores actuales son UUIDs válidos

-- Primero verificar que todos los valores sean UUIDs válidos
-- SELECT artist_id FROM nft_wristbands WHERE artist_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$';

-- Si todos son válidos, convertir:
ALTER TABLE nft_wristbands 
    ALTER COLUMN artist_id TYPE UUID USING artist_id::UUID,
    ADD CONSTRAINT fk_nft_wristbands_artist_id 
        FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT;
```

---

## 12. 📚 Explicación para No Técnicos

### ¿Por qué evaluamos cada cosa?

#### 1. **Primary Keys (PK)**
**Qué es**: El identificador único de cada fila (como el DNI de una persona)  
**Por qué importa**: Sin PK, no puedes identificar de forma única un registro  
**Estado**: ✅ Bien implementado

#### 2. **Foreign Keys (FK)**
**Qué es**: Referencias a otras tablas (como "este pago pertenece a este usuario")  
**Por qué importa**: Garantiza que los datos estén relacionados correctamente  
**Problema**: Si borras un usuario, ¿qué pasa con sus pagos? Las FKs definen esto  
**Estado**: ⚠️ Mayormente bien, pero hay inconsistencia en `payments`

#### 3. **UUID v7**
**Qué es**: Un tipo de ID que incluye la fecha de creación  
**Por qué importa**: Puedes ordenar registros por fecha sin campo adicional  
**Estado**: ❌ No implementado (se usa v4)

#### 4. **Constraints (UNIQUE, CHECK)**
**Qué es**: Reglas que garantizan que los datos sean válidos  
**Ejemplo**: "Un usuario solo puede tener un artista" → necesita UNIQUE  
**Estado**: ⚠️ Faltan algunos UNIQUE importantes

#### 5. **DOUBLE PRECISION vs DECIMAL**
**Qué es**: Formas de almacenar números decimales  
**Problema**: DOUBLE puede tener errores de redondeo (0.1 + 0.2 ≠ 0.3 exactamente)  
**Solución**: DECIMAL es exacto, perfecto para dinero  
**Estado**: ❌ `artist_ventures` usa DOUBLE (debería ser DECIMAL)

#### 6. **Cascadas (ON DELETE)**
**Qué es**: Qué pasa cuando borras un registro relacionado  
**Opciones**:
- **CASCADE**: Borra todo lo relacionado (ej: borrar artista → borrar canciones)
- **RESTRICT**: No permite borrar si hay referencias (ej: no borrar canción si hay pagos)
- **SET NULL**: Pone NULL en la referencia (ej: borrar usuario → poner NULL en user_id)
**Estado**: ✅ Mayormente bien, pero `payments` tiene inconsistencia

---

## 13. 🎯 Conclusión

### Resumen de Calificación

| Aspecto | Calificación | Estado |
|---------|--------------|--------|
| **Primary Keys** | ✅ 9/10 | Bien implementado |
| **Foreign Keys** | ⚠️ 7/10 | Mayormente bien, inconsistencia en payments |
| **Constraints** | ⚠️ 7/10 | Faltan algunos UNIQUE críticos |
| **Tipos de Datos** | ⚠️ 6/10 | DOUBLE para dinero, VARCHAR en vez de UUID |
| **Índices** | ✅ 9/10 | Bien implementado |
| **Estrategia Cascadas** | ✅ 8/10 | Bien pensada, solo corregir payments |
| **UUID v7** | ❌ 0/10 | No implementado |

### Calificación General: **7.5/10**

### Prioridades de Acción

1. **🔴 Urgente**: Corregir inconsistencia `payments` (FK SET NULL vs NOT NULL)
2. **🔴 Urgente**: Cambiar DOUBLE PRECISION a DECIMAL en `artist_ventures`
3. **🟡 Importante**: Agregar UNIQUE constraints faltantes
4. **🟡 Importante**: Decidir sobre UUID v7 (si es necesario)
5. **🟢 Mejora**: Corregir tipos en `nft_wristbands`

---

> **Última actualización**: Diciembre 2024  
> **Próxima revisión**: Después de aplicar correcciones críticas
