-- Migration: 008_payment_tables.sql
-- Description: Creates tables for Payment bounded context
-- Author: AI Assistant
-- Created: 2024

-- Enable UUID extension if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================================================
-- PAYMENTS TABLE
-- =====================================================
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID,
    payer_id UUID NOT NULL,
    payee_id UUID NOT NULL,
    
    -- Amount and currency
    amount_value DECIMAL(15,6) NOT NULL CHECK (amount_value >= 0),
    amount_currency VARCHAR(10) NOT NULL CHECK (amount_currency IN ('USD', 'ETH', 'SOL', 'USDC', 'VIBES')),
    
    -- Payment method details
    payment_method_type VARCHAR(50) NOT NULL CHECK (payment_method_type IN ('CreditCard', 'Cryptocurrency', 'PlatformBalance', 'BankTransfer')),
    payment_method_details JSONB NOT NULL,
    
    -- Payment purpose
    purpose_type VARCHAR(50) NOT NULL,
    purpose_details JSONB NOT NULL,
    
    -- Status tracking
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Processing', 'Completed', 'Failed', 'Cancelled', 'Refunding', 'Refunded')),
    status_details JSONB,
    
    -- Blockchain information
    blockchain_hash VARCHAR(128),
    blockchain_network VARCHAR(20),
    
    -- Fee information
    platform_fee_value DECIMAL(15,6) DEFAULT 0 CHECK (platform_fee_value >= 0),
    platform_fee_currency VARCHAR(10),
    net_amount_value DECIMAL(15,6) NOT NULL CHECK (net_amount_value >= 0),
    net_amount_currency VARCHAR(10) NOT NULL,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    failure_reason TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    
    -- Audit fields
    version BIGINT NOT NULL DEFAULT 1,
    
    -- Constraints
    CONSTRAINT payments_currency_consistency CHECK (amount_currency = net_amount_currency),
    CONSTRAINT payments_platform_fee_currency CHECK (platform_fee_currency IS NULL OR platform_fee_currency = amount_currency),
    CONSTRAINT payments_amounts_consistency CHECK (net_amount_value <= amount_value)
);

-- =====================================================
-- ROYALTY DISTRIBUTIONS TABLE
-- =====================================================
CREATE TABLE royalty_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    song_id UUID NOT NULL,
    artist_id UUID NOT NULL,
    
    -- Revenue amounts
    total_revenue_value DECIMAL(15,6) NOT NULL CHECK (total_revenue_value > 0),
    total_revenue_currency VARCHAR(10) NOT NULL,
    artist_share_percentage DECIMAL(5,2) NOT NULL CHECK (artist_share_percentage >= 0 AND artist_share_percentage <= 100),
    platform_fee_percentage DECIMAL(5,2) NOT NULL CHECK (platform_fee_percentage >= 0 AND platform_fee_percentage <= 100),
    
    -- Calculated amounts
    artist_amount_value DECIMAL(15,6) NOT NULL CHECK (artist_amount_value >= 0),
    artist_amount_currency VARCHAR(10) NOT NULL,
    platform_fee_value DECIMAL(15,6) NOT NULL CHECK (platform_fee_value >= 0),
    platform_fee_currency VARCHAR(10) NOT NULL,
    
    -- Period
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Processing', 'Completed', 'Failed', 'PartiallyCompleted')),
    
    -- Related payments
    payment_ids JSONB DEFAULT '[]',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    distributed_at TIMESTAMPTZ,
    
    -- Constraints
    CONSTRAINT royalty_distributions_period_order CHECK (period_end > period_start),
    CONSTRAINT royalty_distributions_percentage_sum CHECK (artist_share_percentage + platform_fee_percentage <= 100),
    CONSTRAINT royalty_distributions_currency_consistency CHECK (
        total_revenue_currency = artist_amount_currency AND
        artist_amount_currency = platform_fee_currency
    )
);

