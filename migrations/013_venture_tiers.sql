-- Migration to create venture_tiers table and modify venture_benefits
-- This enables artists to configure benefits by investment tiers

-- Create venture_tiers table
-- CREATE TABLE IF NOT EXISTS venture_tiers (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
--     name VARCHAR(100) NOT NULL, -- "Bronze", "Silver", "Gold", "Platinum"
--     min_investment DOUBLE PRECISION NOT NULL,
--     max_investment DOUBLE PRECISION,
--     description TEXT,
--     created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
--     
--     -- Constraints
--     CONSTRAINT venture_tiers_min_investment_check CHECK (min_investment > 0),
--     CONSTRAINT venture_tiers_max_investment_check CHECK (max_investment IS NULL OR max_investment > min_investment)
-- );

-- Create indexes separately
-- CREATE INDEX IF NOT EXISTS idx_venture_tiers_venture_id ON venture_tiers(venture_id);
-- CREATE INDEX IF NOT EXISTS idx_venture_tiers_min_investment ON venture_tiers(min_investment);

-- Add tier_id column to venture_benefits
-- ALTER TABLE venture_benefits 
-- ADD COLUMN tier_id UUID REFERENCES venture_tiers(id) ON DELETE CASCADE;

-- Add delivery_method column to venture_benefits
-- ALTER TABLE venture_benefits 
-- ADD COLUMN delivery_method VARCHAR(50) DEFAULT 'manual';

-- Add constraint for delivery_method
-- ALTER TABLE venture_benefits
-- ADD CONSTRAINT venture_benefits_delivery_method_check 
-- CHECK (delivery_method IN ('automatic', 'manual', 'physical', 'experience'));

-- Add comments for documentation
-- COMMENT ON TABLE venture_tiers IS 'Investment tiers for ventures with their associated benefits';
-- COMMENT ON COLUMN venture_tiers.name IS 'Tier name (Bronze, Silver, Gold, Platinum)';
-- COMMENT ON COLUMN venture_tiers.min_investment IS 'Minimum investment required for this tier';
-- COMMENT ON COLUMN venture_tiers.max_investment IS 'Maximum investment for this tier (NULL = unlimited)';
-- COMMENT ON COLUMN venture_benefits.tier_id IS 'Reference to the tier this benefit belongs to';
-- COMMENT ON COLUMN venture_benefits.delivery_method IS 'How this benefit is delivered to fans'; 