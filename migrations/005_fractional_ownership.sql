-- Migration: 005_fractional_ownership.sql
-- Description: Creates tables for Fractional Ownership bounded context
-- Author: AI Assistant
-- Created: 2024

-- Enable UUID extension if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- OWNERSHIP CONTRACTS TABLE
-- =====================================================
CREATE TABLE ownership_contracts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    song_id UUID NOT NULL,
    artist_id UUID NOT NULL,
    
    -- Contract configuration
    total_shares INTEGER NOT NULL CHECK (total_shares > 0),
    price_per_share DECIMAL(10,2) NOT NULL CHECK (price_per_share > 0),
    artist_retained_percentage DECIMAL(5,2) NOT NULL CHECK (artist_retained_percentage >= 1 AND artist_retained_percentage <= 99),
    
    -- Calculated fields
    shares_available_for_sale INTEGER NOT NULL CHECK (shares_available_for_sale >= 0),
    shares_sold INTEGER NOT NULL DEFAULT 0 CHECK (shares_sold >= 0),
    
    -- Investment controls
    minimum_investment DECIMAL(10,2) CHECK (minimum_investment > 0),
    maximum_ownership_per_user DECIMAL(5,2) CHECK (maximum_ownership_per_user > 0 AND maximum_ownership_per_user <= 100),
    
    -- Contract status
    contract_status VARCHAR(20) NOT NULL DEFAULT 'Draft' CHECK (contract_status IN ('Draft', 'Active', 'Paused', 'SoldOut', 'Terminated')),
    
    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    version BIGINT NOT NULL DEFAULT 1,
    
    -- Foreign key constraints (will be added when other contexts are ready)
    -- FOREIGN KEY (song_id) REFERENCES songs(id),
    -- FOREIGN KEY (artist_id) REFERENCES users(id),
    
    -- Business rules constraints
    CONSTRAINT shares_sold_not_exceed_available CHECK (shares_sold <= shares_available_for_sale),
    CONSTRAINT shares_available_calculation CHECK (shares_available_for_sale = FLOOR(total_shares * (100 - artist_retained_percentage) / 100))
);

-- =====================================================
-- FRACTIONAL SHARES TABLE
-- =====================================================
CREATE TABLE fractional_shares (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract_id UUID NOT NULL REFERENCES ownership_contracts(id) ON DELETE CASCADE,
    song_id UUID NOT NULL, -- Denormalized for performance
    owner_id UUID NOT NULL,
    
    -- Share details
    ownership_percentage DECIMAL(5,2) NOT NULL CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    purchase_price DECIMAL(10,2) NOT NULL CHECK (purchase_price > 0),
    current_market_value DECIMAL(10,2) NOT NULL CHECK (current_market_value >= 0),
    total_revenue_received DECIMAL(10,2) NOT NULL DEFAULT 0 CHECK (total_revenue_received >= 0),
    
    -- Share status
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    lock_reason VARCHAR(100),
    
    -- Vesting information
    vesting_start_date TIMESTAMPTZ,
    vesting_end_date TIMESTAMPTZ,
    
    -- Audit fields
    purchased_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Foreign key constraints
    -- FOREIGN KEY (owner_id) REFERENCES users(id),
    
    -- Business rules constraints
    CONSTRAINT vesting_dates_order CHECK (vesting_start_date IS NULL OR vesting_end_date IS NULL OR vesting_start_date < vesting_end_date),
    CONSTRAINT lock_reason_when_locked CHECK (NOT is_locked OR lock_reason IS NOT NULL)
);

-- =====================================================
-- REVENUE DISTRIBUTIONS TABLE
-- =====================================================
CREATE TABLE revenue_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract_id UUID NOT NULL REFERENCES ownership_contracts(id) ON DELETE CASCADE,
    
    -- Distribution details
    total_revenue DECIMAL(12,2) NOT NULL CHECK (total_revenue > 0),
    total_distributed DECIMAL(12,2) NOT NULL CHECK (total_distributed >= 0),
    artist_share DECIMAL(12,2) NOT NULL CHECK (artist_share >= 0),
    platform_fee DECIMAL(12,2) NOT NULL CHECK (platform_fee >= 0),
    platform_fee_percentage DECIMAL(5,2) NOT NULL CHECK (platform_fee_percentage >= 0 AND platform_fee_percentage <= 100),
    
    -- Distribution period
    distribution_period_start TIMESTAMPTZ NOT NULL,
    distribution_period_end TIMESTAMPTZ NOT NULL,
    
    -- Status
    distribution_status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (distribution_status IN ('Pending', 'Processing', 'Completed', 'Failed', 'Cancelled')),
    
    -- Audit fields
    distributed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Business rules constraints
    CONSTRAINT distribution_period_order CHECK (distribution_period_start < distribution_period_end),
    CONSTRAINT total_calculation CHECK (total_distributed + platform_fee <= total_revenue)
);

