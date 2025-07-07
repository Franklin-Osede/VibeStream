-- Migration 009: Event Outbox Tables for HybridEventBus
-- 
-- This migration creates the necessary tables for the event sourcing
-- and outbox pattern used by the HybridEventBus system.
-- 
-- The outbox pattern ensures reliable event delivery by storing events
-- in the same database transaction as business operations.

-- =============================================================================
-- EVENT OUTBOX TABLE
-- =============================================================================

-- Main event outbox table for reliable event delivery
CREATE TABLE IF NOT EXISTS event_outbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    event_version INTEGER NOT NULL DEFAULT 1,
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'published', 'failed', 'dead_letter')),
    published_at TIMESTAMP WITH TIME ZONE,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    error_message TEXT,
    routing_key VARCHAR(255),
    correlation_id UUID,
    causation_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_event_outbox_status ON event_outbox (status);
CREATE INDEX IF NOT EXISTS idx_event_outbox_aggregate ON event_outbox (aggregate_id, aggregate_type);
CREATE INDEX IF NOT EXISTS idx_event_outbox_event_type ON event_outbox (event_type);
CREATE INDEX IF NOT EXISTS idx_event_outbox_occurred_at ON event_outbox (occurred_at);
CREATE INDEX IF NOT EXISTS idx_event_outbox_pending ON event_outbox (status, occurred_at) WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_event_outbox_retry ON event_outbox (retry_count, status) WHERE status = 'failed';

-- =============================================================================
-- EVENT STORE TABLE (for complete event sourcing)
-- =============================================================================

-- Event store for complete event history and replay capability
CREATE TABLE IF NOT EXISTS event_store (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stream_id UUID NOT NULL,
    stream_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_data JSONB NOT NULL,
    metadata JSONB,
    version INTEGER NOT NULL,
    sequence_number BIGSERIAL,
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    correlation_id UUID,
    causation_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for event store
CREATE UNIQUE INDEX IF NOT EXISTS idx_event_store_stream_version ON event_store (stream_id, version);
CREATE INDEX IF NOT EXISTS idx_event_store_stream ON event_store (stream_id, stream_type);
CREATE INDEX IF NOT EXISTS idx_event_store_event_type ON event_store (event_type);
CREATE INDEX IF NOT EXISTS idx_event_store_sequence ON event_store (sequence_number);
CREATE INDEX IF NOT EXISTS idx_event_store_occurred_at ON event_store (occurred_at);

-- =============================================================================
-- EVENT SUBSCRIPTIONS TABLE (for event consumers)
-- =============================================================================

-- Track event subscriptions and consumer positions
CREATE TABLE IF NOT EXISTS event_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_name VARCHAR(255) NOT NULL UNIQUE,
    consumer_group VARCHAR(255) NOT NULL,
    event_types TEXT[] NOT NULL,
    last_processed_sequence BIGINT NOT NULL DEFAULT 0,
    last_processed_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'paused', 'stopped', 'error')),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for subscriptions
CREATE INDEX IF NOT EXISTS idx_event_subscriptions_consumer_group ON event_subscriptions (consumer_group);
CREATE INDEX IF NOT EXISTS idx_event_subscriptions_status ON event_subscriptions (status);

-- =============================================================================
-- SNAPSHOT STORE TABLE (for CQRS optimization)
-- =============================================================================

