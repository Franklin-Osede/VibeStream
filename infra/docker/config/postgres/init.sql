-- Crear extensiones necesarias
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Crear esquemas
CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS music;
CREATE SCHEMA IF NOT EXISTS blockchain;

-- Configurar búsqueda de esquemas
ALTER DATABASE vibestream SET search_path TO public, auth, music, blockchain;

-- Crear tablas base (ejemplos iniciales)
CREATE TABLE IF NOT EXISTS auth.users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    wallet_address VARCHAR(42),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS music.songs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    artist_id UUID NOT NULL,
    ipfs_hash VARCHAR(255) NOT NULL,
    duration INTEGER NOT NULL,
    genre VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS blockchain.royalties (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    song_id UUID NOT NULL,
    token_id BIGINT NOT NULL,
    contract_address VARCHAR(42) NOT NULL,
    royalty_percentage DECIMAL(5,2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_song FOREIGN KEY(song_id) REFERENCES music.songs(id)
);

-- Crear índices
CREATE INDEX IF NOT EXISTS idx_users_wallet ON auth.users(wallet_address);
CREATE INDEX IF NOT EXISTS idx_songs_artist ON music.songs(artist_id);
CREATE INDEX IF NOT EXISTS idx_royalties_song ON blockchain.royalties(song_id);

-- Crear funciones de actualización de timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Crear triggers para actualización automática de timestamps
CREATE TRIGGER update_users_modtime
    BEFORE UPDATE ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_songs_modtime
    BEFORE UPDATE ON music.songs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 