-- =====================================================
-- SHAREHOLDER DISTRIBUTIONS TABLE (Detail table)
-- =====================================================
CREATE TABLE shareholder_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    distribution_id UUID NOT NULL REFERENCES revenue_distributions(id) ON DELETE CASCADE,
    share_id UUID NOT NULL REFERENCES fractional_shares(id) ON DELETE CASCADE,
    shareholder_id UUID NOT NULL,
    
    -- Distribution amounts
    ownership_percentage_at_distribution DECIMAL(5,2) NOT NULL CHECK (ownership_percentage_at_distribution > 0),
    distribution_amount DECIMAL(10,2) NOT NULL CHECK (distribution_amount >= 0),
    
    -- Payment status
    payment_status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (payment_status IN ('Pending', 'Processing', 'Completed', 'Failed')),
    payment_reference VARCHAR(255),
    payment_processed_at TIMESTAMPTZ,
    
    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(distribution_id, share_id)
);

-- =====================================================
-- DOMAIN EVENTS TABLE (Event Sourcing)
-- =====================================================
CREATE TABLE domain_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    event_version INTEGER NOT NULL CHECK (event_version > 0),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    
    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Indexes for event sourcing
    CONSTRAINT events_aggregate_version_unique UNIQUE(aggregate_id, event_version)
);

-- =====================================================
-- EVENT OUTBOX TABLE (Outbox Pattern)
-- =====================================================
CREATE TABLE event_outbox (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    event_version INTEGER NOT NULL CHECK (event_version > 0),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Outbox specific fields
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'published', 'failed')),
    published_at TIMESTAMPTZ,
    retry_count INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    
    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =====================================================
-- SHARE TRADING HISTORY TABLE
-- =====================================================
CREATE TABLE share_trading_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    share_id UUID NOT NULL REFERENCES fractional_shares(id) ON DELETE CASCADE,
    
    -- Trading details
    from_user_id UUID NOT NULL,
    to_user_id UUID NOT NULL,
    trade_price DECIMAL(10,2) NOT NULL CHECK (trade_price > 0),
    ownership_percentage DECIMAL(5,2) NOT NULL CHECK (ownership_percentage > 0),
    
    -- Trade status
    trade_status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (trade_status IN ('Pending', 'Completed', 'Failed', 'Cancelled')),
    trade_reference VARCHAR(255),
    
    -- Audit fields
    traded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CHECK (from_user_id != to_user_id)
);

-- =====================================================
-- INDEXES FOR PERFORMANCE
-- =====================================================

-- Ownership Contracts indexes
CREATE INDEX idx_ownership_contracts_song_id ON ownership_contracts(song_id);
CREATE INDEX idx_ownership_contracts_artist_id ON ownership_contracts(artist_id);
CREATE INDEX idx_ownership_contracts_status ON ownership_contracts(contract_status);
CREATE INDEX idx_ownership_contracts_created_at ON ownership_contracts(created_at);
CREATE INDEX idx_ownership_contracts_completion ON ownership_contracts((shares_sold::float / total_shares::float));

-- Fractional Shares indexes
CREATE INDEX idx_fractional_shares_contract_id ON fractional_shares(contract_id);
CREATE INDEX idx_fractional_shares_owner_id ON fractional_shares(owner_id);
CREATE INDEX idx_fractional_shares_song_id ON fractional_shares(song_id);
CREATE INDEX idx_fractional_shares_purchased_at ON fractional_shares(purchased_at);
CREATE INDEX idx_fractional_shares_vesting ON fractional_shares(vesting_start_date, vesting_end_date) WHERE vesting_start_date IS NOT NULL;

