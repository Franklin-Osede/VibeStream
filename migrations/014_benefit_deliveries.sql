-- Migration: 014_benefit_deliveries.sql
-- Create benefit_deliveries table for tracking benefit deliveries

CREATE TABLE IF NOT EXISTS benefit_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    benefit_id UUID NOT NULL REFERENCES venture_benefits(id) ON DELETE CASCADE,
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    fan_id UUID NOT NULL,
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

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_benefit_id ON benefit_deliveries(benefit_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_venture_id ON benefit_deliveries(venture_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_fan_id ON benefit_deliveries(fan_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_tier_id ON benefit_deliveries(tier_id);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_status ON benefit_deliveries(delivery_status);
CREATE INDEX IF NOT EXISTS idx_benefit_deliveries_created_at ON benefit_deliveries(created_at);

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_benefit_deliveries_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_benefit_deliveries_updated_at
    BEFORE UPDATE ON benefit_deliveries
    FOR EACH ROW
    EXECUTE FUNCTION update_benefit_deliveries_updated_at();

-- Add comments for documentation
COMMENT ON TABLE benefit_deliveries IS 'Tracks the delivery of benefits to fans';
COMMENT ON COLUMN benefit_deliveries.delivery_status IS 'Current status of the delivery: pending, in_progress, delivered, failed, cancelled';
COMMENT ON COLUMN benefit_deliveries.delivery_method IS 'Method of delivery: automatic, manual, physical, experience';
COMMENT ON COLUMN benefit_deliveries.tracking_number IS 'Tracking number for physical deliveries';
COMMENT ON COLUMN benefit_deliveries.carrier IS 'Delivery carrier for physical deliveries'; 