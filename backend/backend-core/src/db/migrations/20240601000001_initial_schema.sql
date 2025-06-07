-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create schemas
CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS music;
CREATE SCHEMA IF NOT EXISTS blockchain;

-- Users table
CREATE TABLE auth.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    wallet_address VARCHAR(42),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Artists table (extends users)
CREATE TABLE music.artists (
    id UUID PRIMARY KEY REFERENCES auth.users(id),
    name VARCHAR(255) NOT NULL,
    bio TEXT,
    profile_image_url VARCHAR(255),
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Songs table
CREATE TABLE music.songs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    artist_id UUID NOT NULL REFERENCES music.artists(id),
    duration_seconds INTEGER NOT NULL,
    genre VARCHAR(50),
    ipfs_hash VARCHAR(255) NOT NULL,
    cover_art_url VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Playlists table
CREATE TABLE music.playlists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    description TEXT,
    is_public BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Playlist songs junction table
CREATE TABLE music.playlist_songs (
    playlist_id UUID REFERENCES music.playlists(id) ON DELETE CASCADE,
    song_id UUID REFERENCES music.songs(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    added_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (playlist_id, song_id)
);

-- NFT Contracts table
CREATE TABLE blockchain.contracts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    address VARCHAR(42) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    chain_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Song NFTs table
CREATE TABLE blockchain.song_nfts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    song_id UUID NOT NULL REFERENCES music.songs(id),
    contract_id UUID NOT NULL REFERENCES blockchain.contracts(id),
    token_id BIGINT NOT NULL,
    royalty_percentage DECIMAL(5,2) NOT NULL,
    owner_address VARCHAR(42) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(contract_id, token_id)
);

-- Royalty payments table
CREATE TABLE blockchain.royalty_payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    song_nft_id UUID NOT NULL REFERENCES blockchain.song_nfts(id),
    amount DECIMAL(20,18) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    tx_hash VARCHAR(66) NOT NULL,
    paid_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_users_wallet ON auth.users(wallet_address);
CREATE INDEX idx_songs_artist ON music.songs(artist_id);
CREATE INDEX idx_playlist_songs_song ON music.playlist_songs(song_id);
CREATE INDEX idx_song_nfts_contract_token ON blockchain.song_nfts(contract_id, token_id);
CREATE INDEX idx_royalty_payments_nft ON blockchain.royalty_payments(song_nft_id);

-- Create updated_at triggers
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply triggers
CREATE TRIGGER update_users_modtime
    BEFORE UPDATE ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_artists_modtime
    BEFORE UPDATE ON music.artists
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_songs_modtime
    BEFORE UPDATE ON music.songs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_playlists_modtime
    BEFORE UPDATE ON music.playlists
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_contracts_modtime
    BEFORE UPDATE ON blockchain.contracts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_song_nfts_modtime
    BEFORE UPDATE ON blockchain.song_nfts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 