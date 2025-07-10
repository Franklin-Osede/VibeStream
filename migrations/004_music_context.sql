-- Migration: Music Context Tables
-- Description: Create tables for Music bounded context
-- Version: 004
-- Date: 2024-01-15

-- Enable UUID extension if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Songs table
CREATE TABLE IF NOT EXISTS songs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    artist_id UUID NOT NULL,
    duration_seconds INTEGER NOT NULL CHECK (duration_seconds > 0),
    genre VARCHAR(100) NOT NULL,
    mood VARCHAR(50),
    file_format VARCHAR(20),
    audio_quality VARCHAR(20),
    tempo INTEGER,
    release_type VARCHAR(50),
    royalty_percentage DECIMAL(5,2) NOT NULL CHECK (royalty_percentage >= 0 AND royalty_percentage <= 100),
    ipfs_hash VARCHAR(255),
    listen_count BIGINT DEFAULT 0,
    revenue_generated DECIMAL(15,2) DEFAULT 0.00,
    is_available_for_campaign BOOLEAN DEFAULT true,
    is_available_for_ownership BOOLEAN DEFAULT true,
    is_popular BOOLEAN DEFAULT false,
    is_trending BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Albums table
CREATE TABLE IF NOT EXISTS albums (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    artist_id UUID NOT NULL,
    description TEXT,
    cover_art_ipfs VARCHAR(255),
    release_date DATE,
    total_duration_seconds INTEGER,
    track_count INTEGER DEFAULT 0,
    listen_count BIGINT DEFAULT 0,
    revenue_generated DECIMAL(15,2) DEFAULT 0.00,
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Album tracks (junction table for songs in albums)
CREATE TABLE IF NOT EXISTS album_tracks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    album_id UUID NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    track_number INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(album_id, song_id),
    UNIQUE(album_id, track_number)
);

-- Playlists table
CREATE TABLE IF NOT EXISTS playlists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL,
    description TEXT,
    is_public BOOLEAN DEFAULT true,
    is_collaborative BOOLEAN DEFAULT false,
    cover_image_url VARCHAR(500),
    song_count INTEGER DEFAULT 0,
    total_duration_seconds INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Playlist tracks (junction table for songs in playlists)
CREATE TABLE IF NOT EXISTS playlist_tracks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    playlist_id UUID NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    added_by UUID, -- user who added the song (for collaborative playlists)
    added_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(playlist_id, song_id),
    UNIQUE(playlist_id, position)
);

-- Artists table (extended profile for music creators)
CREATE TABLE IF NOT EXISTS artists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL UNIQUE, -- Reference to users table
    stage_name VARCHAR(255) NOT NULL,
    bio TEXT,
    location VARCHAR(255),
    website VARCHAR(500),
    avatar_ipfs VARCHAR(255),
    banner_ipfs VARCHAR(255),
    debut_year INTEGER,
    record_label VARCHAR(255),
    management_contact VARCHAR(255),
    booking_contact VARCHAR(255),
    primary_genres TEXT[], -- Array of genres
    follower_count BIGINT DEFAULT 0,
    following_count BIGINT DEFAULT 0,
    total_songs INTEGER DEFAULT 0,
    total_albums INTEGER DEFAULT 0,
    total_listens BIGINT DEFAULT 0,
    total_revenue DECIMAL(15,2) DEFAULT 0.00,
    tier VARCHAR(20) DEFAULT 'indie' CHECK (tier IN ('indie', 'rising', 'professional', 'major')),
    is_verified BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Genre statistics table
CREATE TABLE IF NOT EXISTS genre_stats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    genre VARCHAR(100) NOT NULL UNIQUE,
    song_count INTEGER DEFAULT 0,
    total_listens BIGINT DEFAULT 0,
    average_duration_seconds INTEGER DEFAULT 0,
    top_artists TEXT[], -- Array of artist IDs
    trending_score DECIMAL(10,4) DEFAULT 0.0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Listen sessions table (for tracking actual listening)
CREATE TABLE IF NOT EXISTS listen_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    album_id UUID REFERENCES albums(id) ON DELETE SET NULL,
    playlist_id UUID REFERENCES playlists(id) ON DELETE SET NULL,
    listen_duration_seconds INTEGER NOT NULL,
    completion_percentage DECIMAL(5,2) DEFAULT 0.00,
    quality_score DECIMAL(3,2) DEFAULT 1.00,
    device_type VARCHAR(50),
    location VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    session_start TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    session_end TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
CREATE INDEX IF NOT EXISTS idx_songs_genre ON songs(genre);
CREATE INDEX IF NOT EXISTS idx_songs_listen_count ON songs(listen_count DESC);
CREATE INDEX IF NOT EXISTS idx_songs_created_at ON songs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_songs_trending ON songs(is_trending, listen_count DESC) WHERE is_trending = true;
CREATE INDEX IF NOT EXISTS idx_songs_popular ON songs(is_popular, listen_count DESC) WHERE is_popular = true;
CREATE INDEX IF NOT EXISTS idx_songs_available_campaign ON songs(is_available_for_campaign) WHERE is_available_for_campaign = true;
CREATE INDEX IF NOT EXISTS idx_songs_available_ownership ON songs(is_available_for_ownership) WHERE is_available_for_ownership = true;
CREATE INDEX IF NOT EXISTS idx_songs_title_search ON songs USING gin(to_tsvector('english', title));

