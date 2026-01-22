-- Migration: 031_revenue_and_benefits.sql
-- Description: Create tables for Revenue Distributions and Benefit Deliveries (Fan Ventures)
-- Date: 2026-01-18

-- 1. Revenue Distributions Table
CREATE TABLE IF NOT EXISTS revenue_distributions (
    id UUID PRIMARY KEY,
    venture_id UUID NOT NULL REFERENCES artist_ventures(id),
    total_revenue DECIMAL(15, 2) NOT NULL,
    artist_share DECIMAL(15, 2) NOT NULL,
    fan_share DECIMAL(15, 2) NOT NULL,
    platform_fee DECIMAL(15, 2) NOT NULL,
    distributed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_rev_dist_venture_id ON revenue_distributions(venture_id);
CREATE INDEX IF NOT EXISTS idx_rev_dist_distributed_at ON revenue_distributions(distributed_at);

-- 2. Benefit Deliveries Table
CREATE TABLE IF NOT EXISTS benefit_deliveries (
    id UUID PRIMARY KEY,
    benefit_id UUID NOT NULL REFERENCES venture_benefits(id),
    venture_id UUID NOT NULL REFERENCES artist_ventures(id),
    fan_id UUID NOT NULL REFERENCES users(id),
    tier_id UUID REFERENCES venture_tiers(id), -- Optional validation
    delivery_status VARCHAR(50) NOT NULL, -- Pending, InProgress, Delivered, Failed, Cancelled
    delivery_method VARCHAR(50) NOT NULL,
    delivery_date TIMESTAMP WITH TIME ZONE,
    tracking_info JSONB, -- Stores TrackingInfo struct
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ben_del_fan_id ON benefit_deliveries(fan_id);
CREATE INDEX IF NOT EXISTS idx_ben_del_venture_id ON benefit_deliveries(venture_id);
CREATE INDEX IF NOT EXISTS idx_ben_del_status ON benefit_deliveries(delivery_status);

-- Trigger for updated_at
CREATE TRIGGER update_benefit_deliveries_updated_at
    BEFORE UPDATE ON benefit_deliveries
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
