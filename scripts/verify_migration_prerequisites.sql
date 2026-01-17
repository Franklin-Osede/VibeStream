-- Script: verify_migration_prerequisites.sql
-- Description: Verificar que los datos están listos para las migraciones 023 y 024
-- Usage: psql -U postgres -d vibestream -f scripts/verify_migration_prerequisites.sql

\echo '========================================'
\echo 'VERIFICACIÓN PRE-MIGRACIÓN'
\echo '========================================'
\echo ''

-- =====================================================
-- 1. VERIFICAR DUPLICADOS EN song_analytics
-- =====================================================
\echo '1. Verificando duplicados en song_analytics.song_id...'
SELECT 
    song_id, 
    COUNT(*) as duplicate_count
FROM song_analytics 
GROUP BY song_id 
HAVING COUNT(*) > 1;

\echo ''
\echo 'Si hay resultados arriba, hay duplicados que deben eliminarse antes de aplicar UNIQUE constraint'
\echo ''

-- =====================================================
-- 2. VERIFICAR DATOS EN payments (para FK RESTRICT)
-- =====================================================
\echo '2. Verificando pagos con usuarios inexistentes...'
SELECT 
    'payer_id' as campo,
    COUNT(*) as registros_con_problema
FROM payments p
WHERE NOT EXISTS (
    SELECT 1 FROM users u WHERE u.id = p.payer_id
)
UNION ALL
SELECT 
    'payee_id' as campo,
    COUNT(*) as registros_con_problema
FROM payments p
WHERE NOT EXISTS (
    SELECT 1 FROM users u WHERE u.id = p.payee_id
);

\echo ''
\echo 'Si hay registros con problema, deben corregirse antes de aplicar FK'
\echo ''

-- =====================================================
-- 3. VERIFICAR TIPOS DE DATOS MONETARIOS
-- =====================================================
\echo '3. Verificando tipos de datos monetarios actuales...'
SELECT 
    table_name,
    column_name,
    data_type,
    numeric_precision,
    numeric_scale
FROM information_schema.columns
WHERE table_name IN ('artist_ventures', 'fan_investments', 'venture_tiers', 'fan_preferences')
AND column_name IN ('funding_goal', 'current_funding', 'min_investment', 'max_investment', 'investment_amount')
ORDER BY table_name, column_name;

\echo ''
\echo 'Verificar que los tipos sean DOUBLE PRECISION (serán convertidos a DECIMAL)'
\echo ''

-- =====================================================
-- 4. VERIFICAR VALORES INVÁLIDOS EN nft_wristbands (para migración 024)
-- =====================================================
\echo '4. Verificando nft_wristbands.artist_id (para migración 024 opcional)...'
SELECT 
    COUNT(*) as total_registros,
    COUNT(*) FILTER (
        WHERE artist_id ~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
    ) as uuid_validos,
    COUNT(*) FILTER (
        WHERE artist_id !~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
    ) as uuid_invalidos
FROM nft_wristbands;

\echo ''
\echo 'Si hay uuid_invalidos > 0, NO aplicar la parte de nft_wristbands en migración 024'
\echo ''

-- Verificar que los artist_id existan en artists
SELECT 
    COUNT(*) as artist_id_inexistentes
FROM nft_wristbands nw
WHERE NOT EXISTS (
    SELECT 1 FROM artists a 
    WHERE a.id::TEXT = nw.artist_id
);

\echo ''
\echo 'Si hay artist_id_inexistentes > 0, corregir antes de aplicar migración 024'
\echo ''

-- =====================================================
-- 5. VERIFICAR CONSTRAINTS EXISTENTES
-- =====================================================
\echo '5. Verificando constraints existentes que serán modificados...'
SELECT 
    constraint_name,
    table_name,
    constraint_type
FROM information_schema.table_constraints
WHERE constraint_name IN (
    'fk_payments_payer_id',
    'fk_payments_payee_id',
    'uk_ownership_contracts_song_id',
    'uk_artists_user_id',
    'uk_song_analytics_song_id'
)
ORDER BY table_name, constraint_name;

\echo ''
\echo 'Estos constraints serán creados o modificados por la migración 023'
\echo ''

-- =====================================================
-- 6. VERIFICAR VALORES INVÁLIDOS PARA CHECK CONSTRAINTS (migración 024)
-- =====================================================
\echo '6. Verificando datos para CHECK constraints (migración 024)...'

-- Verificar current_funding > funding_goal
SELECT 
    COUNT(*) as registros_con_funding_excedido
FROM artist_ventures
WHERE current_funding > funding_goal;

\echo ''
\echo 'Si hay registros con funding_excedido, corregir antes de aplicar CHECK constraint'
\echo ''

-- Verificar min_investment > max_investment en venture_tiers
SELECT 
    COUNT(*) as registros_con_rango_invalido
FROM venture_tiers
WHERE max_investment IS NOT NULL 
AND min_investment > max_investment;

\echo ''
\echo 'Si hay registros con rango_invalido, corregir antes de aplicar CHECK constraint'
\echo ''

-- Verificar min_investment > max_investment en fan_preferences
SELECT 
    COUNT(*) as registros_con_rango_invalido
FROM fan_preferences
WHERE min_investment > max_investment;

\echo ''
\echo 'Si hay registros con rango_invalido, corregir antes de aplicar CHECK constraint'
\echo ''

-- =====================================================
-- RESUMEN
-- =====================================================
\echo '========================================'
\echo 'RESUMEN DE VERIFICACIÓN'
\echo '========================================'
\echo ''
\echo 'Si TODOS los checks están OK (0 registros problemáticos), puedes proceder con:'
\echo '  1. Migración 023 (obligatoria)'
\echo '  2. Migración 024 (opcional, solo si los datos de nft_wristbands son válidos)'
\echo ''
\echo 'Si hay problemas, corregirlos antes de aplicar las migraciones.'
\echo ''
