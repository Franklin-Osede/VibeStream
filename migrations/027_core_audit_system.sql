-- Migration: 027_core_audit_system.sql
-- Description: Implement unified audit logging for Core Bounded Context (Users, Songs, Campaigns)
-- Date: 2026-01-17

-- 1. Create Audit Log Table
CREATE TABLE IF NOT EXISTS system_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name VARCHAR(255) NOT NULL,
    record_id UUID NOT NULL,
    operation VARCHAR(50) NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')),
    old_values JSONB, -- NULL for INSERT
    new_values JSONB, -- NULL for DELETE
    changed_by UUID, -- Optional: User who made the change (needs application logic to pass this via SET local)
    changed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for audit log
CREATE INDEX IF NOT EXISTS idx_system_audit_log_table_name ON system_audit_log(table_name);
CREATE INDEX IF NOT EXISTS idx_system_audit_log_record_id ON system_audit_log(record_id);
CREATE INDEX IF NOT EXISTS idx_system_audit_log_operation ON system_audit_log(operation);
CREATE INDEX IF NOT EXISTS idx_system_audit_log_changed_at ON system_audit_log(changed_at DESC);

-- 2. Create Generic Audit Trigger Function
CREATE OR REPLACE FUNCTION system_audit_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO system_audit_log (table_name, record_id, operation, new_values, changed_at)
        VALUES (TG_TABLE_NAME, NEW.id, 'INSERT', to_jsonb(NEW), NOW());
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        -- Only log if values actually changed (ignoring updated_at)
        IF NEW IS DISTINCT FROM OLD THEN
            INSERT INTO system_audit_log (table_name, record_id, operation, old_values, new_values, changed_at)
            VALUES (TG_TABLE_NAME, NEW.id, 'UPDATE', to_jsonb(OLD), to_jsonb(NEW), NOW());
        END IF;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO system_audit_log (table_name, record_id, operation, old_values, changed_at)
        VALUES (TG_TABLE_NAME, OLD.id, 'DELETE', to_jsonb(OLD), NOW());
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- 3. Apply Triggers to Critical Core Tables

-- Users
DROP TRIGGER IF EXISTS audit_trigger_users ON users;
CREATE TRIGGER audit_trigger_users
    AFTER INSERT OR UPDATE OR DELETE ON users
    FOR EACH ROW EXECUTE FUNCTION system_audit_trigger();

-- Songs
DROP TRIGGER IF EXISTS audit_trigger_songs ON songs;
CREATE TRIGGER audit_trigger_songs
    AFTER INSERT OR UPDATE OR DELETE ON songs
    FOR EACH ROW EXECUTE FUNCTION system_audit_trigger();

-- Campaigns
DROP TRIGGER IF EXISTS audit_trigger_campaigns ON campaigns;
CREATE TRIGGER audit_trigger_campaigns
    AFTER INSERT OR UPDATE OR DELETE ON campaigns
    FOR EACH ROW EXECUTE FUNCTION system_audit_trigger();

-- Ownership Contracts
DROP TRIGGER IF EXISTS audit_trigger_ownership_contracts ON ownership_contracts;
CREATE TRIGGER audit_trigger_ownership_contracts
    AFTER INSERT OR UPDATE OR DELETE ON ownership_contracts
    FOR EACH ROW EXECUTE FUNCTION system_audit_trigger();

-- Comments
COMMENT ON TABLE system_audit_log IS 'Centralized audit log for Core Bounded Context tables';
