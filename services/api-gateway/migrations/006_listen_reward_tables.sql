-- Listen Reward Context Tables
-- Migration: 006_listen_reward_tables.sql
-- Description: Create tables for storing listen sessions, reward distributions, and related analytics

-- =====================================
-- LISTEN SESSIONS TABLE
-- =====================================
CREATE TABLE listen_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    song_id UUID NOT NULL,
    artist_id UUID NOT NULL,
    user_tier VARCHAR(20) NOT NULL CHECK (user_tier IN ('basic', 'premium', 'vip', 'artist')),
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'completed', 'verified', 'rewarded', 'failed', 'deleted')),
    listen_duration_seconds INTEGER,
    quality_score DECIMAL(3,2) CHECK (quality_score >= 0 AND quality_score <= 1),
    zk_proof_hash VARCHAR(64),
    base_reward_tokens DECIMAL(10,4) CHECK (base_reward_tokens >= 0),
    final_reward_tokens DECIMAL(10,4) CHECK (final_reward_tokens >= 0),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    verified_at TIMESTAMPTZ,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_listen_sessions_user_id ON listen_sessions(user_id);
CREATE INDEX idx_listen_sessions_song_id ON listen_sessions(song_id);
CREATE INDEX idx_listen_sessions_artist_id ON listen_sessions(artist_id);
CREATE INDEX idx_listen_sessions_status ON listen_sessions(status);
CREATE INDEX idx_listen_sessions_started_at ON listen_sessions(started_at);
CREATE INDEX idx_listen_sessions_user_started ON listen_sessions(user_id, started_at);
CREATE INDEX idx_listen_sessions_status_verified ON listen_sessions(status, verified_at) WHERE status = 'verified';

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_listen_sessions_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_listen_sessions_updated_at
    BEFORE UPDATE ON listen_sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_listen_sessions_updated_at();

-- =====================================
-- REWARD DISTRIBUTIONS TABLE
-- =====================================
CREATE TABLE reward_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pool_id UUID NOT NULL,
    total_tokens DECIMAL(15,4) NOT NULL CHECK (total_tokens >= 0),
    distributed_tokens DECIMAL(15,4) NOT NULL DEFAULT 0 CHECK (distributed_tokens >= 0),
    reserved_tokens DECIMAL(15,4) NOT NULL DEFAULT 0 CHECK (reserved_tokens >= 0),
    validation_period_start TIMESTAMPTZ NOT NULL,
    validation_period_end TIMESTAMPTZ NOT NULL,
    pending_distributions_count INTEGER NOT NULL DEFAULT 0,
    completed_distributions_count INTEGER NOT NULL DEFAULT 0,
    events_json JSONB,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_distribution_period CHECK (validation_period_end > validation_period_start),
    CONSTRAINT chk_distribution_tokens CHECK (distributed_tokens + reserved_tokens <= total_tokens)
);

-- Indexes for reward distributions
CREATE INDEX idx_reward_distributions_pool_id ON reward_distributions(pool_id);
CREATE INDEX idx_reward_distributions_period ON reward_distributions(validation_period_start, validation_period_end);
-- CREATE INDEX idx_reward_distributions_active ON reward_distributions(validation_period_end) WHERE validation_period_end > NOW();
CREATE INDEX idx_reward_distributions_pending ON reward_distributions(pending_distributions_count) WHERE pending_distributions_count > 0;

-- Trigger for reward distributions updated_at
CREATE OR REPLACE FUNCTION update_reward_distributions_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_reward_distributions_updated_at
    BEFORE UPDATE ON reward_distributions
    FOR EACH ROW
    EXECUTE FUNCTION update_reward_distributions_updated_at();

-- =====================================
-- ANALYTICS VIEWS
-- =====================================

-- View for user reward history analytics
CREATE VIEW user_reward_analytics AS
SELECT 
    ls.user_id,
    ls.song_id,
    ls.artist_id,
    ls.final_reward_tokens as reward_amount,
    ls.quality_score,
    ls.listen_duration_seconds,
    ls.verified_at as earned_at,
    'listen_session' as transaction_type
FROM listen_sessions ls
WHERE ls.status = 'verified' 
AND ls.final_reward_tokens IS NOT NULL;

-- View for artist revenue analytics
CREATE VIEW artist_revenue_analytics AS
SELECT 
    ls.artist_id,
    DATE_TRUNC('day', ls.verified_at) as date,
    COUNT(*) as session_count,
    COUNT(DISTINCT ls.user_id) as unique_listeners,
    SUM(ls.final_reward_tokens) as total_revenue,
    AVG(ls.final_reward_tokens) as avg_reward_per_session,
    AVG(ls.listen_duration_seconds) as avg_listen_duration,
    AVG(ls.quality_score) as avg_quality_score
