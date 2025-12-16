-- Migration: 020_harden_schema.sql
-- Description: Enforce strict Foreign Key relationships to unify the schema and ensure data integrity.

-- 0. CLEANUP ORPHANED DATA (Safety Step)
-- Remove records that would violate the new FKs
DELETE FROM payments WHERE payer_id NOT IN (SELECT id FROM users);
DELETE FROM payments WHERE payee_id NOT IN (SELECT id FROM users);
DELETE FROM listen_sessions WHERE user_id NOT IN (SELECT id FROM users);
DELETE FROM campaigns WHERE artist_id NOT IN (SELECT id FROM users);
DELETE FROM royalty_distributions WHERE song_id NOT IN (SELECT id FROM songs);
DELETE FROM royalty_distributions WHERE artist_id NOT IN (SELECT id FROM users);

-- 1. Harden Listen Sessions (Listen Reward Context)
-- Ensure every session is linked to a real User, Song, and Artist.
ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_user 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_song 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;

-- Note: We link to 'artists' table (profile), not 'users' table, to match 'songs' schema.
ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_artist 
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE;

-- 2. Harden Payments (Payment Context)
-- Ensure real users are paying and receiving money.
ALTER TABLE payments
    ADD CONSTRAINT fk_payments_payer 
    FOREIGN KEY (payer_id) REFERENCES users(id);

ALTER TABLE payments
    ADD CONSTRAINT fk_payments_payee 
    FOREIGN KEY (payee_id) REFERENCES users(id);

-- 3. Harden Campaigns (Campaign Context)
-- Ensure campaigns are attached to real songs and artists.
ALTER TABLE campaigns
    ADD CONSTRAINT fk_campaigns_song 
    FOREIGN KEY (song_id) REFERENCES songs(id);

ALTER TABLE campaigns
    ADD CONSTRAINT fk_campaigns_artist 
    FOREIGN KEY (artist_id) REFERENCES artists(id);

-- 4. Harden Royalty Distributions (Payment Context)
ALTER TABLE royalty_distributions
    ADD CONSTRAINT fk_royalty_distributions_song
    FOREIGN KEY (song_id) REFERENCES songs(id);

ALTER TABLE royalty_distributions
    ADD CONSTRAINT fk_royalty_distributions_artist
    FOREIGN KEY (artist_id) REFERENCES artists(id);

-- 5. Standardize Indexes (if not already present in previous migrations)
-- These might be redundant if the original migrations included them, but good for safety.
CREATE INDEX IF NOT EXISTS idx_payments_payer_id ON payments(payer_id);
CREATE INDEX IF NOT EXISTS idx_payments_payee_id ON payments(payee_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_user_id ON listen_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_song_id ON listen_sessions(song_id);