CREATE INDEX IF NOT EXISTS idx_albums_artist_id ON albums(artist_id);
CREATE INDEX IF NOT EXISTS idx_albums_published ON albums(is_published) WHERE is_published = true;
CREATE INDEX IF NOT EXISTS idx_albums_featured ON albums(is_featured) WHERE is_featured = true;
CREATE INDEX IF NOT EXISTS idx_albums_release_date ON albums(release_date DESC);

CREATE INDEX IF NOT EXISTS idx_album_tracks_album_id ON album_tracks(album_id);
CREATE INDEX IF NOT EXISTS idx_album_tracks_song_id ON album_tracks(song_id);
CREATE INDEX IF NOT EXISTS idx_album_tracks_track_number ON album_tracks(album_id, track_number);

CREATE INDEX IF NOT EXISTS idx_playlists_user_id ON playlists(user_id);
CREATE INDEX IF NOT EXISTS idx_playlists_public ON playlists(is_public) WHERE is_public = true;
CREATE INDEX IF NOT EXISTS idx_playlists_name_search ON playlists USING gin(to_tsvector('english', name));

CREATE INDEX IF NOT EXISTS idx_playlist_tracks_playlist_id ON playlist_tracks(playlist_id);
CREATE INDEX IF NOT EXISTS idx_playlist_tracks_song_id ON playlist_tracks(song_id);
CREATE INDEX IF NOT EXISTS idx_playlist_tracks_position ON playlist_tracks(playlist_id, position);

CREATE INDEX IF NOT EXISTS idx_artists_user_id ON artists(user_id);
CREATE INDEX IF NOT EXISTS idx_artists_verified ON artists(is_verified) WHERE is_verified = true;
CREATE INDEX IF NOT EXISTS idx_artists_featured ON artists(is_featured) WHERE is_featured = true;
CREATE INDEX IF NOT EXISTS idx_artists_tier ON artists(tier);
CREATE INDEX IF NOT EXISTS idx_artists_stage_name_search ON artists USING gin(to_tsvector('english', stage_name));

CREATE INDEX IF NOT EXISTS idx_listen_sessions_user_id ON listen_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_song_id ON listen_sessions(song_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_created_at ON listen_sessions(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_duration ON listen_sessions(listen_duration_seconds DESC);

-- Triggers for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_songs_updated_at ON songs;
CREATE TRIGGER update_songs_updated_at BEFORE UPDATE ON songs FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_albums_updated_at ON albums;
CREATE TRIGGER update_albums_updated_at BEFORE UPDATE ON albums FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_playlists_updated_at ON playlists;
CREATE TRIGGER update_playlists_updated_at BEFORE UPDATE ON playlists FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_artists_updated_at ON artists;
CREATE TRIGGER update_artists_updated_at BEFORE UPDATE ON artists FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_genre_stats_updated_at ON genre_stats;
CREATE TRIGGER update_genre_stats_updated_at BEFORE UPDATE ON genre_stats FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Triggers for maintaining counts
CREATE OR REPLACE FUNCTION update_album_track_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE albums SET track_count = track_count + 1 WHERE id = NEW.album_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE albums SET track_count = track_count - 1 WHERE id = OLD.album_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS album_track_count_trigger ON album_tracks;
CREATE TRIGGER album_track_count_trigger
    AFTER INSERT OR DELETE ON album_tracks
    FOR EACH ROW EXECUTE FUNCTION update_album_track_count();

CREATE OR REPLACE FUNCTION update_playlist_song_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE playlists SET song_count = song_count + 1 WHERE id = NEW.playlist_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE playlists SET song_count = song_count - 1 WHERE id = OLD.playlist_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS playlist_song_count_trigger ON playlist_tracks;
CREATE TRIGGER playlist_song_count_trigger
    AFTER INSERT OR DELETE ON playlist_tracks
    FOR EACH ROW EXECUTE FUNCTION update_playlist_song_count();

-- Sample data for development
INSERT INTO genre_stats (genre, song_count, total_listens, trending_score) VALUES
    ('Pop', 0, 0, 0.0),
    ('Rock', 0, 0, 0.0),
    ('Hip-Hop', 0, 0, 0.0),
    ('Electronic', 0, 0, 0.0),
    ('Jazz', 0, 0, 0.0),
    ('Classical', 0, 0, 0.0),
    ('Country', 0, 0, 0.0),
    ('R&B', 0, 0, 0.0),
    ('Indie', 0, 0, 0.0),
    ('Lo-fi', 0, 0, 0.0)
ON CONFLICT (genre) DO NOTHING;

-- Comments for documentation
COMMENT ON TABLE songs IS 'Core songs table storing music tracks';
COMMENT ON TABLE albums IS 'Albums containing multiple songs';
COMMENT ON TABLE playlists IS 'User-created playlists';
COMMENT ON TABLE artists IS 'Extended artist profiles linked to users';
COMMENT ON TABLE listen_sessions IS 'Tracking actual listening sessions for analytics and rewards';
COMMENT ON TABLE genre_stats IS 'Statistics and metrics for music genres';

COMMENT ON COLUMN songs.royalty_percentage IS 'Percentage of revenue going to the artist (0-100)';
COMMENT ON COLUMN songs.is_available_for_campaign IS 'Whether song can be used in NFT campaigns';
COMMENT ON COLUMN songs.is_available_for_ownership IS 'Whether song can be sold as fractional ownership';
COMMENT ON COLUMN listen_sessions.completion_percentage IS 'How much of the song was listened to (0-100)';
COMMENT ON COLUMN listen_sessions.quality_score IS 'Quality score for anti-fraud (0-1)'; 