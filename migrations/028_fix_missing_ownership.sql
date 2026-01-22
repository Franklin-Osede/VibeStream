-- Repair Script: Restore missing Fractional Ownership tables from Migration 012

-- Ownership contracts table
CREATE TABLE IF NOT EXISTS ownership_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    artist_id UUID NOT NULL REFERENCES users(id),
    contract_name VARCHAR(200) NOT NULL,
    total_shares INTEGER NOT NULL,
    shares_available INTEGER NOT NULL,
    price_per_share DECIMAL(10,4) NOT NULL,
    artist_retained_percentage DECIMAL(5,2) NOT NULL,
    minimum_investment DECIMAL(10,4),
    vesting_period_months INTEGER,
    status VARCHAR(20) DEFAULT 'active',
    total_invested DECIMAL(12,4) DEFAULT 0.0,
    investor_count INTEGER DEFAULT 0,
    monthly_revenue DECIMAL(10,4) DEFAULT 0.0,
    total_revenue DECIMAL(12,4) DEFAULT 0.0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User share holdings
CREATE TABLE IF NOT EXISTS user_shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    contract_id UUID NOT NULL REFERENCES ownership_contracts(id) ON DELETE CASCADE,
    shares_owned INTEGER NOT NULL,
    initial_investment DECIMAL(10,4) NOT NULL,
    current_value DECIMAL(10,4) NOT NULL,
    revenue_earned DECIMAL(10,4) DEFAULT 0.0,
    purchase_date TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, contract_id)
);

-- Share transactions (buy/sell)
CREATE TABLE IF NOT EXISTS share_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contract_id UUID NOT NULL REFERENCES ownership_contracts(id),
    buyer_id UUID REFERENCES users(id),
    seller_id UUID REFERENCES users(id),
    shares_quantity INTEGER NOT NULL,
    price_per_share DECIMAL(10,4) NOT NULL,
    total_amount DECIMAL(10,4) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL, -- 'purchase', 'trade', 'transfer'
    payment_method VARCHAR(50),
    blockchain_tx_hash VARCHAR(100),
    status VARCHAR(20) DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Revenue distributions
CREATE TABLE IF NOT EXISTS revenue_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contract_id UUID NOT NULL REFERENCES ownership_contracts(id),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    total_revenue DECIMAL(12,4) NOT NULL,
    platform_fee DECIMAL(10,4) NOT NULL,
    artist_share DECIMAL(10,4) NOT NULL,
    investor_share DECIMAL(10,4) NOT NULL,
    investors_paid INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'pending',
    distributed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Re-apply indexes (from 012)
CREATE INDEX IF NOT EXISTS idx_ownership_contracts_song_id ON ownership_contracts(song_id);
CREATE INDEX IF NOT EXISTS idx_ownership_contracts_artist_id ON ownership_contracts(artist_id);
CREATE INDEX IF NOT EXISTS idx_ownership_contracts_status ON ownership_contracts(status);
CREATE INDEX IF NOT EXISTS idx_user_shares_user_id ON user_shares(user_id);
CREATE INDEX IF NOT EXISTS idx_user_shares_contract_id ON user_shares(contract_id);

-- Re-apply Audit Trigger (from 027)
DROP TRIGGER IF EXISTS audit_trigger_ownership_contracts ON ownership_contracts;
CREATE TRIGGER audit_trigger_ownership_contracts
    AFTER INSERT OR UPDATE OR DELETE ON ownership_contracts
    FOR EACH ROW EXECUTE FUNCTION system_audit_trigger();
