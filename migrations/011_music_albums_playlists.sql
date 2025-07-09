-- Migration 011: Music Context - Albums and Playlists Support
-- Adds complete support for Albums and Playlists in the Music bounded context

-- Albums table
CREATE TABLE IF NOT EXISTS albums (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    artist_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    description TEXT,
    genre VARCHAR(100) NOT NULL,
    release_date TIMESTAMPTZ,
    cover_art_ipfs VARCHAR(255),
    album_type VARCHAR(50) NOT NULL DEFAULT 'album' CHECK (album_type IN ('album', 'ep', 'compilation', 'single')),
    is_published BOOLEAN NOT NULL DEFAULT false,
    total_duration_seconds INTEGER NOT NULL DEFAULT 0,
    track_count INTEGER NOT NULL DEFAULT 0,
    total_listens BIGINT NOT NULL DEFAULT 0,
    total_revenue DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Album tracks (songs in albums)
CREATE TABLE IF NOT EXISTS album_tracks (
    album_id UUID NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    track_number INTEGER NOT NULL,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (album_id, song_id),
    UNIQUE (album_id, track_number)
);

-- Playlists table
CREATE TABLE IF NOT EXISTS playlists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    creator_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    description TEXT,
    is_public BOOLEAN NOT NULL DEFAULT true,
    is_collaborative BOOLEAN NOT NULL DEFAULT false,
    cover_image_url TEXT,
    follower_count INTEGER NOT NULL DEFAULT 0,
    like_count INTEGER NOT NULL DEFAULT 0,
    listen_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Playlist tracks (songs in playlists)
CREATE TABLE IF NOT EXISTS playlist_tracks (
    playlist_id UUID NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (playlist_id, song_id),
    UNIQUE (playlist_id, position)
);

-- Playlist tags (for categorization and discovery)
CREATE TABLE IF NOT EXISTS playlist_tags (
    playlist_id UUID NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    PRIMARY KEY (playlist_id, tag)
);

-- Playlist collaborators (users who can edit collaborative playlists)
CREATE TABLE IF NOT EXISTS playlist_collaborators (
    playlist_id UUID NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (playlist_id, user_id)
);

-- Playlist followers (users following playlists)
CREATE TABLE IF NOT EXISTS playlist_followers (
    playlist_id UUID NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (playlist_id, user_id)
);

-- Playlist likes
CREATE TABLE IF NOT EXISTS playlist_likes (
    playlist_id UUID NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    liked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (playlist_id, user_id)
);

-- Artist profiles (enhanced artist information)
CREATE TABLE IF NOT EXISTS artist_profiles (
    artist_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name VARCHAR(255) NOT NULL,
    bio TEXT,
    profile_image_url TEXT,
    banner_image_url TEXT,
    location VARCHAR(255),
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verification_type VARCHAR(50) CHECK (verification_type IN ('platform', 'industry', 'celebrity')),
    follower_count INTEGER NOT NULL DEFAULT 0,
    total_songs INTEGER NOT NULL DEFAULT 0,
    total_albums INTEGER NOT NULL DEFAULT 0,
    total_listens BIGINT NOT NULL DEFAULT 0,
    total_revenue DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
    monthly_listeners INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Artist social links
CREATE TABLE IF NOT EXISTS artist_social_links (
    artist_id UUID NOT NULL REFERENCES artist_profiles(artist_id) ON DELETE CASCADE,
    platform VARCHAR(50) NOT NULL,
    url TEXT NOT NULL,
    PRIMARY KEY (artist_id, platform)
);

-- Artist genres (many-to-many relationship)
CREATE TABLE IF NOT EXISTS artist_genres (
    artist_id UUID NOT NULL REFERENCES artist_profiles(artist_id) ON DELETE CASCADE,
    genre VARCHAR(100) NOT NULL,
    PRIMARY KEY (artist_id, genre)
);

-- Artist activities (timeline of artist actions)
CREATE TABLE IF NOT EXISTS artist_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_id UUID NOT NULL REFERENCES artist_profiles(artist_id) ON DELETE CASCADE,
    activity_type VARCHAR(50) NOT NULL CHECK (activity_type IN ('song_released', 'album_released', 'concert_announced', 'collaboration', 'milestone')),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    related_id UUID, -- Can reference songs, albums, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance optimization

-- Albums indexes
CREATE INDEX IF NOT EXISTS idx_albums_artist_id ON albums(artist_id);
CREATE INDEX IF NOT EXISTS idx_albums_genre ON albums(genre);
CREATE INDEX IF NOT EXISTS idx_albums_release_date ON albums(release_date DESC);
CREATE INDEX IF NOT EXISTS idx_albums_total_listens ON albums(total_listens DESC);
CREATE INDEX IF NOT EXISTS idx_albums_created_at ON albums(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_albums_title_search ON albums USING gin(to_tsvector('english', title));

-- Album tracks indexes
CREATE INDEX IF NOT EXISTS idx_album_tracks_song_id ON album_tracks(song_id);
CREATE INDEX IF NOT EXISTS idx_album_tracks_track_number ON album_tracks(album_id, track_number);

-- Playlists indexes
CREATE INDEX IF NOT EXISTS idx_playlists_creator_id ON playlists(creator_id);
CREATE INDEX IF NOT EXISTS idx_playlists_is_public ON playlists(is_public);
CREATE INDEX IF NOT EXISTS idx_playlists_follower_count ON playlists(follower_count DESC);
CREATE INDEX IF NOT EXISTS idx_playlists_like_count ON playlists(like_count DESC);
CREATE INDEX IF NOT EXISTS idx_playlists_created_at ON playlists(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_playlists_name_search ON playlists USING gin(to_tsvector('english', name));

-- Playlist tracks indexes
CREATE INDEX IF NOT EXISTS idx_playlist_tracks_song_id ON playlist_tracks(song_id);
CREATE INDEX IF NOT EXISTS idx_playlist_tracks_position ON playlist_tracks(playlist_id, position);
CREATE INDEX IF NOT EXISTS idx_playlist_tracks_added_by ON playlist_tracks(added_by);

-- Playlist tags indexes
CREATE INDEX IF NOT EXISTS idx_playlist_tags_tag ON playlist_tags(tag);

-- Playlist followers indexes
CREATE INDEX IF NOT EXISTS idx_playlist_followers_user_id ON playlist_followers(user_id);
CREATE INDEX IF NOT EXISTS idx_playlist_followers_followed_at ON playlist_followers(followed_at DESC);

-- Playlist likes indexes
CREATE INDEX IF NOT EXISTS idx_playlist_likes_user_id ON playlist_likes(user_id);

-- Artist profiles indexes
CREATE INDEX IF NOT EXISTS idx_artist_profiles_display_name ON artist_profiles(display_name);
CREATE INDEX IF NOT EXISTS idx_artist_profiles_is_verified ON artist_profiles(is_verified);
CREATE INDEX IF NOT EXISTS idx_artist_profiles_follower_count ON artist_profiles(follower_count DESC);
CREATE INDEX IF NOT EXISTS idx_artist_profiles_total_listens ON artist_profiles(total_listens DESC);
CREATE INDEX IF NOT EXISTS idx_artist_profiles_location ON artist_profiles(location);

-- Artist activities indexes
CREATE INDEX IF NOT EXISTS idx_artist_activities_artist_id ON artist_activities(artist_id);
CREATE INDEX IF NOT EXISTS idx_artist_activities_type ON artist_activities(activity_type);
CREATE INDEX IF NOT EXISTS idx_artist_activities_created_at ON artist_activities(created_at DESC);

-- Triggers for updating counters and timestamps

-- Function to update album counters
CREATE OR REPLACE FUNCTION update_album_counters()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Update track count and total duration when song is added to album
        UPDATE albums 
        SET track_count = track_count + 1,
            total_duration_seconds = total_duration_seconds + (
                SELECT duration_seconds FROM songs WHERE id = NEW.song_id
            ),
            updated_at = NOW()
        WHERE id = NEW.album_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        -- Update track count and total duration when song is removed from album
        UPDATE albums 
        SET track_count = track_count - 1,
            total_duration_seconds = total_duration_seconds - (
                SELECT duration_seconds FROM songs WHERE id = OLD.song_id
            ),
            updated_at = NOW()
        WHERE id = OLD.album_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to update playlist counters
CREATE OR REPLACE FUNCTION update_playlist_counters()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE playlists SET updated_at = NOW() WHERE id = NEW.playlist_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE playlists SET updated_at = NOW() WHERE id = OLD.playlist_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to update artist profile counters
CREATE OR REPLACE FUNCTION update_artist_stats()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF TG_TABLE_NAME = 'songs' THEN
            UPDATE artist_profiles 
            SET total_songs = total_songs + 1, updated_at = NOW()
            WHERE artist_id = NEW.artist_id;
        ELSIF TG_TABLE_NAME = 'albums' THEN
            UPDATE artist_profiles 
            SET total_albums = total_albums + 1, updated_at = NOW()
            WHERE artist_id = NEW.artist_id;
        END IF;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        IF TG_TABLE_NAME = 'songs' THEN
            UPDATE artist_profiles 
            SET total_songs = total_songs - 1, updated_at = NOW()
            WHERE artist_id = OLD.artist_id;
        ELSIF TG_TABLE_NAME = 'albums' THEN
            UPDATE artist_profiles 
            SET total_albums = total_albums - 1, updated_at = NOW()
            WHERE artist_id = OLD.artist_id;
        END IF;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers
CREATE TRIGGER trigger_album_tracks_update_counters
    AFTER INSERT OR DELETE ON album_tracks
    FOR EACH ROW EXECUTE FUNCTION update_album_counters();

CREATE TRIGGER trigger_playlist_tracks_update_counters
    AFTER INSERT OR DELETE ON playlist_tracks
    FOR EACH ROW EXECUTE FUNCTION update_playlist_counters();

CREATE TRIGGER trigger_songs_update_artist_stats
    AFTER INSERT OR DELETE ON songs
    FOR EACH ROW EXECUTE FUNCTION update_artist_stats();

CREATE TRIGGER trigger_albums_update_artist_stats
    AFTER INSERT OR DELETE ON albums
    FOR EACH ROW EXECUTE FUNCTION update_artist_stats();

-- Insert sample data for testing (optional)
INSERT INTO artist_profiles (artist_id, display_name, bio, is_verified) 
SELECT id, username, 'Sample artist bio', false 
FROM users 
WHERE id IN (
    SELECT DISTINCT artist_id FROM songs LIMIT 5
)
ON CONFLICT (artist_id) DO NOTHING;

COMMIT; 