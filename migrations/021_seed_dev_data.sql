-- Migration: 021_seed_dev_data.sql
-- Description: Datos de prueba para desarrollo
-- Author: AI Assistant
-- Created: Diciembre 2024
-- 
-- NOTA: Este script debe ejecutarse solo en entornos de desarrollo
-- NO ejecutar en producción

-- =====================================================
-- USUARIOS DE PRUEBA
-- =====================================================

-- Usuario de prueba 1: Usuario regular
INSERT INTO users (id, email, username, password_hash, display_name, bio, tier, role, is_verified, is_active, created_at, updated_at)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'user1@vibestream.test',
    'testuser1',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyY5Y5Y5Y5Y5', -- password: testpass123
    'Usuario de Prueba 1',
    'Usuario de prueba para desarrollo',
    'free',
    'user',
    false,
    true,
    NOW() - INTERVAL '30 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Usuario de prueba 2: Artista
INSERT INTO users (id, email, username, password_hash, display_name, bio, tier, role, is_verified, is_active, created_at, updated_at)
VALUES (
    '00000000-0000-0000-0000-000000000002',
    'artist1@vibestream.test',
    'testartist1',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyY5Y5Y5Y5Y5', -- password: testpass123
    'Artista de Prueba',
    'Artista de prueba para desarrollo',
    'premium',
    'artist',
    true,
    true,
    NOW() - INTERVAL '60 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Usuario de prueba 3: Admin
INSERT INTO users (id, email, username, password_hash, display_name, bio, tier, role, is_verified, is_active, created_at, updated_at)
VALUES (
    '00000000-0000-0000-0000-000000000003',
    'admin@vibestream.test',
    'admin',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyY5Y5Y5Y5Y5', -- password: testpass123
    'Administrador',
    'Usuario administrador de prueba',
    'vip',
    'admin',
    true,
    true,
    NOW() - INTERVAL '90 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- =====================================================
-- RELACIONES DE SEGUIMIENTO
-- =====================================================

-- Usuario 1 sigue a Artista 1
INSERT INTO user_followers (follower_id, followee_id, created_at)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000002',
    NOW() - INTERVAL '10 days'
) ON CONFLICT (follower_id, followee_id) DO NOTHING;

-- =====================================================
-- ESTADÍSTICAS DE USUARIO
-- =====================================================

-- User tier progress para usuario 1
INSERT INTO user_tier_progress (user_id, current_tier, current_points, points_to_next_tier, tier_since, updated_at)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'free',
    150,
    100,
    NOW() - INTERVAL '30 days',
    NOW()
) ON CONFLICT DO NOTHING;

-- User tier progress para artista 1
INSERT INTO user_tier_progress (user_id, current_tier, current_points, points_to_next_tier, tier_since, updated_at)
VALUES (
    '00000000-0000-0000-0000-000000000002',
    'premium',
    2500,
    5000,
    NOW() - INTERVAL '60 days',
    NOW()
) ON CONFLICT DO NOTHING;

-- =====================================================
-- ARTISTAS DE PRUEBA
-- =====================================================

