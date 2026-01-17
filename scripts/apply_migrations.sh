#!/bin/bash

# Script: apply_migrations.sh
# Description: Aplicar migraciones 023 y 024 de forma segura
# Usage: ./scripts/apply_migrations.sh [--dry-run] [--skip-024]

set -e  # Salir si hay errores

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Variables
DRY_RUN=false
SKIP_024=false
DB_NAME="${DATABASE_NAME:-vibestream}"
DB_USER="${DATABASE_USER:-postgres}"
DB_HOST="${DATABASE_HOST:-localhost}"
DB_PORT="${DATABASE_PORT:-5432}"

# Parsear argumentos
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-024)
            SKIP_024=true
            shift
            ;;
        *)
            echo -e "${RED}Opción desconocida: $1${NC}"
            echo "Uso: $0 [--dry-run] [--skip-024]"
            exit 1
            ;;
    esac
done

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}APLICACIÓN DE MIGRACIONES${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Verificar que psql está disponible
if ! command -v psql &> /dev/null; then
    echo -e "${RED}Error: psql no está instalado o no está en PATH${NC}"
    exit 1
fi

# Verificar conexión a la base de datos
echo -e "${YELLOW}Verificando conexión a la base de datos...${NC}"
if ! psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "SELECT 1;" > /dev/null 2>&1; then
    echo -e "${RED}Error: No se puede conectar a la base de datos${NC}"
    echo "Verifica las variables de entorno: DATABASE_NAME, DATABASE_USER, DATABASE_HOST, DATABASE_PORT"
    exit 1
fi
echo -e "${GREEN}✓ Conexión exitosa${NC}"
echo ""

# Paso 1: Verificación previa
echo -e "${YELLOW}Paso 1: Verificando prerrequisitos...${NC}"
if [ "$DRY_RUN" = false ]; then
    psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
        -f scripts/verify_migration_prerequisites.sql
    
    echo ""
    read -p "¿Continuar con la migración? (s/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Ss]$ ]]; then
        echo -e "${YELLOW}Migración cancelada por el usuario${NC}"
        exit 0
    fi
else
    echo -e "${YELLOW}[DRY RUN] Se saltaría la verificación${NC}"
fi
echo ""

# Paso 2: Backup (solo si no es dry-run)
if [ "$DRY_RUN" = false ]; then
    echo -e "${YELLOW}Paso 2: Creando backup...${NC}"
    BACKUP_FILE="backup_pre_migration_$(date +%Y%m%d_%H%M%S).sql"
    pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" > "$BACKUP_FILE"
    echo -e "${GREEN}✓ Backup creado: $BACKUP_FILE${NC}"
    echo ""
else
    echo -e "${YELLOW}[DRY RUN] Se saltaría el backup${NC}"
    echo ""
fi

# Paso 3: Aplicar migración 023
echo -e "${YELLOW}Paso 3: Aplicando migración 023 (correcciones críticas)...${NC}"
if [ "$DRY_RUN" = false ]; then
    if psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
        -f migrations/023_fix_schema_critical_issues.sql; then
        echo -e "${GREEN}✓ Migración 023 aplicada exitosamente${NC}"
    else
        echo -e "${RED}✗ Error al aplicar migración 023${NC}"
        echo -e "${YELLOW}Restaurar desde backup: psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME < $BACKUP_FILE${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}[DRY RUN] Se aplicaría: migrations/023_fix_schema_critical_issues.sql${NC}"
fi
echo ""

# Paso 4: Aplicar migración 024 (opcional)
if [ "$SKIP_024" = false ]; then
    echo -e "${YELLOW}Paso 4: Aplicando migración 024 (correcciones opcionales)...${NC}"
    read -p "¿Aplicar migración 024 (opcional)? (s/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Ss]$ ]]; then
        if [ "$DRY_RUN" = false ]; then
            if psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" \
                -f migrations/024_optional_fixes.sql; then
                echo -e "${GREEN}✓ Migración 024 aplicada exitosamente${NC}"
            else
                echo -e "${YELLOW}⚠ Migración 024 falló (puede ser esperado si hay datos incompatibles)${NC}"
                echo -e "${YELLOW}Revisar los mensajes de advertencia arriba${NC}"
            fi
        else
            echo -e "${YELLOW}[DRY RUN] Se aplicaría: migrations/024_optional_fixes.sql${NC}"
        fi
    else
        echo -e "${YELLOW}Migración 024 omitida${NC}"
    fi
    echo ""
fi

# Paso 5: Verificación post-migración
echo -e "${YELLOW}Paso 5: Verificando migraciones aplicadas...${NC}"
if [ "$DRY_RUN" = false ]; then
    psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" <<EOF
-- Verificar foreign keys en payments
SELECT 
    CASE 
        WHEN COUNT(*) = 2 THEN '✓ Foreign keys en payments: OK'
        ELSE '✗ Foreign keys en payments: FALTA'
    END as status
FROM information_schema.table_constraints
WHERE constraint_type = 'FOREIGN KEY'
AND table_name = 'payments'
AND constraint_name IN ('fk_payments_payer_id', 'fk_payments_payee_id');

-- Verificar UNIQUE constraints
SELECT 
    CASE 
        WHEN COUNT(*) >= 2 THEN '✓ UNIQUE constraints: OK (' || COUNT(*) || ' de 3)'
        ELSE '✗ UNIQUE constraints: FALTAN'
    END as status
FROM information_schema.table_constraints
WHERE constraint_type = 'UNIQUE'
AND constraint_name IN (
    'uk_ownership_contracts_song_id',
    'uk_artists_user_id',
    'uk_song_analytics_song_id'
);

-- Verificar tipos DECIMAL
SELECT 
    CASE 
        WHEN COUNT(*) >= 7 THEN '✓ Tipos DECIMAL: OK (' || COUNT(*) || ' columnas)'
        ELSE '✗ Tipos DECIMAL: FALTAN'
    END as status
FROM information_schema.columns
WHERE table_name IN ('artist_ventures', 'fan_investments', 'venture_tiers', 'fan_preferences')
AND column_name IN ('funding_goal', 'current_funding', 'min_investment', 'max_investment', 'investment_amount')
AND data_type = 'numeric';
EOF
else
    echo -e "${YELLOW}[DRY RUN] Se saltaría la verificación${NC}"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}MIGRACIONES COMPLETADAS${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
if [ "$DRY_RUN" = false ]; then
    echo -e "${GREEN}✓ Proceso completado exitosamente${NC}"
    echo -e "${YELLOW}Backup guardado en: $BACKUP_FILE${NC}"
else
    echo -e "${YELLOW}[DRY RUN] No se aplicaron cambios reales${NC}"
fi
echo ""