-- Revenue Distributions indexes
CREATE INDEX idx_revenue_distributions_contract_id ON revenue_distributions(contract_id);
CREATE INDEX idx_revenue_distributions_period ON revenue_distributions(distribution_period_start, distribution_period_end);
CREATE INDEX idx_revenue_distributions_status ON revenue_distributions(distribution_status);

-- Shareholder Distributions indexes
CREATE INDEX idx_shareholder_distributions_distribution_id ON shareholder_distributions(distribution_id);
CREATE INDEX idx_shareholder_distributions_shareholder_id ON shareholder_distributions(shareholder_id);
CREATE INDEX idx_shareholder_distributions_payment_status ON shareholder_distributions(payment_status);

-- Domain Events indexes (Event Sourcing)
CREATE INDEX idx_domain_events_aggregate ON domain_events(aggregate_id, aggregate_type);
CREATE INDEX idx_domain_events_type ON domain_events(event_type);
CREATE INDEX idx_domain_events_occurred_at ON domain_events(occurred_at);
CREATE INDEX idx_domain_events_unprocessed ON domain_events(processed_at) WHERE processed_at IS NULL;

-- Event Outbox indexes (Outbox Pattern)
CREATE INDEX idx_event_outbox_status ON event_outbox(status);
CREATE INDEX idx_event_outbox_pending ON event_outbox(status, occurred_at) WHERE status = 'pending';
CREATE INDEX idx_event_outbox_aggregate ON event_outbox(aggregate_id, aggregate_type);
CREATE INDEX idx_event_outbox_occurred_at ON event_outbox(occurred_at);

-- Share Trading History indexes
CREATE INDEX idx_share_trading_history_share_id ON share_trading_history(share_id);
CREATE INDEX idx_share_trading_history_from_user ON share_trading_history(from_user_id);
CREATE INDEX idx_share_trading_history_to_user ON share_trading_history(to_user_id);
CREATE INDEX idx_share_trading_history_traded_at ON share_trading_history(traded_at);

-- =====================================================
-- TRIGGERS FOR AUTOMATIC UPDATES
-- =====================================================

-- Trigger for updating updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to all tables with updated_at
CREATE TRIGGER update_ownership_contracts_updated_at BEFORE UPDATE ON ownership_contracts FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_fractional_shares_updated_at BEFORE UPDATE ON fractional_shares FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_revenue_distributions_updated_at BEFORE UPDATE ON revenue_distributions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_shareholder_distributions_updated_at BEFORE UPDATE ON shareholder_distributions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_share_trading_history_updated_at BEFORE UPDATE ON share_trading_history FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_event_outbox_updated_at BEFORE UPDATE ON event_outbox FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =====================================================
-- MATERIALIZED VIEWS FOR ANALYTICS
-- =====================================================

-- Contract Analytics View
CREATE MATERIALIZED VIEW contract_analytics AS
SELECT 
    oc.id as contract_id,
    oc.song_id,
    oc.artist_id,
    oc.total_shares,
    oc.price_per_share,
    oc.shares_sold,
    oc.shares_available_for_sale,
    (oc.shares_sold::float / oc.total_shares::float * 100) as completion_percentage,
    (oc.total_shares * oc.price_per_share) as total_market_cap,
    COUNT(DISTINCT fs.owner_id) as unique_shareholders,
    COALESCE(SUM(fs.total_revenue_received), 0) as total_revenue_distributed,
    COALESCE(AVG(fs.current_market_value), oc.price_per_share) as avg_market_value,
    oc.created_at,
    oc.updated_at
FROM ownership_contracts oc
LEFT JOIN fractional_shares fs ON oc.id = fs.contract_id
GROUP BY oc.id, oc.song_id, oc.artist_id, oc.total_shares, oc.price_per_share, 
         oc.shares_sold, oc.shares_available_for_sale, oc.created_at, oc.updated_at;

-- Index on materialized view
CREATE UNIQUE INDEX idx_contract_analytics_contract_id ON contract_analytics(contract_id);
CREATE INDEX idx_contract_analytics_completion ON contract_analytics(completion_percentage);
CREATE INDEX idx_contract_analytics_market_cap ON contract_analytics(total_market_cap);

