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
-- SESIONES DE ESCUCHA DE PRUEBA
-- =====================================================

-- Nota: Estas sesiones requieren que existan canciones en la tabla songs
-- Se pueden crear después de tener canciones de prueba

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