-- =====================================================
-- REVENUE SHARING DISTRIBUTIONS TABLE
-- =====================================================
CREATE TABLE revenue_sharing_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract_id UUID NOT NULL,
    song_id UUID NOT NULL,
    
    -- Revenue information
    total_revenue_value DECIMAL(15,6) NOT NULL CHECK (total_revenue_value > 0),
    total_revenue_currency VARCHAR(10) NOT NULL,
    platform_fee_percentage DECIMAL(5,2) NOT NULL CHECK (platform_fee_percentage >= 0 AND platform_fee_percentage <= 100),
    
    -- Distribution period
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'Created' CHECK (status IN ('Created', 'Processing', 'Completed', 'Failed', 'PartiallyCompleted')),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    -- Audit
    version BIGINT NOT NULL DEFAULT 1,
    
    -- Constraints
    CONSTRAINT revenue_sharing_period_order CHECK (period_end > period_start)
);

-- =====================================================
-- SHAREHOLDER DISTRIBUTIONS TABLE (Detail)
-- =====================================================
CREATE TABLE shareholder_distributions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    distribution_id UUID NOT NULL REFERENCES revenue_sharing_distributions(id) ON DELETE CASCADE,
    shareholder_id UUID NOT NULL,
    
    -- Ownership details
    ownership_percentage DECIMAL(8,4) NOT NULL CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    distribution_amount_value DECIMAL(15,6) NOT NULL CHECK (distribution_amount_value >= 0),
    distribution_amount_currency VARCHAR(10) NOT NULL,
    
    -- Payment status
    payment_status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (payment_status IN ('Pending', 'Processing', 'Completed', 'Failed')),
    payment_id UUID,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(distribution_id, shareholder_id)
);

-- =====================================================
-- PAYMENT BATCHES TABLE
-- =====================================================
CREATE TABLE payment_batches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    batch_type VARCHAR(30) NOT NULL CHECK (batch_type IN ('RoyaltyDistribution', 'RevenueSharing', 'ListenRewards', 'Refunds')),
    
    -- Batch details
    total_amount_value DECIMAL(15,6) NOT NULL CHECK (total_amount_value >= 0),
    total_amount_currency VARCHAR(10) NOT NULL,
    payment_count INTEGER NOT NULL DEFAULT 0 CHECK (payment_count >= 0),
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'Created' CHECK (status IN ('Created', 'Processing', 'Completed', 'Failed', 'PartiallyCompleted')),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    -- Processing metrics
    successful_payments INTEGER DEFAULT 0 CHECK (successful_payments >= 0),
    failed_payments INTEGER DEFAULT 0 CHECK (failed_payments >= 0),
    processing_duration_ms BIGINT
);

-- =====================================================
-- PAYMENT BATCH ITEMS TABLE
-- =====================================================
CREATE TABLE payment_batch_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    batch_id UUID NOT NULL REFERENCES payment_batches(id) ON DELETE CASCADE,
    payment_id UUID NOT NULL,
    
    -- Item details
    amount_value DECIMAL(15,6) NOT NULL CHECK (amount_value >= 0),
    amount_currency VARCHAR(10) NOT NULL,
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Processing', 'Completed', 'Failed')),
    error_message TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    
    -- Constraints
    UNIQUE(batch_id, payment_id)
);

-- =====================================================
-- PAYMENT EVENTS TABLE (Event Sourcing)
-- =====================================================
CREATE TABLE payment_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL CHECK (aggregate_type IN ('Payment', 'RoyaltyDistribution', 'RevenueSharing')),
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    event_version INTEGER NOT NULL CHECK (event_version > 0),
    
    -- Timestamps
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Event sourcing constraints
    CONSTRAINT payment_events_aggregate_version_unique UNIQUE(aggregate_id, event_version)
);

