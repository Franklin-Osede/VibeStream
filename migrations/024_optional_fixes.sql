-- Migration: 024_optional_fixes.sql
-- Description: Correcciones opcionales identificadas en el análisis
-- Author: AI Assistant
-- Created: Diciembre 2024
--
-- NOTA: Esta migración es OPCIONAL y debe aplicarse solo después de:
-- 1. Verificar que los datos existentes son compatibles
-- 2. Hacer backup de la base de datos
-- 3. Probar en ambiente de desarrollo primero
--
-- Correcciones opcionales:
-- 1. Corregir tipos en nft_wristbands (VARCHAR -> UUID)
-- 2. Agregar validaciones adicionales
-- 3. Mejoras de índices

-- =====================================================
-- 1. CORREGIR NFT_WRISTBANDS.ARTIST_ID (OPCIONAL)
-- =====================================================
-- ADVERTENCIA: Solo aplicar si TODOS los valores actuales son UUIDs válidos
-- 
-- Paso 1: Verificar datos antes de aplicar
-- SELECT artist_id, COUNT(*) 
-- FROM nft_wristbands 
-- WHERE artist_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
-- GROUP BY artist_id;
--
-- Si la query anterior devuelve filas, NO aplicar esta migración
-- Si devuelve 0 filas, todos los valores son UUIDs válidos y puedes continuar

DO $$
DECLARE
    invalid_uuids INTEGER;
BEGIN
    -- Verificar si hay valores que NO son UUIDs válidos
    SELECT COUNT(*) INTO invalid_uuids
    FROM nft_wristbands
    WHERE artist_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$';
    
    IF invalid_uuids > 0 THEN
        RAISE WARNING 'No se puede convertir nft_wristbands.artist_id a UUID: % valores inválidos encontrados', invalid_uuids;
        RAISE NOTICE 'Por favor, corrija los datos manualmente antes de aplicar esta migración';
    ELSE
        -- Verificar que todos los artist_id existen en la tabla artists
        SELECT COUNT(*) INTO invalid_uuids
        FROM nft_wristbands nw
        WHERE NOT EXISTS (
            SELECT 1 FROM artists a 
            WHERE a.id::TEXT = nw.artist_id
        );
        
        IF invalid_uuids > 0 THEN
            RAISE WARNING 'No se puede convertir: % artist_id no existen en la tabla artists', invalid_uuids;
        ELSE
            -- Aplicar conversión
            BEGIN
                ALTER TABLE nft_wristbands 
                    ALTER COLUMN artist_id TYPE UUID USING artist_id::UUID;
                
                -- Agregar foreign key
                ALTER TABLE nft_wristbands 
                    ADD CONSTRAINT fk_nft_wristbands_artist_id 
                    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT;
                
                RAISE NOTICE '✓ nft_wristbands.artist_id convertido a UUID y FK agregada';
            EXCEPTION WHEN OTHERS THEN
                RAISE WARNING 'Error al convertir nft_wristbands.artist_id: %', SQLERRM;
            END;
        END IF;
    END IF;
END $$;

-- =====================================================
-- 2. AGREGAR VALIDACIONES ADICIONALES (OPCIONAL)
-- =====================================================

-- Validar que current_funding <= funding_goal en artist_ventures
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'ck_artist_ventures_funding_limit'
    ) THEN
        ALTER TABLE artist_ventures 
            ADD CONSTRAINT ck_artist_ventures_funding_limit 
            CHECK (current_funding <= funding_goal);
        
        COMMENT ON CONSTRAINT ck_artist_ventures_funding_limit ON artist_ventures IS 
            'Garantiza que el financiamiento actual no exceda la meta';
    END IF;
END $$;

-- Validar que min_investment <= max_investment en venture_tiers
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'ck_venture_tiers_investment_range'
    ) THEN
        ALTER TABLE venture_tiers 
            ADD CONSTRAINT ck_venture_tiers_investment_range 
            CHECK (max_investment IS NULL OR min_investment <= max_investment);
        
        COMMENT ON CONSTRAINT ck_venture_tiers_investment_range ON venture_tiers IS 
            'Garantiza que min_investment <= max_investment';
    END IF;
END $$;

-- Validar que min_investment <= max_investment en fan_preferences
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'ck_fan_preferences_investment_range'
    ) THEN
        ALTER TABLE fan_preferences 
            ADD CONSTRAINT ck_fan_preferences_investment_range 
            CHECK (max_investment >= min_investment);
        
        COMMENT ON CONSTRAINT ck_fan_preferences_investment_range ON fan_preferences IS 
            'Garantiza que min_investment <= max_investment';
    END IF;
END $$;

-- =====================================================
-- 3. MEJORAS DE ÍNDICES (OPCIONAL)
-- =====================================================

-- Índice compuesto para búsquedas frecuentes en artist_ventures
CREATE INDEX IF NOT EXISTS idx_artist_ventures_status_category 
    ON artist_ventures(status, category) 
    WHERE status IN ('active', 'funded');

-- Índice para búsquedas por rango de inversión
CREATE INDEX IF NOT EXISTS idx_venture_tiers_investment_range 
    ON venture_tiers(min_investment, max_investment) 
    WHERE max_investment IS NOT NULL;

-- Índice para búsquedas de inversiones por fan
CREATE INDEX IF NOT EXISTS idx_fan_investments_fan_status 
    ON fan_investments(fan_id, status) 
    WHERE status IN ('confirmed', 'pending');

-- =====================================================
-- 4. VERIFICACIÓN FINAL
-- =====================================================

DO $$
DECLARE
    constraint_count INTEGER;
    index_count INTEGER;
BEGIN
    -- Verificar constraints agregados
    SELECT COUNT(*) INTO constraint_count
    FROM information_schema.table_constraints
    WHERE constraint_name IN (
        'ck_artist_ventures_funding_limit',
        'ck_venture_tiers_investment_range',
        'ck_fan_preferences_investment_range'
    );
    
    RAISE NOTICE '✓ CHECK constraints agregados: % de 3 esperados', constraint_count;
    
    -- Verificar índices agregados
    SELECT COUNT(*) INTO index_count
    FROM pg_indexes
    WHERE indexname IN (
        'idx_artist_ventures_status_category',
        'idx_venture_tiers_investment_range',
        'idx_fan_investments_fan_status'
    );
    
    RAISE NOTICE '✓ Índices agregados: % de 3 esperados', index_count;
    
    RAISE NOTICE 'Migration 024 (opcional) completada!';
END $$;

-- =====================================================
-- NOTAS FINALES
-- =====================================================
--
-- Esta migración es OPCIONAL y debe aplicarse con precaución:
--
-- 1. NFT_WRISTBANDS: 
--    - Solo aplicar si los datos son compatibles
--    - Hacer backup antes de aplicar
--    - Verificar que todos los artist_id existen en artists
--
-- 2. VALIDACIONES:
--    - Los CHECK constraints pueden fallar si hay datos inválidos
--    - Revisar datos antes de aplicar
--
-- 3. ÍNDICES:
--    - Los índices mejoran búsquedas pero ocupan espacio
--    - Monitorear rendimiento después de agregarlos
