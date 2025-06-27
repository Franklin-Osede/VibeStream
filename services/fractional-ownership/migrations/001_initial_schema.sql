-- Migration 001: Initial schema for Fractional Ownership Context
-- Created: 2024

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table: fractional_songs
-- Stores songs that can be purchased fractionally
CREATE TABLE fractional_songs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    song_id UUID NOT NULL, -- Reference to the main Song entity from Music Context
    artist_id UUID NOT NULL, -- Reference to Artist from Music Context
    title VARCHAR(255) NOT NULL,
    total_shares INTEGER NOT NULL CHECK (total_shares > 0),
    available_shares INTEGER NOT NULL CHECK (available_shares >= 0),
    current_price_per_share DECIMAL(10,2) NOT NULL CHECK (current_price_per_share > 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT check_available_shares_not_exceed_total 
        CHECK (available_shares <= total_shares),
    
    -- Indexes
    UNIQUE(song_id), -- One fractional song per main song
    INDEX idx_fractional_songs_artist_id (artist_id),
    INDEX idx_fractional_songs_created_at (created_at),
    INDEX idx_fractional_songs_price (current_price_per_share)
);

-- Table: share_ownerships
-- Stores ownership information for each user-song combination
CREATE TABLE share_ownerships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    user_id UUID NOT NULL, -- Reference to User from User Context
    shares_owned INTEGER NOT NULL CHECK (shares_owned > 0),
    purchase_price DECIMAL(10,2) NOT NULL CHECK (purchase_price > 0),
    total_earnings DECIMAL(10,2) DEFAULT 0.00 CHECK (total_earnings >= 0),
    purchase_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(fractional_song_id, user_id), -- One ownership record per user-song
    
    -- Indexes
    INDEX idx_share_ownerships_user_id (user_id),
    INDEX idx_share_ownerships_song_id (fractional_song_id),
    INDEX idx_share_ownerships_purchase_date (purchase_date),
    INDEX idx_share_ownerships_earnings (total_earnings)
);

-- Table: share_transactions
-- Stores all share transaction history (purchases, transfers, sales)
CREATE TABLE share_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    from_user_id UUID, -- NULL for initial purchases from artist
    to_user_id UUID NOT NULL, -- Target user
    shares_quantity INTEGER NOT NULL CHECK (shares_quantity > 0),
    total_amount DECIMAL(10,2) NOT NULL CHECK (total_amount > 0),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'failed', 'cancelled')),
    transaction_type VARCHAR(20) DEFAULT 'purchase' CHECK (transaction_type IN ('purchase', 'transfer', 'sale')),
    blockchain_tx_hash VARCHAR(128), -- For Web3 transactions
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Indexes
    INDEX idx_share_transactions_from_user (from_user_id),
    INDEX idx_share_transactions_to_user (to_user_id),
    INDEX idx_share_transactions_song_id (fractional_song_id),
    INDEX idx_share_transactions_status (status),
    INDEX idx_share_transactions_type (transaction_type),
    INDEX idx_share_transactions_created_at (created_at),
    INDEX idx_share_transactions_blockchain_hash (blockchain_tx_hash)
);

-- Table: revenue_distributions
-- Stores revenue distribution events and calculations
CREATE TABLE revenue_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    total_revenue DECIMAL(12,2) NOT NULL CHECK (total_revenue > 0),
    distribution_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    source_type VARCHAR(50) NOT NULL, -- 'streaming', 'nft_sale', 'licensing', etc.
    source_details JSONB, -- Additional metadata about revenue source
    shareholders_count INTEGER NOT NULL CHECK (shareholders_count > 0),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'distributed', 'failed')),
    blockchain_tx_hash VARCHAR(128),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Indexes
    INDEX idx_revenue_distributions_song_id (fractional_song_id),
    INDEX idx_revenue_distributions_date (distribution_date),
    INDEX idx_revenue_distributions_status (status),
    INDEX idx_revenue_distributions_source (source_type)
);

-- Table: individual_revenue_payouts
-- Stores individual payouts to each shareholder
CREATE TABLE individual_revenue_payouts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    revenue_distribution_id UUID NOT NULL REFERENCES revenue_distributions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    shares_owned INTEGER NOT NULL CHECK (shares_owned > 0),
    ownership_percentage DECIMAL(5,4) NOT NULL CHECK (ownership_percentage > 0 AND ownership_percentage <= 1),
    payout_amount DECIMAL(10,2) NOT NULL CHECK (payout_amount > 0),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'paid', 'failed')),
    blockchain_tx_hash VARCHAR(128),
    paid_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(revenue_distribution_id, user_id), -- One payout per user per distribution
    
    -- Indexes
    INDEX idx_individual_payouts_user_id (user_id),
    INDEX idx_individual_payouts_song_id (fractional_song_id),
    INDEX idx_individual_payouts_distribution_id (revenue_distribution_id),
    INDEX idx_individual_payouts_status (status)
);

