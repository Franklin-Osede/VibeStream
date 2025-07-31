-- VibeStream Complete Database Schema
-- This migration creates all tables needed for the complete VibeStream platform

-- =====================================
-- 1. CORE USER MANAGEMENT
-- =====================================

-- Enhanced users table with wallet and tier info
ALTER TABLE users ADD COLUMN IF NOT EXISTS wallet_address VARCHAR(255);
ALTER TABLE users ADD COLUMN IF NOT EXISTS tier VARCHAR(20) DEFAULT 'free';
ALTER TABLE users ADD COLUMN IF NOT EXISTS total_rewards_earned DECIMAL(10,4) DEFAULT 0.0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS current_balance DECIMAL(10,4) DEFAULT 0.0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS listening_streak_days INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS avatar_url VARCHAR(500);
ALTER TABLE users ADD COLUMN IF NOT EXISTS bio TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS is_verified BOOLEAN DEFAULT false;

-- User achievements table
CREATE TABLE IF NOT EXISTS user_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    achievement_id VARCHAR(100) NOT NULL,
    achievement_name VARCHAR(200) NOT NULL,
    description TEXT,
    reward_points INTEGER DEFAULT 0,
    rarity VARCHAR(20) DEFAULT 'common',
    unlocked_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, achievement_id)
);

-- User tier progress table
CREATE TABLE IF NOT EXISTS user_tier_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    current_tier VARCHAR(20) NOT NULL DEFAULT 'free',
    current_points INTEGER DEFAULT 0,
    points_to_next_tier INTEGER DEFAULT 100,
    tier_since TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =====================================
-- 2. ENHANCED SONGS AND MUSIC
-- =====================================

-- Enhanced songs table
ALTER TABLE songs ADD COLUMN IF NOT EXISTS genre VARCHAR(100);
ALTER TABLE songs ADD COLUMN IF NOT EXISTS royalty_percentage DECIMAL(5,2) DEFAULT 10.0;
ALTER TABLE songs ADD COLUMN IF NOT EXISTS play_count INTEGER DEFAULT 0;
ALTER TABLE songs ADD COLUMN IF NOT EXISTS revenue_generated DECIMAL(10,4) DEFAULT 0.0;
ALTER TABLE songs ADD COLUMN IF NOT EXISTS metadata JSONB;
ALTER TABLE songs ADD COLUMN IF NOT EXISTS cover_art_url VARCHAR(500);
ALTER TABLE songs ADD COLUMN IF NOT EXISTS is_explicit BOOLEAN DEFAULT false;
ALTER TABLE songs ADD COLUMN IF NOT EXISTS release_date DATE;

-- Song analytics table
CREATE TABLE IF NOT EXISTS song_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    song_id UUID NOT NULL REFERENCES songs(id) ON DELETE CASCADE,
    total_listens INTEGER DEFAULT 0,
    unique_listeners INTEGER DEFAULT 0,
    total_listen_time_seconds BIGINT DEFAULT 0,
    average_completion_rate DECIMAL(5,2) DEFAULT 0.0,
    revenue_per_play DECIMAL(8,4) DEFAULT 0.0,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =====================================
-- 3. FRACTIONAL OWNERSHIP SYSTEM
-- =====================================

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

-- =====================================
-- 4. CAMPAIGN & NFT SYSTEM
-- =====================================

-- Campaigns table
CREATE TABLE IF NOT EXISTS campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    song_id UUID NOT NULL REFERENCES songs(id),
    artist_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    nft_price DECIMAL(10,4) NOT NULL,
    max_nfts INTEGER NOT NULL,
    nfts_sold INTEGER DEFAULT 0,
    boost_multiplier DECIMAL(5,2) DEFAULT 1.0,
    target_revenue DECIMAL(12,4),
    current_revenue DECIMAL(12,4) DEFAULT 0.0,
    artwork_ipfs_hash VARCHAR(100),
    nft_contract_address VARCHAR(100),
    status VARCHAR(20) DEFAULT 'upcoming',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- NFT purchases
CREATE TABLE IF NOT EXISTS nft_purchases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    user_id UUID NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    total_cost DECIMAL(10,4) NOT NULL,
    payment_method VARCHAR(50) NOT NULL,
    nft_ids TEXT[], -- Array of NFT token IDs
    boost_expires_at TIMESTAMP WITH TIME ZONE,
    blockchain_tx_hash VARCHAR(100),
    status VARCHAR(20) DEFAULT 'pending',
    purchase_date TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Campaign analytics
CREATE TABLE IF NOT EXISTS campaign_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    date DATE NOT NULL,
    nfts_sold_daily INTEGER DEFAULT 0,
    revenue_daily DECIMAL(10,4) DEFAULT 0.0,
    unique_buyers_daily INTEGER DEFAULT 0,
    total_buyers INTEGER DEFAULT 0,
    conversion_rate DECIMAL(5,2) DEFAULT 0.0,
    social_shares INTEGER DEFAULT 0,
    social_likes INTEGER DEFAULT 0,
    social_comments INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(campaign_id, date)
);

-- =====================================
-- 5. LISTEN REWARD SYSTEM
-- =====================================

