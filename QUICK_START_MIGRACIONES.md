# 🚀 Quick Start - Aplicar Correcciones del Esquema

> **Guía rápida para aplicar las migraciones 023 y 024**

---

## ⚡ Inicio Rápido (5 minutos)

### Paso 1: Verificar Datos
```bash
psql -U postgres -d vibestream -f scripts/verify_migration_prerequisites.sql
```

**Si hay problemas**: Revisar la salida y corregir datos antes de continuar.

### Paso 2: Aplicar Migraciones (Automático)
```bash
# Modo prueba (sin cambios reales)
./scripts/apply_migrations.sh --dry-run

# Aplicación real
./scripts/apply_migrations.sh
```

**¡Listo!** Las migraciones se aplicarán automáticamente con backup incluido.

---

## 📋 Opciones del Script

```bash
# Aplicar solo migración 023 (obligatoria)
./scripts/apply_migrations.sh --skip-024

# Modo prueba sin cambios
./scripts/apply_migrations.sh --dry-run

# Ver ayuda
./scripts/apply_migrations.sh --help
```

---

## 🔧 Aplicación Manual (Alternativa)

Si prefieres aplicar manualmente:

```bash
# 1. Backup
pg_dump -U postgres -d vibestream > backup_$(date +%Y%m%d).sql

# 2. Verificar datos
psql -U postgres -d vibestream -f scripts/verify_migration_prerequisites.sql

# 3. Aplicar migración 023
psql -U postgres -d vibestream -f migrations/023_fix_schema_critical_issues.sql

# 4. Aplicar migración 024 (opcional)
psql -U postgres -d vibestream -f migrations/024_optional_fixes.sql
```

---

## ✅ Verificación Post-Migración

```sql
-- Verificar que todo está OK
SELECT 
    'Foreign Keys' as tipo,
    COUNT(*) as cantidad
FROM information_schema.table_constraints
WHERE constraint_name IN ('fk_payments_payer_id', 'fk_payments_payee_id')
UNION ALL
SELECT 
    'UNIQUE Constraints',
    COUNT(*)
FROM information_schema.table_constraints
WHERE constraint_name IN (
    'uk_ownership_contracts_song_id',
    'uk_artists_user_id',
    'uk_song_analytics_song_id'
);
```

---

## 🆘 Problemas Comunes

### Error: "duplicate key value violates unique constraint"
**Solución**: Ejecutar `scripts/fix_duplicate_song_analytics.sql` primero

### Error: "column contains null values"
**Solución**: Revisar datos con `scripts/verify_migration_prerequisites.sql` y corregir

### Error: "relation does not exist"
**Solución**: Verificar que todas las migraciones anteriores se aplicaron

---

## 📚 Documentación Completa

- **Análisis completo**: `ANALISIS_ESQUEMA_BASE_DATOS.md`
- **Plan detallado**: `PLAN_IMPLEMENTACION_CORRECCIONES.md`
- **Resumen**: `RESUMEN_IMPLEMENTACION_CORRECCIONES.md`

---

> **¿Listo?** Ejecuta `./scripts/apply_migrations.sh` y sigue las instrucciones en pantalla.
