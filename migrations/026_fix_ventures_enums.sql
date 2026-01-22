-- Migration: 026_fix_ventures_enums.sql
-- Description: Aligning database CHECK constraints with Rust Enums for Fan Ventures
-- Date: 2026-01-17

-- =============================================================================
-- 1. FIX ARTIST VENTURES STATUS
-- =============================================================================
-- Rust Enum: Draft, Open, Closed, Cancelled
-- Current DB: draft, active, funded, completed, cancelled
-- Goal: draft, open, closed, cancelled

-- First, map old values to new values (migration strategy)
UPDATE artist_ventures 
SET status = 'open' 
WHERE status = 'active';

UPDATE artist_ventures 
SET status = 'closed' 
WHERE status IN ('funded', 'completed');

-- Drop old constraint
ALTER TABLE artist_ventures 
DROP CONSTRAINT IF EXISTS artist_ventures_status_check;

-- Add new constraint
ALTER TABLE artist_ventures 
ADD CONSTRAINT artist_ventures_status_check 
CHECK (status IN ('draft', 'open', 'closed', 'cancelled'));

COMMENT ON CONSTRAINT artist_ventures_status_check ON artist_ventures IS 
'Status aligned with Rust Enum: Draft, Open, Closed, Cancelled';

-- =============================================================================
-- 2. FIX FAN INVESTMENTS STATUS
-- =============================================================================
-- Rust Enum: Pending, Active, Completed, Cancelled
-- Current DB: pending, confirmed, cancelled, refunded
-- Goal: pending, active, completed, cancelled, refunded (keeping refunded for safety)

-- Map old values to new values
UPDATE fan_investments 
SET status = 'completed' 
WHERE status = 'confirmed';

-- Drop old constraint
ALTER TABLE fan_investments 
DROP CONSTRAINT IF EXISTS fan_investments_status_check;

-- Add new constraint
ALTER TABLE fan_investments 
ADD CONSTRAINT fan_investments_status_check 
CHECK (status IN ('pending', 'active', 'completed', 'cancelled', 'refunded'));

COMMENT ON CONSTRAINT fan_investments_status_check ON fan_investments IS 
'Status aligned with Rust Enum: Pending, Active, Completed, Cancelled';

-- =============================================================================
-- 3. FIX INVESTMENT TYPE (Optional alignment)
-- =============================================================================
-- Rust Enum: EarlyAccess, ExclusiveContent, Merchandise, ConcertTickets, MeetAndGreet, RevenueShare, Custom
-- Current DB: equity, revenue_share, donation, loan
-- Goal: Update check to support Rust types

ALTER TABLE fan_investments 
DROP CONSTRAINT IF EXISTS fan_investments_investment_type_check;

ALTER TABLE fan_investments 
ADD CONSTRAINT fan_investments_investment_type_check 
CHECK (investment_type IN (
    'early_access', 
    'exclusive_content', 
    'merchandise', 
    'concert_tickets', 
    'meet_and_greet', 
    'revenue_share', 
    'custom',
    'equity', -- Keeping legacy types just in case
    'donation',
    'loan'
));