-- Store aggregate snapshots for performance optimization
CREATE TABLE IF NOT EXISTS snapshot_store (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    version INTEGER NOT NULL,
    data JSONB NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for snapshots
CREATE UNIQUE INDEX IF NOT EXISTS idx_snapshot_store_aggregate_version ON snapshot_store (aggregate_id, version);
CREATE INDEX IF NOT EXISTS idx_snapshot_store_aggregate ON snapshot_store (aggregate_id, aggregate_type);
CREATE INDEX IF NOT EXISTS idx_snapshot_store_created_at ON snapshot_store (created_at);

-- =============================================================================
-- TRIGGERS FOR AUTOMATIC TIMESTAMP UPDATES
-- =============================================================================

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language plpgsql;

-- Triggers for automatic timestamp updates
CREATE TRIGGER update_event_outbox_updated_at
    BEFORE UPDATE ON event_outbox
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_event_subscriptions_updated_at
    BEFORE UPDATE ON event_subscriptions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- FUNCTIONS FOR EVENT PROCESSING
-- =============================================================================

-- Function to get pending events for processing
CREATE OR REPLACE FUNCTION get_pending_events(batch_size INTEGER DEFAULT 100)
RETURNS SETOF event_outbox AS $$
BEGIN
    RETURN QUERY
    SELECT * FROM event_outbox
    WHERE status = 'pending'
       OR (status = 'failed' AND retry_count < max_retries)
    ORDER BY occurred_at ASC, created_at ASC
    LIMIT batch_size;
END;
$$ LANGUAGE plpgsql;

-- Function to mark event as published
CREATE OR REPLACE FUNCTION mark_event_published(event_id UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE event_outbox
    SET status = 'published',
        published_at = NOW(),
        updated_at = NOW()
    WHERE id = event_id;
END;
$$ LANGUAGE plpgsql;

-- Function to mark event as failed
CREATE OR REPLACE FUNCTION mark_event_failed(event_id UUID, error_msg TEXT)
RETURNS VOID AS $$
BEGIN
    UPDATE event_outbox
    SET status = CASE
        WHEN retry_count >= max_retries THEN 'dead_letter'
        ELSE 'failed'
    END,
    retry_count = retry_count + 1,
    error_message = error_msg,
    updated_at = NOW()
    WHERE id = event_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get events for a specific aggregate
CREATE OR REPLACE FUNCTION get_events_for_aggregate(agg_id UUID, agg_type VARCHAR(100))
RETURNS SETOF event_store AS $$
BEGIN
    RETURN QUERY
    SELECT * FROM event_store
    WHERE stream_id = agg_id
      AND stream_type = agg_type
    ORDER BY version ASC;
END;
$$ LANGUAGE plpgsql;

-- Function to get events since a specific version
CREATE OR REPLACE FUNCTION get_events_since_version(agg_id UUID, since_version INTEGER)
RETURNS SETOF event_store AS $$
BEGIN
    RETURN QUERY
    SELECT * FROM event_store
    WHERE stream_id = agg_id
      AND version > since_version
    ORDER BY version ASC;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- INDEXES FOR PERFORMANCE OPTIMIZATION
-- =============================================================================

-- Additional composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_event_outbox_aggregate_status ON event_outbox (aggregate_id, status);
CREATE INDEX IF NOT EXISTS idx_event_outbox_event_type_status ON event_outbox (event_type, status);
CREATE INDEX IF NOT EXISTS idx_event_store_stream_type_occurred ON event_store (stream_type, occurred_at);
CREATE INDEX IF NOT EXISTS idx_event_store_event_type_occurred ON event_store (event_type, occurred_at);

-- Partial indexes for active subscriptions
CREATE INDEX IF NOT EXISTS idx_event_subscriptions_active ON event_subscriptions (subscription_name, last_processed_sequence) WHERE status = 'active';

-- =============================================================================
-- CLEANUP FUNCTIONS
-- =============================================================================

-- Function to cleanup old published events (for maintenance)
CREATE OR REPLACE FUNCTION cleanup_old_events(older_than_days INTEGER DEFAULT 90)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM event_outbox
    WHERE status = 'published'
      AND published_at < NOW() - INTERVAL '1 day' * older_than_days;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get event processing statistics
CREATE OR REPLACE FUNCTION get_event_processing_stats()
RETURNS TABLE (
    status VARCHAR(20),
    count BIGINT,
    oldest_event TIMESTAMP WITH TIME ZONE,
    newest_event TIMESTAMP WITH TIME ZONE
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        eo.status,
        COUNT(*) as count,
        MIN(eo.occurred_at) as oldest_event,
        MAX(eo.occurred_at) as newest_event
    FROM event_outbox eo
    GROUP BY eo.status;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- GRANT PERMISSIONS
-- =============================================================================

-- Grant permissions to the application user
-- (Adjust the username as needed for your environment)
GRANT SELECT, INSERT, UPDATE, DELETE ON event_outbox TO vibestream;
GRANT SELECT, INSERT, UPDATE, DELETE ON event_store TO vibestream;
GRANT SELECT, INSERT, UPDATE, DELETE ON event_subscriptions TO vibestream;
GRANT SELECT, INSERT, UPDATE, DELETE ON snapshot_store TO vibestream;
GRANT USAGE ON SEQUENCE event_store_sequence_number_seq TO vibestream;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO vibestream;

-- =============================================================================
-- COMMENTS FOR DOCUMENTATION
-- =============================================================================

COMMENT ON TABLE event_outbox IS 'Outbox pattern table for reliable event delivery';
COMMENT ON TABLE event_store IS 'Complete event store for event sourcing and replay';
COMMENT ON TABLE event_subscriptions IS 'Event consumer subscriptions and positions';
COMMENT ON TABLE snapshot_store IS 'Aggregate snapshots for CQRS optimization';

COMMENT ON COLUMN event_outbox.status IS 'Event processing status: pending, processing, published, failed, dead_letter';
COMMENT ON COLUMN event_outbox.retry_count IS 'Number of processing attempts';
COMMENT ON COLUMN event_outbox.max_retries IS 'Maximum number of retry attempts before dead letter';
COMMENT ON COLUMN event_store.sequence_number IS 'Global sequence number for event ordering';
COMMENT ON COLUMN event_store.version IS 'Aggregate version number';
COMMENT ON COLUMN event_subscriptions.last_processed_sequence IS 'Last processed event sequence number';

-- =============================================================================
-- INITIAL DATA
-- =============================================================================

-- Create initial subscription for system events
INSERT INTO event_subscriptions (subscription_name, consumer_group, event_types)
VALUES 
    ('system-events', 'system-processors', ARRAY['SystemHealthCheck', 'SystemMetrics']),
    ('financial-events', 'financial-processors', ARRAY['SharesPurchased', 'SharesTraded', 'RevenueDistributed']),
    ('analytics-events', 'analytics-processors', ARRAY['ListenSessionCompleted', 'UserInteraction', 'ContentViewed'])
ON CONFLICT (subscription_name) DO NOTHING;

-- =============================================================================
-- MIGRATION COMPLETE
-- =============================================================================

-- Log migration completion
INSERT INTO migrations (migration_name, applied_at) 
VALUES ('009_event_outbox_tables', NOW())
ON CONFLICT (migration_name) DO NOTHING;

-- Summary of created objects
SELECT 
    'Migration 009 completed successfully. Created:' as summary,
    '- event_outbox table with indexes and triggers' as detail_1,
    '- event_store table for complete event sourcing' as detail_2,
    '- event_subscriptions table for consumer tracking' as detail_3,
    '- snapshot_store table for CQRS optimization' as detail_4,
    '- Helper functions for event processing' as detail_5,
    '- Cleanup and maintenance functions' as detail_6; 