-- =====================================================
-- FRAUD ALERTS TABLE
-- =====================================================
CREATE TABLE fraud_alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_id UUID NOT NULL,
    user_id UUID NOT NULL,
    
    -- Risk assessment
    risk_score DECIMAL(5,4) NOT NULL CHECK (risk_score >= 0 AND risk_score <= 1),
    fraud_indicators JSONB NOT NULL DEFAULT '[]',
    
    -- Action taken
    action_taken VARCHAR(50) NOT NULL CHECK (action_taken IN ('Allow', 'Review', 'Block', 'RequireVerification')),
    
    -- Review status
    review_status VARCHAR(20) DEFAULT 'Pending' CHECK (review_status IN ('Pending', 'Reviewed', 'Resolved', 'Escalated')),
    reviewed_by UUID,
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT fraud_alerts_payment_unique UNIQUE(payment_id)
);

-- =====================================================
-- PAYMENT ANALYTICS MATERIALIZED VIEW
-- =====================================================
CREATE MATERIALIZED VIEW payment_analytics AS
SELECT 
    DATE_TRUNC('day', created_at) as date,
    status,
    amount_currency,
    purpose_type,
    payment_method_type,
    COUNT(*) as payment_count,
    SUM(amount_value) as total_volume,
    AVG(amount_value) as average_amount,
    SUM(platform_fee_value) as total_fees,
    COUNT(CASE WHEN status = 'Completed' THEN 1 END)::FLOAT / COUNT(*)::FLOAT as success_rate
FROM payments
WHERE created_at >= CURRENT_DATE - INTERVAL '365 days'
GROUP BY DATE_TRUNC('day', created_at), status, amount_currency, purpose_type, payment_method_type;

-- =====================================================
-- USER PAYMENT SUMMARY VIEW
-- =====================================================
CREATE MATERIALIZED VIEW user_payment_summary AS
SELECT 
    payer_id as user_id,
    'payer' as role,
    COUNT(*) as total_payments,
    SUM(amount_value) as total_volume,
    AVG(amount_value) as average_payment,
    MIN(created_at) as first_payment_date,
    MAX(created_at) as last_payment_date,
    COUNT(CASE WHEN status = 'Completed' THEN 1 END) as successful_payments,
    COUNT(CASE WHEN status = 'Failed' THEN 1 END) as failed_payments
FROM payments
GROUP BY payer_id

UNION ALL

SELECT 
    payee_id as user_id,
    'payee' as role,
    COUNT(*) as total_payments,
    SUM(amount_value) as total_volume,
    AVG(amount_value) as average_payment,
    MIN(created_at) as first_payment_date,
    MAX(created_at) as last_payment_date,
    COUNT(CASE WHEN status = 'Completed' THEN 1 END) as successful_payments,
    COUNT(CASE WHEN status = 'Failed' THEN 1 END) as failed_payments
FROM payments
GROUP BY payee_id;

-- =====================================================
-- INDEXES FOR PERFORMANCE
-- =====================================================

