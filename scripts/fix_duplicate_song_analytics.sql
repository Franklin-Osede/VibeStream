-- Script: fix_duplicate_song_analytics.sql
-- Description: Eliminar duplicados en song_analytics antes de aplicar UNIQUE constraint
-- Usage: psql -U postgres -d vibestream -f scripts/fix_duplicate_song_analytics.sql
--
-- ADVERTENCIA: Este script elimina duplicados, manteniendo solo el registro más reciente
-- Hacer backup antes de ejecutar

\echo '========================================'
\echo 'CORRECCIÓN DE DUPLICADOS EN song_analytics'
\echo '========================================'
\echo ''

-- Primero mostrar los duplicados
\echo 'Duplicados encontrados:'
SELECT 
    song_id, 
    COUNT(*) as duplicate_count,
    array_agg(id ORDER BY created_at DESC) as ids,
    array_agg(created_at ORDER BY created_at DESC) as fechas
FROM song_analytics 
GROUP BY song_id 
HAVING COUNT(*) > 1
ORDER BY duplicate_count DESC;

\echo ''
\echo 'Presiona Ctrl+C para cancelar, o Enter para continuar...'
\echo ''

-- Eliminar duplicados, manteniendo el más reciente
DO $$
DECLARE
    deleted_count INTEGER;
BEGIN
    -- Eliminar duplicados, manteniendo el registro con id más reciente (o created_at más reciente)
    WITH ranked AS (
        SELECT 
            id,
            song_id,
            ROW_NUMBER() OVER (
                PARTITION BY song_id 
                ORDER BY created_at DESC, id DESC
            ) as rn
        FROM song_analytics
    )
    DELETE FROM song_analytics
    WHERE id IN (
        SELECT id FROM ranked WHERE rn > 1
    );
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RAISE NOTICE 'Eliminados % registros duplicados', deleted_count;
    
    -- Verificar que no quedan duplicados
    IF EXISTS (
        SELECT 1 
        FROM song_analytics 
        GROUP BY song_id 
        HAVING COUNT(*) > 1
    ) THEN
        RAISE WARNING 'Aún quedan duplicados. Revisar manualmente.';
    ELSE
        RAISE NOTICE '✓ No quedan duplicados. Puedes proceder con la migración 023.';
    END IF;
END $$;

\echo ''
\echo 'Verificación final:'
SELECT 
    song_id, 
    COUNT(*) as count
FROM song_analytics 
GROUP BY song_id 
HAVING COUNT(*) > 1;

\echo ''
\echo 'Si la query anterior no devuelve resultados, los duplicados fueron eliminados correctamente.'
\echo ''
