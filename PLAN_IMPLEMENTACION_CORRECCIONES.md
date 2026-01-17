# 📋 Plan de Implementación - Correcciones del Esquema

> **Fecha**: Diciembre 2024  
> **Estado**: Migraciones creadas, pendiente de aplicación

---

## 🎯 Resumen

Se han creado **2 migraciones SQL** para corregir los problemas críticos identificados en el análisis del esquema:

1. **`023_fix_schema_critical_issues.sql`** - Correcciones críticas (OBLIGATORIA)
2. **`024_optional_fixes.sql`** - Correcciones opcionales (OPCIONAL)

---

## 📦 Migración 023: Correcciones Críticas

### ✅ Problemas Corregidos

#### 1. **Inconsistencia en `payments`**
- **Problema**: `payer_id` y `payee_id` son NOT NULL pero FK tiene `ON DELETE SET NULL`
- **Solución**: Cambiar a `ON DELETE RESTRICT`
- **Impacto**: No se podrá borrar un usuario si tiene pagos activos (mantiene integridad)

#### 2. **DOUBLE PRECISION → DECIMAL**
- **Tablas afectadas**:
  - `artist_ventures` (funding_goal, current_funding, min_investment, max_investment)
  - `fan_investments` (investment_amount)
  - `venture_tiers` (min_investment, max_investment)
  - `fan_preferences` (min_investment, max_investment)
- **Cambio**: `DOUBLE PRECISION` → `DECIMAL(15,2)`
- **Beneficio**: Precisión exacta en cálculos monetarios (sin errores de redondeo)

#### 3. **UNIQUE Constraints Faltantes**
- `ownership_contracts.song_id` → UNIQUE (1 canción = 1 contrato)
- `artists.user_id` → UNIQUE (1 usuario = 1 artista)
- `song_analytics.song_id` → UNIQUE (1 canción = 1 analytics, si es 1:1)

### 📝 Cómo Aplicar

```bash
# Opción 1: Usando psql
psql -U postgres -d vibestream -f migrations/023_fix_schema_critical_issues.sql

# Opción 2: Usando sqlx migrate (Rust)
sqlx migrate run

# Opción 3: Desde el código de la aplicación
# Ejecutar la migración desde tu sistema de migraciones
```

### ⚠️ Advertencias

1. **Backup obligatorio**: Hacer backup antes de aplicar
2. **Datos duplicados**: Si hay duplicados en `song_analytics.song_id`, la migración fallará
3. **Payments RESTRICT**: No se podrá borrar usuarios con pagos (considerar borrado lógico)

### ✅ Verificación Post-Migración

```sql
-- Verificar foreign keys en payments
SELECT constraint_name, constraint_type 
FROM information_schema.table_constraints 
WHERE table_name = 'payments' 
AND constraint_type = 'FOREIGN KEY';

-- Verificar tipos DECIMAL
SELECT table_name, column_name, data_type 
FROM information_schema.columns 
WHERE table_name IN ('artist_ventures', 'fan_investments', 'venture_tiers', 'fan_preferences')
AND column_name IN ('funding_goal', 'current_funding', 'min_investment', 'max_investment', 'investment_amount')
AND data_type = 'numeric';

-- Verificar UNIQUE constraints
SELECT constraint_name, table_name 
FROM information_schema.table_constraints 
WHERE constraint_name IN (
    'uk_ownership_contracts_song_id',
    'uk_artists_user_id',
    'uk_song_analytics_song_id'
);
```

---

## 📦 Migración 024: Correcciones Opcionales

### ✅ Mejoras Incluidas

#### 1. **Corregir `nft_wristbands.artist_id`** (OPCIONAL)
- **Cambio**: `VARCHAR(255)` → `UUID`
- **Requisito**: Todos los valores actuales deben ser UUIDs válidos
- **Verificación**: La migración verifica automáticamente antes de aplicar

#### 2. **Validaciones Adicionales**
- `artist_ventures`: `current_funding <= funding_goal`
- `venture_tiers`: `min_investment <= max_investment`
- `fan_preferences`: `min_investment <= max_investment`

#### 3. **Índices de Performance**
- `idx_artist_ventures_status_category` (búsquedas por status y categoría)
- `idx_venture_tiers_investment_range` (búsquedas por rango de inversión)
- `idx_fan_investments_fan_status` (búsquedas de inversiones por fan)

### 📝 Cómo Aplicar

```bash
# SOLO después de aplicar la migración 023
# Y SOLO después de verificar datos

# 1. Verificar datos de nft_wristbands
psql -U postgres -d vibestream -c "
SELECT artist_id, COUNT(*) 
FROM nft_wristbands 
WHERE artist_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
GROUP BY artist_id;
"

# 2. Si no hay resultados, aplicar migración
psql -U postgres -d vibestream -f migrations/024_optional_fixes.sql
```

### ⚠️ Advertencias

1. **Datos incompatibles**: Si `nft_wristbands.artist_id` tiene valores no-UUID, la migración fallará
2. **Validaciones**: Los CHECK constraints pueden fallar si hay datos inválidos
3. **Índices**: Ocupan espacio adicional (monitorear después de aplicar)

---

## 📊 Checklist de Aplicación

### Pre-Migración

