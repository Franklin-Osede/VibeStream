-- Drop triggers
DROP TRIGGER IF EXISTS update_song_nfts_modtime ON blockchain.song_nfts;
DROP TRIGGER IF EXISTS update_contracts_modtime ON blockchain.contracts;
DROP TRIGGER IF EXISTS update_playlists_modtime ON music.playlists;
DROP TRIGGER IF EXISTS update_songs_modtime ON music.songs;
DROP TRIGGER IF EXISTS update_artists_modtime ON music.artists;
DROP TRIGGER IF EXISTS update_users_modtime ON auth.users;

-- Drop trigger function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS blockchain.idx_royalty_payments_nft;
DROP INDEX IF EXISTS blockchain.idx_song_nfts_contract_token;
DROP INDEX IF EXISTS music.idx_playlist_songs_song;
DROP INDEX IF EXISTS music.idx_songs_artist;
DROP INDEX IF EXISTS auth.idx_users_wallet;

-- Drop tables
DROP TABLE IF EXISTS blockchain.royalty_payments;
DROP TABLE IF EXISTS blockchain.song_nfts;
DROP TABLE IF EXISTS blockchain.contracts;
DROP TABLE IF EXISTS music.playlist_songs;
DROP TABLE IF EXISTS music.playlists;
DROP TABLE IF EXISTS music.songs;
DROP TABLE IF EXISTS music.artists;
DROP TABLE IF EXISTS auth.users;

-- Drop schemas
DROP SCHEMA IF EXISTS blockchain;
DROP SCHEMA IF EXISTS music;
DROP SCHEMA IF EXISTS auth;

-- Drop extensions
DROP EXTENSION IF EXISTS "pgcrypto";
DROP EXTENSION IF EXISTS "uuid-ossp"; 