-- Artista para usuario artista (00000000-0000-0000-0000-000000000002)
INSERT INTO artists (id, user_id, stage_name, bio, verified, created_at, updated_at)
VALUES (
    '10000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000002',
    'Test Artist',
    'Artista de prueba para desarrollo y testing',
    true,
    NOW() - INTERVAL '60 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- =====================================================
-- CANCIONES DE PRUEBA
-- =====================================================

-- Canción 1
INSERT INTO songs (id, title, artist_id, duration_seconds, genre, royalty_percentage, listen_count, revenue_generated, created_at, updated_at)
VALUES (
    '20000000-0000-0000-0000-000000000001',
    'Test Song 1',
    '10000000-0000-0000-0000-000000000001',
    180,
    'Rock',
    10.00,
    150,
    25.50,
    NOW() - INTERVAL '30 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Canción 2
INSERT INTO songs (id, title, artist_id, duration_seconds, genre, royalty_percentage, listen_count, revenue_generated, created_at, updated_at)
VALUES (
    '20000000-0000-0000-0000-000000000002',
    'Test Song 2',
    '10000000-0000-0000-0000-000000000001',
    210,
    'Pop',
    12.50,
    89,
    15.20,
    NOW() - INTERVAL '20 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Canción 3
INSERT INTO songs (id, title, artist_id, duration_seconds, genre, royalty_percentage, listen_count, revenue_generated, created_at, updated_at)
VALUES (
    '20000000-0000-0000-0000-000000000003',
    'Test Song 3',
    '10000000-0000-0000-0000-000000000001',
    195,
    'Jazz',
    15.00,
    45,
    8.75,
    NOW() - INTERVAL '10 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- =====================================================
-- ÁLBUMES DE PRUEBA
-- =====================================================

-- Álbum 1
INSERT INTO albums (id, title, artist_id, genre, is_published, created_at, updated_at)
VALUES (
    '30000000-0000-0000-0000-000000000001',
    'Test Album 1',
    '10000000-0000-0000-0000-000000000001',
    'Rock',
    true,
    NOW() - INTERVAL '30 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Álbum 2
INSERT INTO albums (id, title, artist_id, genre, is_published, created_at, updated_at)
VALUES (
    '30000000-0000-0000-0000-000000000002',
    'Test Album 2',
    '10000000-0000-0000-0000-000000000001',
    'Pop',
    true,
    NOW() - INTERVAL '20 days',
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- =====================================================
-- PLAYLISTS DE PRUEBA
-- =====================================================
-- NOTA: Requiere que la migración 022_update_playlists_and_add_albums.sql se ejecute primero
-- para que existan las columnas name y created_by

-- Playlist 1 (pública)
-- Usar INSERT con ON CONFLICT para manejar si las columnas ya existen o no
INSERT INTO playlists (id, name, description, is_public, song_count, created_by, created_at, updated_at)
SELECT 
    '40000000-0000-0000-0000-000000000001',
    'Test Playlist 1',
    'Playlist de prueba para desarrollo',
    true,
    2,
    '00000000-0000-0000-0000-000000000001',
    NOW() - INTERVAL '15 days',
    NOW()
WHERE NOT EXISTS (SELECT 1 FROM playlists WHERE id = '40000000-0000-0000-0000-000000000001');

-- Si la tabla aún tiene la estructura antigua (title, user_id), actualizar
UPDATE playlists 
SET 
    name = COALESCE(name, 'Test Playlist 1'),
    created_by = COALESCE(created_by, '00000000-0000-0000-0000-000000000001'),
    description = COALESCE(description, 'Playlist de prueba para desarrollo'),
    is_public = COALESCE(is_public, true),
    song_count = COALESCE(song_count, 2)
WHERE id = '40000000-0000-0000-0000-000000000001';

-- Playlist 2 (privada)
INSERT INTO playlists (id, name, description, is_public, song_count, created_by, created_at, updated_at)
SELECT 
    '40000000-0000-0000-0000-000000000002',
    'Test Playlist 2',
    'Playlist privada de prueba',
    false,
    1,
    '00000000-0000-0000-0000-000000000001',
    NOW() - INTERVAL '5 days',
    NOW()
WHERE NOT EXISTS (SELECT 1 FROM playlists WHERE id = '40000000-0000-0000-0000-000000000002');

UPDATE playlists 
SET 
    name = COALESCE(name, 'Test Playlist 2'),
    created_by = COALESCE(created_by, '00000000-0000-0000-0000-000000000001'),
    description = COALESCE(description, 'Playlist privada de prueba'),
    is_public = COALESCE(is_public, false),
    song_count = COALESCE(song_count, 1)
WHERE id = '40000000-0000-0000-0000-000000000002';

-- =====================================================
-- CANCIONES EN PLAYLISTS
-- =====================================================

-- Agregar canciones a playlist 1
INSERT INTO playlist_songs (playlist_id, song_id, position, added_at)
VALUES (
    '40000000-0000-0000-0000-000000000001',
    '20000000-0000-0000-0000-000000000001',
    1,
    NOW() - INTERVAL '15 days'
) ON CONFLICT (playlist_id, song_id) DO NOTHING;

INSERT INTO playlist_songs (playlist_id, song_id, position, added_at)
VALUES (
    '40000000-0000-0000-0000-000000000001',
    '20000000-0000-0000-0000-000000000002',
    2,
    NOW() - INTERVAL '14 days'
) ON CONFLICT (playlist_id, song_id) DO NOTHING;

-- Agregar canción a playlist 2
INSERT INTO playlist_songs (playlist_id, song_id, position, added_at)
VALUES (
    '40000000-0000-0000-0000-000000000002',
    '20000000-0000-0000-0000-000000000003',
    1,
    NOW() - INTERVAL '5 days'
) ON CONFLICT (playlist_id, song_id) DO NOTHING;

-- =====================================================
-- SESIONES DE ESCUCHA DE PRUEBA
-- =====================================================

-- Sesión de escucha 1
INSERT INTO listen_sessions (
    id, user_id, song_id, artist_id, user_tier, status, 
    listen_duration_seconds, quality_score, 
    base_reward_tokens, final_reward_tokens,
    started_at, completed_at, verified_at, created_at, updated_at
)
VALUES (
    '50000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001',
    '20000000-0000-0000-0000-000000000001',
    '10000000-0000-0000-0000-000000000001',
    'free',
    'completed',
    180,
    0.95,
    1.5,
    1.5,
    NOW() - INTERVAL '5 days',
    NOW() - INTERVAL '5 days' + INTERVAL '180 seconds',
    NOW() - INTERVAL '5 days' + INTERVAL '180 seconds',
    NOW() - INTERVAL '5 days',
    NOW() - INTERVAL '5 days'
) ON CONFLICT (id) DO NOTHING;

-- Sesión de escucha 2
INSERT INTO listen_sessions (
    id, user_id, song_id, artist_id, user_tier, status, 
    listen_duration_seconds, quality_score, 
    base_reward_tokens, final_reward_tokens,
    started_at, completed_at, verified_at, created_at, updated_at
)
VALUES (
    '50000000-0000-0000-0000-000000000002',
    '00000000-0000-0000-0000-000000000001',
    '20000000-0000-0000-0000-000000000002',
    '10000000-0000-0000-0000-000000000001',
    'free',
    'completed',
    150,
    0.88,
    1.2,
    1.2,
    NOW() - INTERVAL '3 days',
    NOW() - INTERVAL '3 days' + INTERVAL '150 seconds',
    NOW() - INTERVAL '3 days' + INTERVAL '150 seconds',
    NOW() - INTERVAL '3 days',
    NOW() - INTERVAL '3 days'
) ON CONFLICT (id) DO NOTHING;

-- =====================================================
-- COMENTARIOS
-- =====================================================

COMMENT ON TABLE users IS 'Usuarios del sistema - incluye datos de prueba para desarrollo';
COMMENT ON TABLE user_followers IS 'Relaciones de seguimiento - incluye datos de prueba';
COMMENT ON TABLE user_tier_progress IS 'Progreso de tier de usuarios - incluye datos de prueba';

-- =====================================================
-- INSTRUCCIONES DE USO
-- =====================================================

-- Para ejecutar este script:
-- psql -U vibestream -d vibestream -f migrations/021_seed_dev_data.sql
--
-- O usando sqlx:
-- sqlx migrate run
--
-- Para limpiar datos de prueba (CUIDADO: solo en desarrollo):
-- DELETE FROM user_followers WHERE follower_id IN (
--     '00000000-0000-0000-0000-000000000001',
--     '00000000-0000-0000-0000-000000000002'
-- );
-- DELETE FROM user_tier_progress WHERE user_id IN (
--     '00000000-0000-0000-0000-000000000001',
--     '00000000-0000-0000-0000-000000000002',
--     '00000000-0000-0000-0000-000000000003'
-- );
-- DELETE FROM users WHERE id IN (
--     '00000000-0000-0000-0000-000000000001',
--     '00000000-0000-0000-0000-000000000002',
--     '00000000-0000-0000-0000-000000000003'
-- );