-- User Portfolio View
CREATE MATERIALIZED VIEW user_portfolio_analytics AS
SELECT 
    fs.owner_id,
    COUNT(DISTINCT fs.contract_id) as contracts_invested,
    COUNT(fs.id) as total_shares_owned,
    SUM(fs.ownership_percentage) as total_ownership_percentage,
    SUM(fs.purchase_price) as total_invested,
    SUM(fs.current_market_value) as current_portfolio_value,
    SUM(fs.total_revenue_received) as total_revenue_received,
    AVG(fs.current_market_value / NULLIF(fs.purchase_price, 0) * 100 - 100) as avg_roi_percentage
FROM fractional_shares fs
GROUP BY fs.owner_id;

-- Index on user portfolio view
CREATE UNIQUE INDEX idx_user_portfolio_analytics_owner_id ON user_portfolio_analytics(owner_id);
CREATE INDEX idx_user_portfolio_analytics_portfolio_value ON user_portfolio_analytics(current_portfolio_value);

-- =====================================================
-- REFRESH FUNCTION FOR MATERIALIZED VIEWS
-- =====================================================

CREATE OR REPLACE FUNCTION refresh_analytics_views()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY contract_analytics;
    REFRESH MATERIALIZED VIEW CONCURRENTLY user_portfolio_analytics;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- SAMPLE DATA (for development/testing)
-- =====================================================

-- Insert sample ownership contract
INSERT INTO ownership_contracts (
    id, song_id, artist_id, total_shares, price_per_share, 
    artist_retained_percentage, shares_available_for_sale, contract_status
) VALUES (
    '123e4567-e89b-12d3-a456-426614174000',
    '223e4567-e89b-12d3-a456-426614174001', 
    '323e4567-e89b-12d3-a456-426614174002',
    1000, 10.00, 51.0, 490, 'Active'
);

-- Insert sample fractional shares
INSERT INTO fractional_shares (
    id, contract_id, song_id, owner_id, ownership_percentage, 
    purchase_price, current_market_value
) VALUES (
    '423e4567-e89b-12d3-a456-426614174003',
    '123e4567-e89b-12d3-a456-426614174000',
    '223e4567-e89b-12d3-a456-426614174001',
    '523e4567-e89b-12d3-a456-426614174004',
    10.0, 1000.00, 1050.00
);

-- Refresh analytics views
SELECT refresh_analytics_views();

-- =====================================================
-- COMMENTS FOR DOCUMENTATION
-- =====================================================

COMMENT ON TABLE ownership_contracts IS 'Stores ownership contracts for songs, allowing fractional investment';
COMMENT ON TABLE fractional_shares IS 'Individual shares owned by users in ownership contracts';
COMMENT ON TABLE revenue_distributions IS 'Revenue distribution events for ownership contracts';
COMMENT ON TABLE shareholder_distributions IS 'Individual shareholder distributions within a revenue distribution';
COMMENT ON TABLE domain_events IS 'Event sourcing store for all domain events';
COMMENT ON TABLE event_outbox IS 'Outbox pattern for reliable event publishing to external systems';
COMMENT ON TABLE share_trading_history IS 'History of share trades between users';

COMMENT ON COLUMN ownership_contracts.artist_retained_percentage IS 'Percentage of ownership retained by the artist (1-99%)';
COMMENT ON COLUMN fractional_shares.ownership_percentage IS 'Percentage of the song owned by this share';
COMMENT ON COLUMN fractional_shares.is_locked IS 'Whether the share is locked and cannot be traded';
COMMENT ON COLUMN domain_events.event_version IS 'Version number for event ordering within aggregate';

-- =====================================================
-- FINAL VALIDATION
-- =====================================================

-- Verify all tables exist
DO $$
DECLARE
    table_count INTEGER;
BEGIN
    SELECT COUNT(*) INTO table_count 
    FROM information_schema.tables 
    WHERE table_schema = 'public' 
    AND table_name IN (
        'ownership_contracts', 'fractional_shares', 'revenue_distributions',
        'shareholder_distributions', 'domain_events', 'event_outbox', 'share_trading_history'
    );
    
    IF table_count != 7 THEN
        RAISE EXCEPTION 'Expected 7 tables, found %', table_count;
    END IF;
    
    RAISE NOTICE 'Fractional Ownership migration completed successfully! Created % tables.', table_count;
END $$; 