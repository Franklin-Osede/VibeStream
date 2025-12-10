-- Migration: 017_fan_ventures_tables.sql
-- Create missing tables for Fan Ventures bounded context

-- Create artist_ventures table
DROP TABLE IF EXISTS artist_ventures CASCADE;
CREATE TABLE IF NOT EXISTS artist_ventures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(20) DEFAULT 'other' CHECK (category IN ('music', 'visual_arts', 'film', 'gaming', 'technology', 'fashion', 'food', 'travel', 'education', 'health', 'other')),
    tags TEXT[] DEFAULT '{}',
    risk_level VARCHAR(10) DEFAULT 'medium' CHECK (risk_level IN ('low', 'medium', 'high', 'very_high')),
    expected_return DOUBLE PRECISION DEFAULT 0.0,
    artist_rating DOUBLE PRECISION DEFAULT 0.0,
    artist_previous_ventures INTEGER DEFAULT 0,
    artist_success_rate DOUBLE PRECISION DEFAULT 0.0,
    funding_goal DOUBLE PRECISION NOT NULL,
    current_funding DOUBLE PRECISION DEFAULT 0.0,
    min_investment DOUBLE PRECISION NOT NULL,
    max_investment DOUBLE PRECISION,
    status VARCHAR(20) DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'funded', 'completed', 'cancelled')),
    start_date TIMESTAMP WITH TIME ZONE,
    end_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create fan_investments table
DROP TABLE IF EXISTS fan_investments CASCADE;
CREATE TABLE IF NOT EXISTS fan_investments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fan_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    investment_amount DOUBLE PRECISION NOT NULL,
    investment_type VARCHAR(20) DEFAULT 'equity' CHECK (investment_type IN ('equity', 'revenue_share', 'donation', 'loan')),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'cancelled', 'refunded')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(fan_id, venture_id)
);

-- Create venture_tiers table
DROP TABLE IF EXISTS venture_tiers CASCADE;
CREATE TABLE IF NOT EXISTS venture_tiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL, -- "Bronze", "Silver", "Gold", "Platinum"
    min_investment DOUBLE PRECISION NOT NULL,
    max_investment DOUBLE PRECISION,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT venture_tiers_min_investment_check CHECK (min_investment > 0),
    CONSTRAINT venture_tiers_max_investment_check CHECK (max_investment IS NULL OR max_investment > min_investment)
);

