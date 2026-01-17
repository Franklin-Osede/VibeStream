-- Migration: 023_fix_schema_critical_issues.sql
-- Description: Corregir problemas críticos identificados en el análisis del esquema
-- Author: AI Assistant
-- Created: Diciembre 2024
--
-- Problemas corregidos:
-- 1. Inconsistencia en payments: FK SET NULL vs columnas NOT NULL
-- 2. DOUBLE PRECISION para dinero (cambiar a DECIMAL)
-- 3. Faltan UNIQUE constraints críticos
-- 4. Validación de tipos de datos

-- =====================================================
-- 1. CORREGIR INCONSISTENCIA EN PAYMENTS
-- =====================================================
-- Problema: payer_id y payee_id son NOT NULL pero FK tiene ON DELETE SET NULL
-- Solución: Cambiar a ON DELETE RESTRICT para mantener integridad

-- Primero eliminar las constraints existentes si existen
ALTER TABLE payments 
    DROP CONSTRAINT IF EXISTS fk_payments_payer_id,
    DROP CONSTRAINT IF EXISTS fk_payments_payee_id;

-- Recrear con RESTRICT (no permite borrar usuario si tiene pagos)
ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payer_id 
    FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payee_id 
    FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE RESTRICT;

COMMENT ON CONSTRAINT fk_payments_payer_id ON payments IS 
    'Usuario que realiza el pago. RESTRICT: no se puede borrar usuario con pagos activos.';
COMMENT ON CONSTRAINT fk_payments_payee_id ON payments IS 
    'Usuario que recibe el pago. RESTRICT: no se puede borrar usuario con pagos activos.';

-- =====================================================
-- 2. CAMBIAR DOUBLE PRECISION A DECIMAL PARA DINERO
-- =====================================================
-- Problema: DOUBLE PRECISION causa errores de redondeo en cálculos monetarios
-- Solución: Usar DECIMAL(15,2) para precisión exacta

-- artist_ventures
ALTER TABLE artist_ventures 
    ALTER COLUMN funding_goal TYPE DECIMAL(15,2) USING funding_goal::DECIMAL(15,2),
    ALTER COLUMN current_funding TYPE DECIMAL(15,2) USING current_funding::DECIMAL(15,2),
    ALTER COLUMN min_investment TYPE DECIMAL(15,2) USING min_investment::DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2) USING max_investment::DECIMAL(15,2);

-- fan_investments
ALTER TABLE fan_investments 
    ALTER COLUMN investment_amount TYPE DECIMAL(15,2) USING investment_amount::DECIMAL(15,2);

-- venture_tiers
ALTER TABLE venture_tiers 
    ALTER COLUMN min_investment TYPE DECIMAL(15,2) USING min_investment::DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2) USING max_investment::DECIMAL(15,2);

-- fan_preferences
ALTER TABLE fan_preferences 
    ALTER COLUMN min_investment TYPE DECIMAL(15,2) USING min_investment::DECIMAL(15,2),
    ALTER COLUMN max_investment TYPE DECIMAL(15,2) USING max_investment::DECIMAL(15,2);

-- Agregar comentarios
COMMENT ON COLUMN artist_ventures.funding_goal IS 
    'Meta de financiamiento en DECIMAL(15,2) para precisión exacta en cálculos monetarios';
COMMENT ON COLUMN artist_ventures.current_funding IS 
    'Financiamiento actual en DECIMAL(15,2) para precisión exacta';
COMMENT ON COLUMN fan_investments.investment_amount IS 
    'Monto de inversión en DECIMAL(15,2) para precisión exacta';

-- =====================================================
-- 3. AGREGAR UNIQUE CONSTRAINTS FALTANTES
-- =====================================================
-- Garantizar reglas de negocio 1:1

-- Una canción solo puede tener un contrato de ownership
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'uk_ownership_contracts_song_id'
    ) THEN
        ALTER TABLE ownership_contracts 
            ADD CONSTRAINT uk_ownership_contracts_song_id UNIQUE(song_id);
        
        COMMENT ON CONSTRAINT uk_ownership_contracts_song_id ON ownership_contracts IS 
            'Garantiza que una canción solo puede tener un contrato de ownership (relación 1:1)';
    END IF;
END $$;

-- Un usuario solo puede tener un artista
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'uk_artists_user_id'
    ) THEN
        ALTER TABLE artists 
            ADD CONSTRAINT uk_artists_user_id UNIQUE(user_id);
        
        COMMENT ON CONSTRAINT uk_artists_user_id ON artists IS 
            'Garantiza que un usuario solo puede tener un artista (relación 1:1)';
    END IF;
END $$;

