-- Migration: 003_campaigns.sql
-- Campaigns table for DDD Campaign Context

CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY,
    artist_id UUID NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    nft_contract VARCHAR(255) NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    end_date   TIMESTAMPTZ NOT NULL,
    multiplier NUMERIC(3,1) NOT NULL CHECK (multiplier BETWEEN 1.0 AND 5.0),
    is_active  BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_campaigns_artist_id ON campaigns(artist_id); 