-- Listen sessions
CREATE TABLE IF NOT EXISTS listen_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    song_id UUID NOT NULL REFERENCES songs(id),
    device_type VARCHAR(50),
    platform VARCHAR(50),
    app_version VARCHAR(20),
    location_country VARCHAR(10),
    location_city VARCHAR(100),
    boost_multiplier DECIMAL(5,2) DEFAULT 1.0,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    listen_duration_seconds INTEGER,
    completion_percentage DECIMAL(5,2),
    quality_score DECIMAL(5,2),
    expected_reward DECIMAL(8,4),
    final_reward DECIMAL(8,4),
    zk_proof_hash VARCHAR(100),
    verification_status VARCHAR(20) DEFAULT 'pending',
    session_token VARCHAR(100),
    engagement_metrics JSONB,
    status VARCHAR(20) DEFAULT 'active'
);

-- Reward distributions
CREATE TABLE IF NOT EXISTS reward_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    total_reward_pool DECIMAL(12,4) NOT NULL,
    total_distributed DECIMAL(12,4) NOT NULL,
    users_rewarded INTEGER NOT NULL,
    sessions_processed INTEGER NOT NULL,
    distribution_type VARCHAR(20) NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User reward history
CREATE TABLE IF NOT EXISTS user_reward_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    session_id UUID REFERENCES listen_sessions(id),
    distribution_id UUID REFERENCES reward_distributions(id),
    reward_amount DECIMAL(8,4) NOT NULL,
    reward_type VARCHAR(50) NOT NULL, -- 'listen', 'quality_bonus', 'tier_bonus', etc.
    earned_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =====================================
-- 6. PAYMENT SYSTEM (Enhanced)
-- =====================================

-- Enhanced payments table (building on existing)
ALTER TABLE payments ADD COLUMN IF NOT EXISTS purpose_type VARCHAR(50);
ALTER TABLE payments ADD COLUMN IF NOT EXISTS purpose_id UUID;
ALTER TABLE payments ADD COLUMN IF NOT EXISTS blockchain_network VARCHAR(20);
ALTER TABLE payments ADD COLUMN IF NOT EXISTS gas_fee DECIMAL(10,4);
ALTER TABLE payments ADD COLUMN IF NOT EXISTS exchange_rate DECIMAL(10,6);
ALTER TABLE payments ADD COLUMN IF NOT EXISTS metadata JSONB;

-- Payment methods
CREATE TABLE IF NOT EXISTS payment_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    method_type VARCHAR(50) NOT NULL, -- 'credit_card', 'crypto_wallet', 'bank_account'
    provider VARCHAR(50) NOT NULL, -- 'stripe', 'metamask', 'coinbase'
    external_id VARCHAR(100),
    details JSONB, -- Encrypted payment details
    is_default BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =====================================
-- 7. NOTIFICATIONS SYSTEM
-- =====================================

-- User notifications
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(50) NOT NULL, -- 'reward', 'payment', 'campaign', 'system'
    related_id UUID, -- Can reference any related entity
    is_read BOOLEAN DEFAULT false,
    action_url VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    read_at TIMESTAMP WITH TIME ZONE
);

-- =====================================
-- 8. ANALYTICS AND REPORTING
-- =====================================

-- Daily platform statistics
CREATE TABLE IF NOT EXISTS daily_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL UNIQUE,
    total_users INTEGER DEFAULT 0,
    active_users INTEGER DEFAULT 0,
    new_users INTEGER DEFAULT 0,
    total_songs INTEGER DEFAULT 0,
    total_listens INTEGER DEFAULT 0,
    total_revenue DECIMAL(12,4) DEFAULT 0.0,
    total_rewards_distributed DECIMAL(10,4) DEFAULT 0.0,
    nfts_sold INTEGER DEFAULT 0,
    shares_traded INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =====================================
-- 9. INDEXES FOR PERFORMANCE
-- =====================================

-- User indexes
CREATE INDEX IF NOT EXISTS idx_users_wallet_address ON users(wallet_address);
CREATE INDEX IF NOT EXISTS idx_users_tier ON users(tier);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Song indexes
CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
CREATE INDEX IF NOT EXISTS idx_songs_genre ON songs(genre);
CREATE INDEX IF NOT EXISTS idx_songs_play_count ON songs(play_count DESC);

-- Ownership contract indexes
CREATE INDEX IF NOT EXISTS idx_ownership_contracts_song_id ON ownership_contracts(song_id);
CREATE INDEX IF NOT EXISTS idx_ownership_contracts_artist_id ON ownership_contracts(artist_id);
CREATE INDEX IF NOT EXISTS idx_ownership_contracts_status ON ownership_contracts(status);

-- User shares indexes
CREATE INDEX IF NOT EXISTS idx_user_shares_user_id ON user_shares(user_id);
CREATE INDEX IF NOT EXISTS idx_user_shares_contract_id ON user_shares(contract_id);

