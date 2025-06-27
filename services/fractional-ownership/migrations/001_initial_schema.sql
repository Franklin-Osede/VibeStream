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
    artist_reserved_shares INTEGER DEFAULT 0 NOT NULL CHECK (artist_reserved_shares >= 0),
    fan_available_shares INTEGER NOT NULL CHECK (fan_available_shares >= 0),
    artist_revenue_percentage DECIMAL(5,4) DEFAULT 0.0 CHECK (artist_revenue_percentage >= 0 AND artist_revenue_percentage <= 1),
    available_shares INTEGER NOT NULL CHECK (available_shares >= 0),
    current_price_per_share DECIMAL(10,2) NOT NULL CHECK (current_price_per_share > 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT check_available_shares_not_exceed_total 
        CHECK (available_shares <= total_shares),
    CONSTRAINT check_artist_shares_not_exceed_total
        CHECK (artist_reserved_shares <= total_shares),
    CONSTRAINT check_fan_shares_consistency
        CHECK (fan_available_shares = total_shares - artist_reserved_shares),
    
    -- Unique constraints
    UNIQUE(song_id) -- One fractional song per main song
);

-- Create indexes for fractional_songs
CREATE INDEX idx_fractional_songs_artist_id ON fractional_songs(artist_id);
CREATE INDEX idx_fractional_songs_created_at ON fractional_songs(created_at);
CREATE INDEX idx_fractional_songs_price ON fractional_songs(current_price_per_share);

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
    last_earning_date TIMESTAMP WITH TIME ZONE, -- Added missing field
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(fractional_song_id, user_id) -- One ownership record per user-song
);

-- Create indexes for share_ownerships
CREATE INDEX idx_share_ownerships_user_id ON share_ownerships(user_id);
CREATE INDEX idx_share_ownerships_song_id ON share_ownerships(fractional_song_id);
CREATE INDEX idx_share_ownerships_purchase_date ON share_ownerships(purchase_date);
CREATE INDEX idx_share_ownerships_earnings ON share_ownerships(total_earnings);

-- Table: share_transactions
-- Stores all share transaction history (purchases, transfers, sales)
CREATE TABLE share_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    from_user_id UUID, -- NULL for initial purchases from artist (seller_id)
    to_user_id UUID NOT NULL, -- Target user (buyer_id)
    buyer_id UUID, -- Alias for to_user_id for compatibility
    seller_id UUID, -- Alias for from_user_id for compatibility
    shares_quantity INTEGER NOT NULL CHECK (shares_quantity > 0),
    price_per_share DECIMAL(10,2) NOT NULL CHECK (price_per_share > 0), -- Added missing field
    total_amount DECIMAL(10,2) NOT NULL CHECK (total_amount > 0),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'failed', 'cancelled')),
    transaction_type VARCHAR(20) DEFAULT 'purchase' CHECK (transaction_type IN ('purchase', 'transfer', 'sale')),
    blockchain_tx_hash VARCHAR(128), -- For Web3 transactions
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE -- Added missing field
);

-- Create indexes for share_transactions
CREATE INDEX idx_share_transactions_from_user ON share_transactions(from_user_id);
CREATE INDEX idx_share_transactions_to_user ON share_transactions(to_user_id);
CREATE INDEX idx_share_transactions_song_id ON share_transactions(fractional_song_id);
CREATE INDEX idx_share_transactions_status ON share_transactions(status);
CREATE INDEX idx_share_transactions_type ON share_transactions(transaction_type);
CREATE INDEX idx_share_transactions_created_at ON share_transactions(created_at);
CREATE INDEX idx_share_transactions_blockchain_hash ON share_transactions(blockchain_tx_hash);

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
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for revenue_distributions
CREATE INDEX idx_revenue_distributions_song_id ON revenue_distributions(fractional_song_id);
CREATE INDEX idx_revenue_distributions_date ON revenue_distributions(distribution_date);
CREATE INDEX idx_revenue_distributions_status ON revenue_distributions(status);
CREATE INDEX idx_revenue_distributions_source ON revenue_distributions(source_type);

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
    UNIQUE(revenue_distribution_id, user_id) -- One payout per user per distribution
);

