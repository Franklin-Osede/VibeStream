-- Migration: 022_update_playlists_and_add_albums.sql
-- Description: Actualizar estructura de playlists y agregar tabla albums
-- Created: Diciembre 2024

-- =====================================================
-- ACTUALIZAR ESTRUCTURA DE PLAYLISTS
-- =====================================================

-- Agregar columnas que faltan en playlists
ALTER TABLE playlists 
    ADD COLUMN IF NOT EXISTS name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS created_by UUID REFERENCES users(id) ON DELETE CASCADE,
    ADD COLUMN IF NOT EXISTS song_count INTEGER DEFAULT 0;

-- Migrar datos existentes: title -> name, user_id -> created_by
UPDATE playlists 
SET 
    name = COALESCE(title, 'Untitled Playlist'),
    created_by = COALESCE(user_id, (SELECT id FROM users LIMIT 1))
WHERE name IS NULL OR created_by IS NULL;

-- Hacer name NOT NULL después de migrar
ALTER TABLE playlists 
    ALTER COLUMN name SET NOT NULL,
    ALTER COLUMN created_by SET NOT NULL;

-- Crear índice en created_by
CREATE INDEX IF NOT EXISTS idx_playlists_created_by ON playlists(created_by);

-- =====================================================
-- CREAR TABLA ALBUMS
-- =====================================================

CREATE TABLE IF NOT EXISTS albums (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    artist_id UUID NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    description TEXT,
    genre VARCHAR(100),
    is_published BOOLEAN DEFAULT false,
    release_date TIMESTAMP WITH TIME ZONE,
    song_count INTEGER DEFAULT 0,
    cover_image_url VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Índices para albums
CREATE INDEX IF NOT EXISTS idx_albums_artist_id ON albums(artist_id);
CREATE INDEX IF NOT EXISTS idx_albums_genre ON albums(genre);
CREATE INDEX IF NOT EXISTS idx_albums_is_published ON albums(is_published);

-- Trigger para actualizar updated_at en albums
CREATE TRIGGER update_albums_updated_at 
    BEFORE UPDATE ON albums 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- =====================================================
-- ACTUALIZAR SONGS PARA AGREGAR CAMPOS FALTANTES
-- =====================================================

-- Agregar listen_count si no existe
ALTER TABLE songs 
    ADD COLUMN IF NOT EXISTS listen_count BIGINT DEFAULT 0;

-- Agregar revenue_generated si no existe
ALTER TABLE songs 
    ADD COLUMN IF NOT EXISTS revenue_generated DECIMAL(15,2) DEFAULT 0.0;

-- =====================================================
-- COMENTARIOS
-- =====================================================

COMMENT ON TABLE albums IS 'Álbumes de música creados por artistas';
COMMENT ON TABLE playlists IS 'Playlists de usuarios - estructura actualizada con name y created_by';
COMMENT ON COLUMN playlists.name IS 'Nombre de la playlist (antes title)';
COMMENT ON COLUMN playlists.created_by IS 'Usuario que creó la playlist (antes user_id)';