- [ ] **Backup completo de la base de datos**
- [ ] **Verificar versión de PostgreSQL** (recomendado 12+)
- [ ] **Revisar datos duplicados**:
  ```sql
  -- Verificar song_analytics duplicados
  SELECT song_id, COUNT(*) 
  FROM song_analytics 
  GROUP BY song_id 
  HAVING COUNT(*) > 1;
  ```
- [ ] **Revisar datos de nft_wristbands** (si vas a aplicar 024):
  ```sql
  SELECT artist_id 
  FROM nft_wristbands 
  WHERE artist_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$';
  ```

### Aplicación

- [ ] **Aplicar migración 023** en ambiente de desarrollo
- [ ] **Verificar que no hay errores**
- [ ] **Ejecutar queries de verificación**
- [ ] **Probar funcionalidad crítica** (pagos, inversiones, etc.)
- [ ] **Aplicar en producción** (después de pruebas)
- [ ] **Aplicar migración 024** (opcional, solo si es necesario)

### Post-Migración

- [ ] **Verificar integridad de datos**
- [ ] **Monitorear performance** (especialmente índices nuevos)
- [ ] **Actualizar documentación** si es necesario
- [ ] **Comunicar cambios** al equipo

---

## 🔄 Rollback (Si es Necesario)

### Rollback Migración 023

```sql
-- 1. Eliminar UNIQUE constraints
ALTER TABLE ownership_contracts DROP CONSTRAINT IF EXISTS uk_ownership_contracts_song_id;
ALTER TABLE artists DROP CONSTRAINT IF EXISTS uk_artists_user_id;
ALTER TABLE song_analytics DROP CONSTRAINT IF EXISTS uk_song_analytics_song_id;

-- 2. Revertir DECIMAL a DOUBLE (CUIDADO: puede perder precisión)
ALTER TABLE artist_ventures 
    ALTER COLUMN funding_goal TYPE DOUBLE PRECISION USING funding_goal::DOUBLE PRECISION,
    ALTER COLUMN current_funding TYPE DOUBLE PRECISION USING current_funding::DOUBLE PRECISION,
    ALTER COLUMN min_investment TYPE DOUBLE PRECISION USING min_investment::DOUBLE PRECISION,
    ALTER COLUMN max_investment TYPE DOUBLE PRECISION USING max_investment::DOUBLE PRECISION;

-- Similar para otras tablas...

-- 3. Revertir payments (volver a SET NULL, pero requiere hacer columnas NULL primero)
ALTER TABLE payments 
    ALTER COLUMN payer_id DROP NOT NULL,
    ALTER COLUMN payee_id DROP NOT NULL;

ALTER TABLE payments 
    DROP CONSTRAINT IF EXISTS fk_payments_payer_id,
    DROP CONSTRAINT IF EXISTS fk_payments_payee_id;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payer_id 
    FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payee_id 
    FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE SET NULL;
```

**⚠️ NOTA**: El rollback puede ser complejo. **Mejor hacer backup y restaurar** si es necesario.

---

## 📈 Impacto Esperado

### Beneficios

1. ✅ **Integridad referencial mejorada**: FKs correctas en payments
2. ✅ **Precisión monetaria**: Sin errores de redondeo en cálculos
3. ✅ **Reglas de negocio garantizadas**: UNIQUE constraints aseguran 1:1
4. ✅ **Mejor performance**: Índices optimizados (migración 024)

### Riesgos

1. ⚠️ **Payments RESTRICT**: Puede bloquear borrado de usuarios (necesita borrado lógico)
2. ⚠️ **Datos duplicados**: Si existen, la migración fallará
3. ⚠️ **Downtime mínimo**: Migración puede tomar tiempo en tablas grandes

---

## 🆘 Troubleshooting

### Error: "duplicate key value violates unique constraint"

**Causa**: Hay datos duplicados en una columna que se está haciendo UNIQUE

**Solución**:
```sql
-- Encontrar duplicados
SELECT song_id, COUNT(*) 
FROM song_analytics 
GROUP BY song_id 
HAVING COUNT(*) > 1;

-- Eliminar duplicados (mantener el más reciente)
DELETE FROM song_analytics a
USING song_analytics b
WHERE a.id < b.id 
AND a.song_id = b.song_id;
```

### Error: "column contains null values" al agregar NOT NULL

**Causa**: Hay valores NULL en una columna que se está haciendo NOT NULL

**Solución**:
```sql
-- Encontrar NULLs
SELECT COUNT(*) FROM payments WHERE payer_id IS NULL;

-- Actualizar o eliminar registros con NULL
UPDATE payments SET payer_id = ... WHERE payer_id IS NULL;
-- O
DELETE FROM payments WHERE payer_id IS NULL;
```

### Error: "invalid input syntax for type uuid"

**Causa**: Valores en `nft_wristbands.artist_id` no son UUIDs válidos

**Solución**: 
- No aplicar migración 024 para `nft_wristbands`
- O corregir datos manualmente primero

---

## 📞 Soporte

Si encuentras problemas durante la aplicación:

1. **Revisar logs** de PostgreSQL
2. **Verificar datos** con las queries de verificación
3. **Consultar documentación** en `ANALISIS_ESQUEMA_BASE_DATOS.md`
4. **Hacer rollback** si es necesario (usar backup)

---

> **Última actualización**: Diciembre 2024  
> **Próxima revisión**: Después de aplicar migraciones en producción