-- Create indexes for individual_revenue_payouts
CREATE INDEX idx_individual_payouts_user_id ON individual_revenue_payouts(user_id);
CREATE INDEX idx_individual_payouts_song_id ON individual_revenue_payouts(fractional_song_id);
CREATE INDEX idx_individual_payouts_distribution_id ON individual_revenue_payouts(revenue_distribution_id);
CREATE INDEX idx_individual_payouts_status ON individual_revenue_payouts(status);

-- Table: price_history
-- Stores historical price data for analytics and charting
CREATE TABLE price_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fractional_song_id UUID NOT NULL REFERENCES fractional_songs(id) ON DELETE CASCADE,
    price DECIMAL(10,2) NOT NULL CHECK (price > 0),
    volume INTEGER DEFAULT 0, -- Number of shares traded in this period
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    period_type VARCHAR(10) DEFAULT 'daily' CHECK (period_type IN ('hourly', 'daily', 'weekly')),
    
    -- Unique constraint
    UNIQUE(fractional_song_id, timestamp, period_type) -- No duplicate entries for same period
);

-- Create indexes for price_history
CREATE INDEX idx_price_history_song_id ON price_history(fractional_song_id);
CREATE INDEX idx_price_history_timestamp ON price_history(timestamp);
CREATE INDEX idx_price_history_period ON price_history(period_type);

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
    fs.artist_reserved_shares,
    fs.fan_available_shares,
    fs.artist_revenue_percentage,
    COUNT(DISTINCT so.user_id) as unique_shareholders,
    COALESCE(SUM(so.shares_owned), 0) as shares_sold,
    COALESCE(AVG(so.purchase_price), 0) as avg_purchase_price,
    COALESCE(SUM(so.total_earnings), 0) as total_earnings_distributed,
    CASE 
        WHEN fs.fan_available_shares > 0 
        THEN ((fs.fan_available_shares - fs.available_shares)::float / fs.fan_available_shares) * 100 
        ELSE 0 
    END as fan_funding_percentage
FROM fractional_songs fs
LEFT JOIN share_ownerships so ON fs.id = so.fractional_song_id
GROUP BY fs.id, fs.title, fs.artist_id, fs.total_shares, fs.available_shares, fs.current_price_per_share, 
         fs.artist_reserved_shares, fs.fan_available_shares, fs.artist_revenue_percentage;

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

-- Sample data for testing
INSERT INTO fractional_songs (
    id, song_id, artist_id, title, total_shares, 
    artist_reserved_shares, fan_available_shares, artist_revenue_percentage,
    available_shares, current_price_per_share
) VALUES 
(
    '123e4567-e89b-12d3-a456-426614174000', 
    '223e4567-e89b-12d3-a456-426614174000', 
    '323e4567-e89b-12d3-a456-426614174000', 
    'Epic Song #1', 
    1000,
    200,  -- Artist keeps 200 shares (20%)
    800,  -- 800 shares available for fans (80%)
    0.15, -- Artist gets additional 15% of all revenue
    800,  -- All 800 fan shares still available
    10.50
);

-- Comments
COMMENT ON TABLE fractional_songs IS 'Songs available for fractional ownership with artist control';
COMMENT ON TABLE share_ownerships IS 'User ownership records for fractional songs';
COMMENT ON TABLE share_transactions IS 'Transaction history for share purchases, transfers, and sales';
COMMENT ON TABLE revenue_distributions IS 'Revenue distribution events from song earnings';
COMMENT ON TABLE individual_revenue_payouts IS 'Individual payouts to shareholders from revenue distributions';
COMMENT ON TABLE price_history IS 'Historical price data for market analytics';

COMMENT ON COLUMN fractional_songs.song_id IS 'Reference to the main Song entity from Music Context';
COMMENT ON COLUMN fractional_songs.artist_id IS 'Reference to Artist from Music Context';
COMMENT ON COLUMN fractional_songs.artist_reserved_shares IS 'Number of shares the artist keeps for themselves';
COMMENT ON COLUMN fractional_songs.fan_available_shares IS 'Number of shares available for fans to purchase';
COMMENT ON COLUMN fractional_songs.artist_revenue_percentage IS 'Additional percentage of revenue that goes to artist (beyond their share ownership)';
COMMENT ON COLUMN share_ownerships.user_id IS 'Reference to User from User Context';
COMMENT ON COLUMN share_transactions.blockchain_tx_hash IS 'Hash of the blockchain transaction for Web3 integration'; 