-- Payments table indexes
CREATE INDEX idx_payments_payer_id ON payments(payer_id);
CREATE INDEX idx_payments_payee_id ON payments(payee_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_created_at ON payments(created_at);
CREATE INDEX idx_payments_updated_at ON payments(updated_at);
CREATE INDEX idx_payments_completed_at ON payments(completed_at) WHERE completed_at IS NOT NULL;
CREATE INDEX idx_payments_transaction_id ON payments(transaction_id) WHERE transaction_id IS NOT NULL;
CREATE INDEX idx_payments_blockchain_hash ON payments(blockchain_hash) WHERE blockchain_hash IS NOT NULL;
CREATE INDEX idx_payments_purpose_type ON payments(purpose_type);
CREATE INDEX idx_payments_payment_method_type ON payments(payment_method_type);
CREATE INDEX idx_payments_amount_currency ON payments(amount_currency);
CREATE INDEX idx_payments_amount_value ON payments(amount_value);

-- Composite indexes for common queries
CREATE INDEX idx_payments_payer_status_date ON payments(payer_id, status, created_at);
CREATE INDEX idx_payments_payee_status_date ON payments(payee_id, status, created_at);
CREATE INDEX idx_payments_status_date ON payments(status, created_at);
CREATE INDEX idx_payments_currency_date ON payments(amount_currency, created_at);
CREATE INDEX idx_payments_purpose_date ON payments(purpose_type, created_at);

-- Royalty distributions indexes
CREATE INDEX idx_royalty_distributions_song_id ON royalty_distributions(song_id);
CREATE INDEX idx_royalty_distributions_artist_id ON royalty_distributions(artist_id);
CREATE INDEX idx_royalty_distributions_status ON royalty_distributions(status);
CREATE INDEX idx_royalty_distributions_period ON royalty_distributions(period_start, period_end);
CREATE INDEX idx_royalty_distributions_created_at ON royalty_distributions(created_at);

-- Revenue sharing distributions indexes
CREATE INDEX idx_revenue_sharing_contract_id ON revenue_sharing_distributions(contract_id);
CREATE INDEX idx_revenue_sharing_song_id ON revenue_sharing_distributions(song_id);
CREATE INDEX idx_revenue_sharing_status ON revenue_sharing_distributions(status);
CREATE INDEX idx_revenue_sharing_period ON revenue_sharing_distributions(period_start, period_end);

-- Shareholder distributions indexes
CREATE INDEX idx_shareholder_distributions_distribution_id ON shareholder_distributions(distribution_id);
CREATE INDEX idx_shareholder_distributions_shareholder_id ON shareholder_distributions(shareholder_id);
CREATE INDEX idx_shareholder_distributions_payment_status ON shareholder_distributions(payment_status);

-- Payment batches indexes
CREATE INDEX idx_payment_batches_batch_type ON payment_batches(batch_type);
CREATE INDEX idx_payment_batches_status ON payment_batches(status);
CREATE INDEX idx_payment_batches_created_at ON payment_batches(created_at);

-- Payment batch items indexes
CREATE INDEX idx_payment_batch_items_batch_id ON payment_batch_items(batch_id);
CREATE INDEX idx_payment_batch_items_payment_id ON payment_batch_items(payment_id);
CREATE INDEX idx_payment_batch_items_status ON payment_batch_items(status);

-- Payment events indexes (Event Sourcing)
CREATE INDEX idx_payment_events_aggregate ON payment_events(aggregate_id, aggregate_type);
CREATE INDEX idx_payment_events_type ON payment_events(event_type);
CREATE INDEX idx_payment_events_occurred_at ON payment_events(occurred_at);
CREATE INDEX idx_payment_events_version ON payment_events(aggregate_id, event_version);

-- Fraud alerts indexes
CREATE INDEX idx_fraud_alerts_payment_id ON fraud_alerts(payment_id);
CREATE INDEX idx_fraud_alerts_user_id ON fraud_alerts(user_id);
CREATE INDEX idx_fraud_alerts_risk_score ON fraud_alerts(risk_score);
CREATE INDEX idx_fraud_alerts_action_taken ON fraud_alerts(action_taken);
CREATE INDEX idx_fraud_alerts_review_status ON fraud_alerts(review_status);
CREATE INDEX idx_fraud_alerts_created_at ON fraud_alerts(created_at);

-- Materialized view indexes
CREATE UNIQUE INDEX idx_payment_analytics_unique ON payment_analytics(date, status, amount_currency, purpose_type, payment_method_type);
CREATE INDEX idx_payment_analytics_date ON payment_analytics(date);
CREATE INDEX idx_payment_analytics_currency ON payment_analytics(amount_currency);
CREATE INDEX idx_payment_analytics_purpose ON payment_analytics(purpose_type);

CREATE INDEX idx_user_payment_summary_user_id ON user_payment_summary(user_id);
CREATE INDEX idx_user_payment_summary_role ON user_payment_summary(role);
CREATE INDEX idx_user_payment_summary_total_volume ON user_payment_summary(total_volume);

-- =====================================================
-- TRIGGERS FOR AUTOMATIC UPDATES
-- =====================================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_payment_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    NEW.version = OLD.version + 1;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_payments_updated_at 
    BEFORE UPDATE ON payments 
    FOR EACH ROW EXECUTE FUNCTION update_payment_updated_at();

CREATE TRIGGER update_shareholder_distributions_updated_at 
    BEFORE UPDATE ON shareholder_distributions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to refresh materialized views
CREATE OR REPLACE FUNCTION refresh_payment_analytics()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY payment_analytics;
    REFRESH MATERIALIZED VIEW CONCURRENTLY user_payment_summary;
END;
$$ LANGUAGE plpgsql;

-- =====================================================
-- SAMPLE DATA (for development/testing)
-- =====================================================

-- Insert sample payment
INSERT INTO payments (
    id, payer_id, payee_id, amount_value, amount_currency,
    payment_method_type, payment_method_details, purpose_type, purpose_details,
    net_amount_value, net_amount_currency, platform_fee_value, platform_fee_currency,
    status, metadata
) VALUES (
    '723e4567-e89b-12d3-a456-426614174000',
    '823e4567-e89b-12d3-a456-426614174001',
    '923e4567-e89b-12d3-a456-426614174002',
    100.00, 'USD',
    'PlatformBalance', '{"balance_type": "main"}',
    'NFTPurchase', '{"campaign_id": "a23e4567-e89b-12d3-a456-426614174003", "nft_quantity": 1}',
    97.50, 'USD', 2.50, 'USD',
    'Completed',
    '{"user_ip": "127.0.0.1", "platform_version": "1.0.0"}'
);

-- Insert sample royalty distribution
INSERT INTO royalty_distributions (
    id, song_id, artist_id, total_revenue_value, total_revenue_currency,
    artist_share_percentage, platform_fee_percentage,
    artist_amount_value, artist_amount_currency,
    platform_fee_value, platform_fee_currency,
    period_start, period_end, status
) VALUES (
    'b23e4567-e89b-12d3-a456-426614174004',
    'c23e4567-e89b-12d3-a456-426614174005',
    'd23e4567-e89b-12d3-a456-426614174006',
    1000.00, 'USD',
    85.0, 10.0,
    850.00, 'USD',
    100.00, 'USD',
    '2024-01-01'::timestamptz,
    '2024-01-31'::timestamptz,
    'Completed'
);

-- Refresh materialized views
SELECT refresh_payment_analytics();

-- =====================================================
-- COMMENTS FOR DOCUMENTATION
-- =====================================================

COMMENT ON TABLE payments IS 'Core payments table storing all payment transactions in the system';
COMMENT ON TABLE royalty_distributions IS 'Royalty distributions to artists for their songs';
COMMENT ON TABLE revenue_sharing_distributions IS 'Revenue sharing distributions for fractional ownership contracts';
COMMENT ON TABLE shareholder_distributions IS 'Individual shareholder distributions within revenue sharing';
COMMENT ON TABLE payment_batches IS 'Batch processing of multiple payments for efficiency';
COMMENT ON TABLE payment_events IS 'Event sourcing store for payment domain events';
COMMENT ON TABLE fraud_alerts IS 'Fraud detection alerts and risk assessments';

COMMENT ON COLUMN payments.purpose_type IS 'Type of payment: NFTPurchase, SharePurchase, ListenReward, etc.';
COMMENT ON COLUMN payments.payment_method_type IS 'Payment method: CreditCard, Cryptocurrency, PlatformBalance, BankTransfer';
COMMENT ON COLUMN payments.status IS 'Current payment status in the processing lifecycle';
COMMENT ON COLUMN fraud_alerts.risk_score IS 'Fraud risk score from 0.0 (low risk) to 1.0 (high risk)';

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
        'payments', 'royalty_distributions', 'revenue_sharing_distributions',
        'shareholder_distributions', 'payment_batches', 'payment_batch_items',
        'payment_events', 'fraud_alerts'
    );
    
    IF table_count != 8 THEN
        RAISE EXCEPTION 'Expected 8 tables, found %', table_count;
    END IF;
    
    RAISE NOTICE 'Payment Context migration completed successfully! Created % tables.', table_count;
END $$; 