-- Migration to change BigDecimal columns to f64 for Fan Ventures
-- This aligns the database schema with our simplified f64 approach
-- SKIPPED: Tables will be created in migration 012_complete_vibestream_schema.sql

-- -- Change artist_ventures table columns
-- ALTER TABLE artist_ventures 
--     ALTER COLUMN min_investment TYPE DOUBLE PRECISION,
--     ALTER COLUMN max_investment TYPE DOUBLE PRECISION,
--     ALTER COLUMN total_goal TYPE DOUBLE PRECISION,
--     ALTER COLUMN current_amount TYPE DOUBLE PRECISION;

-- -- Change fan_investments table columns
-- ALTER TABLE fan_investments 
--     ALTER COLUMN investment_amount TYPE DOUBLE PRECISION,
--     ALTER COLUMN expected_return TYPE DOUBLE PRECISION;

-- -- Update constraints to work with f64
-- ALTER TABLE artist_ventures 
--     DROP CONSTRAINT IF EXISTS artist_ventures_current_amount_check;

-- ALTER TABLE artist_ventures 
--     ADD CONSTRAINT artist_ventures_current_amount_check 
--     CHECK (current_amount >= 0.0);

-- ALTER TABLE artist_ventures 
--     DROP CONSTRAINT IF EXISTS artist_ventures_total_goal_check;

-- ALTER TABLE artist_ventures 
--     ADD CONSTRAINT artist_ventures_total_goal_check 
--     CHECK (total_goal > 0.0);

-- ALTER TABLE artist_ventures 
--     DROP CONSTRAINT IF EXISTS artist_ventures_min_investment_check;

-- ALTER TABLE artist_ventures 
--     ADD CONSTRAINT artist_ventures_min_investment_check 
--     CHECK (min_investment > 0.0);

-- -- Add comments for documentation
-- COMMENT ON COLUMN artist_ventures.min_investment IS 'Minimum investment amount in f64 format';
-- COMMENT ON COLUMN artist_ventures.max_investment IS 'Maximum investment amount in f64 format (nullable)';
-- COMMENT ON COLUMN artist_ventures.total_goal IS 'Total funding goal in f64 format';
-- COMMENT ON COLUMN artist_ventures.current_amount IS 'Current amount raised in f64 format';
-- COMMENT ON COLUMN fan_investments.investment_amount IS 'Investment amount in f64 format';
-- COMMENT ON COLUMN fan_investments.expected_return IS 'Expected return amount in f64 format'; 