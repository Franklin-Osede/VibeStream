-- Migration: 007_fractional_shares.sql
-- Description: Creates tables for fractional shares (missing from 005_fractional_ownership.sql)
-- Author: AI Assistant
-- Created: 2024

-- Enable UUID extension if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- FRACTIONAL SHARES TABLE
-- =====================================================
CREATE TABLE fractional_shares (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract_id UUID NOT NULL,
    owner_id UUID NOT NULL,
    song_id UUID NOT NULL,
    
    -- Share ownership details
    ownership_percentage DECIMAL(8,4) NOT NULL CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    purchase_price DECIMAL(10,2) NOT NULL CHECK (purchase_price > 0),
    purchased_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Vesting configuration
    vesting_start_date TIMESTAMPTZ,
    vesting_end_date TIMESTAMPTZ,
    
    -- Share status
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    lock_reason VARCHAR(50),
    locked_until TIMESTAMPTZ,
    
    -- Financial tracking
    total_revenue_received DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    is_tradeable BOOLEAN NOT NULL DEFAULT TRUE,
    current_market_value DECIMAL(10,2) NOT NULL,
    
    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    version BIGINT NOT NULL DEFAULT 1,
    
    -- Foreign key constraints
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE CASCADE,
    -- FOREIGN KEY (owner_id) REFERENCES users(id), -- Will be enabled when users context is ready
    -- FOREIGN KEY (song_id) REFERENCES songs(id),  -- Will be enabled when songs context is ready
    
    -- Business rules constraints
    CONSTRAINT valid_vesting_period CHECK (
        (vesting_start_date IS NULL AND vesting_end_date IS NULL) OR
        (vesting_start_date IS NOT NULL AND vesting_end_date IS NOT NULL AND vesting_end_date > vesting_start_date)
    ),
    CONSTRAINT valid_lock_reason CHECK (
        (is_locked = FALSE AND lock_reason IS NULL) OR
        (is_locked = TRUE AND lock_reason IS NOT NULL)
    )
);

-- =====================================================
-- INDICES FOR PERFORMANCE
-- =====================================================

-- Primary lookup indices
CREATE INDEX idx_fractional_shares_contract_id ON fractional_shares(contract_id);
CREATE INDEX idx_fractional_shares_owner_id ON fractional_shares(owner_id);
CREATE INDEX idx_fractional_shares_song_id ON fractional_shares(song_id);

-- Query optimization indices
CREATE INDEX idx_fractional_shares_tradeable ON fractional_shares(is_tradeable) WHERE is_tradeable = TRUE;
CREATE INDEX idx_fractional_shares_locked ON fractional_shares(is_locked, locked_until) WHERE is_locked = TRUE;
CREATE INDEX idx_fractional_shares_vesting ON fractional_shares(vesting_end_date) WHERE vesting_end_date IS NOT NULL;

-- Composite indices for complex queries
CREATE INDEX idx_fractional_shares_owner_tradeable ON fractional_shares(owner_id, is_tradeable) WHERE is_tradeable = TRUE;
CREATE INDEX idx_fractional_shares_contract_owner ON fractional_shares(contract_id, owner_id);

-- =====================================================
-- SHARE REVENUE DISTRIBUTIONS TABLE
-- =====================================================
CREATE TABLE share_revenue_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    share_id UUID NOT NULL,
    contract_id UUID NOT NULL,
    distribution_id UUID NOT NULL,
    
    -- Distribution details
    revenue_amount DECIMAL(10,2) NOT NULL CHECK (revenue_amount >= 0),
    ownership_percentage_at_distribution DECIMAL(8,4) NOT NULL,
    revenue_source VARCHAR(100) NOT NULL,
    
    -- Transaction details
    transaction_hash VARCHAR(100),
    blockchain_network VARCHAR(50),
    distribution_status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (distribution_status IN ('Pending', 'Processing', 'Completed', 'Failed')),
    
    -- Audit fields
    distributed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Foreign key constraints
    FOREIGN KEY (share_id) REFERENCES fractional_shares(id) ON DELETE CASCADE,
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE CASCADE
);

-- Indices for revenue distributions
CREATE INDEX idx_share_revenue_distributions_share_id ON share_revenue_distributions(share_id);
CREATE INDEX idx_share_revenue_distributions_contract_id ON share_revenue_distributions(contract_id);
CREATE INDEX idx_share_revenue_distributions_status ON share_revenue_distributions(distribution_status);
CREATE INDEX idx_share_revenue_distributions_date ON share_revenue_distributions(distributed_at);

-- =====================================================
-- TRIGGERS FOR AUTOMATIC TIMESTAMP UPDATES
-- =====================================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger for fractional_shares
CREATE TRIGGER update_fractional_shares_updated_at
    BEFORE UPDATE ON fractional_shares
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =====================================================
-- VIEWS FOR ANALYTICS
-- =====================================================

-- View for user portfolios
CREATE VIEW user_portfolios AS
SELECT 
    fs.owner_id,
    COUNT(fs.id) as total_shares,
    SUM(fs.ownership_percentage) as total_ownership_percentage,
    SUM(fs.current_market_value) as total_portfolio_value,
    SUM(fs.total_revenue_received) as total_revenue_earned,
    AVG(fs.ownership_percentage) as avg_ownership_per_share,
    MAX(fs.purchased_at) as last_purchase_date,
    COUNT(CASE WHEN fs.is_tradeable THEN 1 END) as tradeable_shares,
    COUNT(CASE WHEN fs.is_locked THEN 1 END) as locked_shares
FROM fractional_shares fs
GROUP BY fs.owner_id;

-- View for contract share analytics
CREATE VIEW contract_share_analytics AS
SELECT 
    oc.id as contract_id,
    oc.song_id,
    oc.artist_id,
    COUNT(fs.id) as total_shareholders,
    SUM(fs.ownership_percentage) as total_sold_percentage,
    (oc.artist_retained_percentage + COALESCE(SUM(fs.ownership_percentage), 0)) as total_allocated_percentage,
    (100 - oc.artist_retained_percentage - COALESCE(SUM(fs.ownership_percentage), 0)) as remaining_percentage,
    SUM(fs.current_market_value) as total_share_market_value,
    AVG(fs.ownership_percentage) as avg_ownership_per_shareholder,
    COUNT(CASE WHEN fs.is_tradeable THEN 1 END) as tradeable_shares_count,
    SUM(fs.total_revenue_received) as total_revenue_distributed
FROM ownership_contracts oc
LEFT JOIN fractional_shares fs ON oc.id = fs.contract_id
GROUP BY oc.id, oc.song_id, oc.artist_id, oc.artist_retained_percentage;

-- =====================================================
-- SAMPLE DATA FOR TESTING
-- =====================================================

-- Note: Sample data would be inserted here, but we'll skip it for now
-- to avoid dependency issues with other contexts that aren't fully set up yet.

-- Example structure for when ready:
/*
INSERT INTO fractional_shares (
    contract_id, owner_id, song_id, ownership_percentage, 
    purchase_price, current_market_value, is_tradeable
) VALUES 
(
    '550e8400-e29b-41d4-a716-446655440000'::uuid,  -- contract_id
    '550e8400-e29b-41d4-a716-446655440001'::uuid,  -- owner_id
    '550e8400-e29b-41d4-a716-446655440002'::uuid,  -- song_id
    5.50,  -- ownership_percentage
    100.00,  -- purchase_price
    110.00,  -- current_market_value
    true  -- is_tradeable
);
*/ 