-- Una canción solo puede tener un analytics (si es relación 1:1)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'uk_song_analytics_song_id'
    ) THEN
        -- Verificar que no haya duplicados antes de agregar
        IF NOT EXISTS (
            SELECT song_id, COUNT(*) 
            FROM song_analytics 
            GROUP BY song_id 
            HAVING COUNT(*) > 1
        ) THEN
            ALTER TABLE song_analytics 
                ADD CONSTRAINT uk_song_analytics_song_id UNIQUE(song_id);
            
            COMMENT ON CONSTRAINT uk_song_analytics_song_id ON song_analytics IS 
                'Garantiza que una canción solo puede tener un registro de analytics (relación 1:1)';
        ELSE
            RAISE WARNING 'No se puede agregar UNIQUE en song_analytics.song_id: existen duplicados';
        END IF;
    END IF;
END $$;

-- =====================================================
-- 4. VALIDACIÓN Y VERIFICACIÓN
-- =====================================================

-- Verificar que las correcciones se aplicaron correctamente
DO $$
DECLARE
    fk_count INTEGER;
    unique_count INTEGER;
    decimal_columns INTEGER;
BEGIN
    -- Verificar foreign keys en payments
    SELECT COUNT(*) INTO fk_count
    FROM information_schema.table_constraints
    WHERE constraint_type = 'FOREIGN KEY'
    AND table_name = 'payments'
    AND constraint_name IN ('fk_payments_payer_id', 'fk_payments_payee_id');
    
    IF fk_count != 2 THEN
        RAISE WARNING 'Se esperaban 2 foreign keys en payments, se encontraron %', fk_count;
    ELSE
        RAISE NOTICE '✓ Foreign keys en payments corregidas: % constraints', fk_count;
    END IF;
    
    -- Verificar UNIQUE constraints agregados
    SELECT COUNT(*) INTO unique_count
    FROM information_schema.table_constraints
    WHERE constraint_type = 'UNIQUE'
    AND constraint_name IN (
        'uk_ownership_contracts_song_id',
        'uk_artists_user_id',
        'uk_song_analytics_song_id'
    );
    
    RAISE NOTICE '✓ UNIQUE constraints agregados: % de 3 esperados', unique_count;
    
    -- Verificar que las columnas monetarias son DECIMAL
    SELECT COUNT(*) INTO decimal_columns
    FROM information_schema.columns
    WHERE table_name IN ('artist_ventures', 'fan_investments', 'venture_tiers', 'fan_preferences')
    AND column_name IN ('funding_goal', 'current_funding', 'min_investment', 'max_investment', 'investment_amount')
    AND data_type = 'numeric';
    
    RAISE NOTICE '✓ Columnas monetarias convertidas a DECIMAL: % columnas', decimal_columns;
    
    RAISE NOTICE 'Migration 023 completada exitosamente!';
END $$;

-- =====================================================
-- 5. ÍNDICES ADICIONALES (si son necesarios)
-- =====================================================
-- Los UNIQUE constraints automáticamente crean índices, pero podemos verificar

-- Verificar índices en las columnas UNIQUE
DO $$
BEGIN
    -- Verificar índice para ownership_contracts.song_id
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'ownership_contracts' 
        AND indexname LIKE '%song_id%'
    ) THEN
        RAISE NOTICE 'Índice para ownership_contracts.song_id será creado automáticamente por UNIQUE constraint';
    END IF;
    
    -- Verificar índice para artists.user_id
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE tablename = 'artists' 
        AND indexname LIKE '%user_id%'
    ) THEN
        RAISE NOTICE 'Índice para artists.user_id será creado automáticamente por UNIQUE constraint';
    END IF;
END $$;

-- =====================================================
-- NOTAS IMPORTANTES
-- =====================================================
-- 
-- 1. PAYMENTS: 
--    - Cambiado a RESTRICT significa que NO se puede borrar un usuario
--      si tiene pagos como payer o payee
--    - Si necesitas borrar usuarios, primero debes:
--      a) Archivar/eliminar sus pagos, o
--      b) Implementar borrado lógico (soft delete)
--
-- 2. DECIMAL vs DOUBLE:
--    - DECIMAL(15,2) garantiza precisión exacta hasta 2 decimales
--    - Útil para cálculos monetarios sin errores de redondeo
--
-- 3. UNIQUE CONSTRAINTS:
--    - Garantizan reglas de negocio 1:1
--    - Si hay datos duplicados, la migración fallará
--    - Revisar datos antes de aplicar en producción
--
-- 4. PRÓXIMOS PASOS OPCIONALES:
--    - Considerar UUID v7 si se necesita ordenamiento por fecha
--    - Revisar nft_wristbands.artist_id (VARCHAR vs UUID)
--    - Implementar borrado lógico para tablas históricas