-- Campaign indexes
CREATE INDEX IF NOT EXISTS idx_campaigns_song_id ON campaigns(song_id);
CREATE INDEX IF NOT EXISTS idx_campaigns_artist_id ON campaigns(artist_id);
CREATE INDEX IF NOT EXISTS idx_campaigns_status ON campaigns(status);
CREATE INDEX IF NOT EXISTS idx_campaigns_end_date ON campaigns(end_date);

-- Listen session indexes
CREATE INDEX IF NOT EXISTS idx_listen_sessions_user_id ON listen_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_song_id ON listen_sessions(song_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_started_at ON listen_sessions(started_at);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_status ON listen_sessions(status);

-- Payment indexes
CREATE INDEX IF NOT EXISTS idx_payments_payer_id ON payments(payer_id);
CREATE INDEX IF NOT EXISTS idx_payments_purpose_type ON payments(purpose_type);
CREATE INDEX IF NOT EXISTS idx_payments_created_at ON payments(created_at);

-- Notification indexes
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
CREATE INDEX IF NOT EXISTS idx_notifications_is_read ON notifications(is_read);

-- =====================================
-- 10. TRIGGERS AND FUNCTIONS
-- =====================================

-- Function to update user tier based on points
CREATE OR REPLACE FUNCTION update_user_tier()
RETURNS TRIGGER AS $$
BEGIN
    -- Update tier based on total points
    IF NEW.current_points >= 1000 THEN
        NEW.current_tier = 'vip';
        NEW.points_to_next_tier = 0;
    ELSIF NEW.current_points >= 500 THEN
        NEW.current_tier = 'premium';  
        NEW.points_to_next_tier = 1000 - NEW.current_points;
    ELSE
        NEW.current_tier = 'free';
        NEW.points_to_next_tier = 500 - NEW.current_points;
    END IF;
    
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for user tier updates
DROP TRIGGER IF EXISTS trigger_update_user_tier ON user_tier_progress;
CREATE TRIGGER trigger_update_user_tier
    BEFORE UPDATE ON user_tier_progress
    FOR EACH ROW
    EXECUTE FUNCTION update_user_tier();

-- Function to update song play count
CREATE OR REPLACE FUNCTION increment_song_play_count()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'completed' AND OLD.status != 'completed' THEN
        UPDATE songs 
        SET play_count = play_count + 1,
            updated_at = NOW()
        WHERE id = NEW.song_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for song play count
DROP TRIGGER IF EXISTS trigger_increment_play_count ON listen_sessions;
CREATE TRIGGER trigger_increment_play_count
    AFTER UPDATE ON listen_sessions
    FOR EACH ROW
    EXECUTE FUNCTION increment_song_play_count();

-- Function to update ownership contract stats
CREATE OR REPLACE FUNCTION update_contract_stats()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE ownership_contracts
    SET total_invested = (
        SELECT COALESCE(SUM(initial_investment), 0)
        FROM user_shares
        WHERE contract_id = NEW.contract_id
    ),
    investor_count = (
        SELECT COUNT(DISTINCT user_id)
        FROM user_shares
        WHERE contract_id = NEW.contract_id
    ),
    shares_available = total_shares - (
        SELECT COALESCE(SUM(shares_owned), 0)
        FROM user_shares
        WHERE contract_id = NEW.contract_id
    ),
    updated_at = NOW()
    WHERE id = NEW.contract_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for ownership contract stats
DROP TRIGGER IF EXISTS trigger_update_contract_stats ON user_shares;
CREATE TRIGGER trigger_update_contract_stats
    AFTER INSERT OR UPDATE OR DELETE ON user_shares
    FOR EACH ROW
    EXECUTE FUNCTION update_contract_stats();

-- =====================================
-- 11. INSERT SAMPLE DATA
-- =====================================

-- Insert sample achievements
INSERT INTO user_achievements (user_id, achievement_id, achievement_name, description, reward_points, rarity)
SELECT id, 'first_login', 'Welcome to VibeStream', 'Completed first login', 10, 'common'
FROM users 
WHERE email IN ('demo@example.com', 'artist@example.com')
ON CONFLICT (user_id, achievement_id) DO NOTHING;

-- Insert sample tier progress
INSERT INTO user_tier_progress (user_id, current_tier, current_points)
SELECT id, 'free', 0
FROM users
ON CONFLICT DO NOTHING;

-- Insert sample ownership contract
-- INSERT INTO ownership_contracts (
--     song_id, artist_id, contract_name, total_shares, shares_available, 
--     price_per_share, artist_retained_percentage
-- )
-- SELECT 
--     s.id, s.artist_id, s.title || ' - Ownership Contract', 1000, 800,
--     10.0, 20.0
-- FROM songs s
-- LIMIT 1
-- ON CONFLICT DO NOTHING;

-- Insert sample campaign
-- INSERT INTO campaigns (
--     song_id, artist_id, name, description, start_date, end_date,
--     nft_price, max_nfts, boost_multiplier
-- )
-- SELECT 
--     s.id, s.artist_id, s.title || ' - NFT Drop', 'Limited edition NFTs with listening boosts',
--     NOW(), NOW() + INTERVAL '30 days', 25.0, 1000, 2.0
-- FROM songs s
-- LIMIT 1
-- ON CONFLICT DO NOTHING;

COMMIT; 