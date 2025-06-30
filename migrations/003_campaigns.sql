-- Migration: 003_campaigns.sql
-- Campaigns table for DDD Campaign Context

CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY,
    song_id UUID NOT NULL,
    artist_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    boost_multiplier NUMERIC(5,2) NOT NULL CHECK (boost_multiplier BETWEEN 1.0 AND 10.0),
    nft_price NUMERIC(10,2) NOT NULL CHECK (nft_price > 0),
    max_nfts INTEGER NOT NULL CHECK (max_nfts > 0),
    nfts_sold INTEGER DEFAULT 0 CHECK (nfts_sold >= 0 AND nfts_sold <= max_nfts),
    target_revenue NUMERIC(15,2),
    status VARCHAR(50) NOT NULL DEFAULT 'Draft',
    nft_contract_address VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_campaigns_artist_id ON campaigns(artist_id);
CREATE INDEX IF NOT EXISTS idx_campaigns_song_id ON campaigns(song_id);
CREATE INDEX IF NOT EXISTS idx_campaigns_status ON campaigns(status);
CREATE INDEX IF NOT EXISTS idx_campaigns_dates ON campaigns(start_date, end_date);

-- Campaign NFTs table
CREATE TABLE IF NOT EXISTS campaign_nfts (
    id UUID PRIMARY KEY,
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    token_id BIGINT,
    owner_address VARCHAR(255),
    metadata_uri TEXT NOT NULL,
    tradeable BOOLEAN DEFAULT TRUE,
    purchase_price NUMERIC(10,2),
    purchased_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_campaign_nfts_campaign_id ON campaign_nfts(campaign_id);
CREATE INDEX IF NOT EXISTS idx_campaign_nfts_owner ON campaign_nfts(owner_address); 