FROM listen_sessions ls
WHERE ls.status = 'verified' 
AND ls.final_reward_tokens IS NOT NULL
GROUP BY ls.artist_id, DATE_TRUNC('day', ls.verified_at);

-- View for song performance metrics
CREATE VIEW song_metrics AS
SELECT 
    ls.song_id,
    ls.artist_id,
    COUNT(*) as total_listens,
    COUNT(DISTINCT ls.user_id) as unique_listeners,
    SUM(ls.final_reward_tokens) as total_rewards_paid,
    AVG(ls.listen_duration_seconds) as avg_listen_duration,
    AVG(ls.quality_score) as avg_quality_score,
    COUNT(CASE WHEN ls.status = 'completed' THEN 1 END)::FLOAT / COUNT(*)::FLOAT as completion_rate
FROM listen_sessions ls
WHERE ls.status IN ('completed', 'verified', 'rewarded')
GROUP BY ls.song_id, ls.artist_id;

-- View for platform statistics
CREATE VIEW platform_statistics AS
SELECT 
    DATE_TRUNC('day', ls.created_at) as date,
    COUNT(*) as total_sessions,
    COUNT(DISTINCT ls.user_id) as unique_users,
    COUNT(DISTINCT ls.artist_id) as unique_artists,
    COUNT(DISTINCT ls.song_id) as unique_songs,
    SUM(CASE WHEN ls.final_reward_tokens IS NOT NULL THEN ls.final_reward_tokens ELSE 0 END) as total_rewards_distributed,
    AVG(ls.listen_duration_seconds) as avg_session_duration,
    COUNT(CASE WHEN ls.status = 'verified' THEN 1 END)::FLOAT / 
        COUNT(CASE WHEN ls.zk_proof_hash IS NOT NULL THEN 1 END)::FLOAT as zk_proof_success_rate
FROM listen_sessions ls
WHERE ls.status != 'deleted'
GROUP BY DATE_TRUNC('day', ls.created_at);

-- =====================================
-- EVENT OUTBOX TABLE
-- =====================================
CREATE TABLE IF NOT EXISTS event_outbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(50) NOT NULL,
    event_data JSONB NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed BOOLEAN NOT NULL DEFAULT FALSE,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for event processing
CREATE INDEX idx_event_outbox_unprocessed ON event_outbox(processed, occurred_at) WHERE NOT processed;
CREATE INDEX idx_event_outbox_aggregate ON event_outbox(aggregate_type, aggregate_id);
CREATE INDEX idx_event_outbox_type ON event_outbox(event_type);

-- =====================================
-- SECURITY AND PERMISSIONS
-- =====================================

-- Create roles if they don't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'vibestream_api') THEN
        CREATE ROLE vibestream_api;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'vibestream_analytics') THEN
        CREATE ROLE vibestream_analytics;
    END IF;
END $$;

-- Grant permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON listen_sessions TO vibestream_api;
GRANT SELECT, INSERT, UPDATE, DELETE ON reward_distributions TO vibestream_api;
GRANT SELECT, INSERT, UPDATE ON event_outbox TO vibestream_api;

-- Analytics role (read-only)
GRANT SELECT ON listen_sessions TO vibestream_analytics;
GRANT SELECT ON reward_distributions TO vibestream_analytics;
GRANT SELECT ON user_reward_analytics TO vibestream_analytics;
GRANT SELECT ON artist_revenue_analytics TO vibestream_analytics;
GRANT SELECT ON song_metrics TO vibestream_analytics;
GRANT SELECT ON platform_statistics TO vibestream_analytics;

-- Grant sequence permissions
GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO vibestream_api;

-- =====================================
-- SAMPLE DATA FOR TESTING
-- =====================================

-- Insert sample reward pool for testing
INSERT INTO reward_distributions (
    pool_id,
    total_tokens,
    validation_period_start,
    validation_period_end,
    created_at
) VALUES (
    gen_random_uuid(),
    10000.0000,
    NOW(),
    NOW() + INTERVAL '7 days',
    NOW()
) ON CONFLICT DO NOTHING;

COMMENT ON TABLE listen_sessions IS 'Stores individual listening sessions with ZK proof verification and reward calculations';
COMMENT ON TABLE reward_distributions IS 'Manages reward pool distributions and tracks pending/completed payouts';
COMMENT ON TABLE event_outbox IS 'Event sourcing outbox pattern for reliable event publishing';
COMMENT ON VIEW user_reward_analytics IS 'User reward history for analytics and reporting';
COMMENT ON VIEW artist_revenue_analytics IS 'Artist revenue aggregated by day for analytics';
COMMENT ON VIEW song_metrics IS 'Song performance metrics including completion rates and rewards';
COMMENT ON VIEW platform_statistics IS 'Platform-wide daily statistics and KPIs'; 