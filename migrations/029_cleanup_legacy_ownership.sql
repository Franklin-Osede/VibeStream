-- Migration: 029_cleanup_legacy_ownership.sql
-- Description: Drop legacy Fractional Ownership tables that were replaced by Fan Ventures
-- Date: 2026-01-17

-- Drop triggers first
DROP TRIGGER IF EXISTS audit_trigger_ownership_contracts ON ownership_contracts;
DROP TRIGGER IF EXISTS trigger_update_contract_stats ON user_shares;

-- Drop tables in dependency order
DROP TABLE IF EXISTS revenue_distributions;
DROP TABLE IF EXISTS share_transactions;
DROP TABLE IF EXISTS user_shares;
DROP TABLE IF EXISTS ownership_contracts;

-- Note: 'revenue_distributions' is used in Fan Ventures code but as a TODO.
-- The table schema was for the old module. Fan Ventures will need a new table 
-- structure matching its specific needs when implemented.