-- Table: price_history
-- Stores historical price data for analytics and charting
CREATE TABLE price_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    price DECIMAL(10,2) NOT NULL CHECK (price > 0),
    volume INTEGER DEFAULT 0, -- Number of shares traded in this period
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    period_type VARCHAR(10) DEFAULT 'daily' CHECK (period_type IN ('hourly', 'daily', 'weekly')),
    
    -- Indexes
    INDEX idx_price_history_song_id (fractional_song_id),
    INDEX idx_price_history_timestamp (timestamp),
    INDEX idx_price_history_period (period_type),
    UNIQUE(fractional_song_id, timestamp, period_type) -- No duplicate entries for same period
);

-- View: user_portfolio_summary
-- Convenient view for user portfolio analytics
CREATE VIEW user_portfolio_summary AS
SELECT 
    so.user_id,
    COUNT(DISTINCT so.fractional_song_id) as songs_owned,
    SUM(so.shares_owned) as total_shares,
    SUM(so.shares_owned * so.purchase_price) as total_investment,
    SUM(so.total_earnings) as total_earnings,
    CASE 
        WHEN SUM(so.shares_owned * so.purchase_price) > 0 
        THEN (SUM(so.total_earnings) / SUM(so.shares_owned * so.purchase_price)) * 100 
        ELSE 0 
    END as roi_percentage
FROM share_ownerships so
GROUP BY so.user_id;

-- View: song_market_stats
-- Market statistics per song
CREATE VIEW song_market_stats AS
SELECT 
    fs.id as fractional_song_id,
    fs.title,
    fs.artist_id,
    fs.total_shares,
    fs.available_shares,
    fs.current_price_per_share,
    COUNT(DISTINCT so.user_id) as unique_shareholders,
    SUM(so.shares_owned) as shares_sold,
    AVG(so.purchase_price) as avg_purchase_price,
    SUM(so.total_earnings) as total_earnings_distributed,
    (fs.total_shares - fs.available_shares)::float / fs.total_shares * 100 as ownership_percentage
FROM fractional_songs fs
LEFT JOIN share_ownerships so ON fs.id = so.fractional_song_id
GROUP BY fs.id, fs.title, fs.artist_id, fs.total_shares, fs.available_shares, fs.current_price_per_share;

-- Triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_fractional_songs_updated_at 
    BEFORE UPDATE ON fractional_songs 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_share_ownerships_updated_at 
    BEFORE UPDATE ON share_ownerships 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_share_transactions_updated_at 
    BEFORE UPDATE ON share_transactions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_revenue_distributions_updated_at 
    BEFORE UPDATE ON revenue_distributions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Sample data for testing (optional)
-- INSERT INTO fractional_songs (id, song_id, artist_id, title, total_shares, available_shares, current_price_per_share)
-- VALUES 
--     ('123e4567-e89b-12d3-a456-426614174000', '223e4567-e89b-12d3-a456-426614174000', '323e4567-e89b-12d3-a456-426614174000', 'Epic Song #1', 1000, 750, 10.50);

-- Comments
COMMENT ON TABLE fractional_songs IS 'Songs available for fractional ownership';
COMMENT ON TABLE share_ownerships IS 'User ownership records for fractional songs';
COMMENT ON TABLE share_transactions IS 'Transaction history for share purchases, transfers, and sales';
COMMENT ON TABLE revenue_distributions IS 'Revenue distribution events from song earnings';
COMMENT ON TABLE individual_revenue_payouts IS 'Individual payouts to shareholders from revenue distributions';
COMMENT ON TABLE price_history IS 'Historical price data for market analytics';

COMMENT ON COLUMN fractional_songs.song_id IS 'Reference to the main Song entity from Music Context';
COMMENT ON COLUMN fractional_songs.artist_id IS 'Reference to Artist from Music Context';
COMMENT ON COLUMN share_ownerships.user_id IS 'Reference to User from User Context';
COMMENT ON COLUMN share_transactions.blockchain_tx_hash IS 'Hash of the blockchain transaction for Web3 integration'; 