-- Create venture_benefits table
DROP TABLE IF EXISTS venture_benefits CASCADE;
CREATE TABLE IF NOT EXISTS venture_benefits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    tier_id UUID REFERENCES venture_tiers(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    benefit_type VARCHAR(50) NOT NULL CHECK (benefit_type IN ('exclusive_content', 'meet_greet', 'merchandise', 'concert_tickets', 'revenue_share', 'voting_rights', 'early_access')),
    delivery_method VARCHAR(20) DEFAULT 'manual' CHECK (delivery_method IN ('automatic', 'manual', 'physical', 'experience')),
    estimated_delivery_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create benefit_deliveries table
DROP TABLE IF EXISTS benefit_deliveries CASCADE;
CREATE TABLE IF NOT EXISTS benefit_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    benefit_id UUID NOT NULL REFERENCES venture_benefits(id) ON DELETE CASCADE,
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    fan_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tier_id UUID REFERENCES venture_tiers(id) ON DELETE SET NULL,
    delivery_status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (delivery_status IN ('pending', 'in_progress', 'delivered', 'failed', 'cancelled')),
    delivery_method VARCHAR(20) NOT NULL DEFAULT 'manual' CHECK (delivery_method IN ('automatic', 'manual', 'physical', 'experience')),
    delivery_date TIMESTAMP WITH TIME ZONE,
    tracking_number VARCHAR(100),
    carrier VARCHAR(50),
    estimated_delivery TIMESTAMP WITH TIME ZONE,
    actual_delivery TIMESTAMP WITH TIME ZONE,
    delivery_notes TEXT,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create fan_preferences table
DROP TABLE IF EXISTS fan_preferences CASCADE;
CREATE TABLE IF NOT EXISTS fan_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fan_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    favorite_categories TEXT[] DEFAULT '{}',
    preferred_investment_types TEXT[] DEFAULT '{}',
    risk_tolerance VARCHAR(10) DEFAULT 'medium' CHECK (risk_tolerance IN ('low', 'medium', 'high', 'very_high')),
    min_investment DOUBLE PRECISION DEFAULT 0.0,
    max_investment DOUBLE PRECISION DEFAULT 10000.0,
    favorite_artists UUID[] DEFAULT '{}',
    interests TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(fan_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_artist_ventures_artist_id ON artist_ventures(artist_id);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_status ON artist_ventures(status);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_category ON artist_ventures(category);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_risk_level ON artist_ventures(risk_level);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_artist_rating ON artist_ventures(artist_rating);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_expected_return ON artist_ventures(expected_return);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_tags ON artist_ventures USING GIN(tags);

CREATE INDEX IF NOT EXISTS idx_fan_investments_fan_id ON fan_investments(fan_id);
CREATE INDEX IF NOT EXISTS idx_fan_investments_venture_id ON fan_investments(venture_id);
CREATE INDEX IF NOT EXISTS idx_fan_investments_status ON fan_investments(status);

CREATE INDEX IF NOT EXISTS idx_venture_tiers_venture_id ON venture_tiers(venture_id);
CREATE INDEX IF NOT EXISTS idx_venture_tiers_min_investment ON venture_tiers(min_investment);

CREATE INDEX IF NOT EXISTS idx_venture_benefits_venture_id ON venture_benefits(venture_id);
CREATE INDEX IF NOT EXISTS idx_venture_benefits_tier_id ON venture_benefits(tier_id);
CREATE INDEX IF NOT EXISTS idx_venture_benefits_benefit_type ON venture_benefits(benefit_type);

CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_benefit_id ON benefit_deliveries(benefit_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_venture_id ON benefit_deliveries(venture_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_fan_id ON benefit_deliveries(fan_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_tier_id ON benefit_deliveries(tier_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_status ON benefit_deliveries(delivery_status);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_created_at ON benefit_deliveries(created_at);

CREATE INDEX IF NOT EXISTS idx_fan_preferences_fan_id ON fan_preferences(fan_id);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_favorite_categories ON fan_preferences USING GIN(favorite_categories);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_preferred_investment_types ON fan_preferences USING GIN(preferred_investment_types);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_favorite_artists ON fan_preferences USING GIN(favorite_artists);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_interests ON fan_preferences USING GIN(interests);

-- Create triggers to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_artist_ventures_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_fan_investments_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_venture_benefits_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_benefit_deliveries_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_fan_preferences_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply triggers
CREATE TRIGGER trigger_update_artist_ventures_updated_at
    BEFORE UPDATE ON artist_ventures
    FOR EACH ROW
    EXECUTE FUNCTION update_artist_ventures_updated_at();

CREATE TRIGGER trigger_update_fan_investments_updated_at
    BEFORE UPDATE ON fan_investments
    FOR EACH ROW
    EXECUTE FUNCTION update_fan_investments_updated_at();

CREATE TRIGGER trigger_update_venture_benefits_updated_at
    BEFORE UPDATE ON venture_benefits
    FOR EACH ROW
    EXECUTE FUNCTION update_venture_benefits_updated_at();

CREATE TRIGGER trigger_update_benefit_deliveries_updated_at
    BEFORE UPDATE ON benefit_deliveries
    FOR EACH ROW
    EXECUTE FUNCTION update_benefit_deliveries_updated_at();

CREATE TRIGGER trigger_update_fan_preferences_updated_at
    BEFORE UPDATE ON fan_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_fan_preferences_updated_at();

-- Add comments for documentation
COMMENT ON TABLE artist_ventures IS 'Ventures created by artists for fan investment';
COMMENT ON TABLE fan_investments IS 'Investments made by fans in artist ventures';
COMMENT ON TABLE venture_tiers IS 'Investment tiers for ventures with their associated benefits';
COMMENT ON TABLE venture_benefits IS 'Benefits offered to investors in ventures';
COMMENT ON TABLE benefit_deliveries IS 'Tracks the delivery of benefits to fans';
COMMENT ON TABLE fan_preferences IS 'Stores fan preferences for venture recommendations'; 