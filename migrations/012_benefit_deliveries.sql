-- Migration to create benefit_deliveries table
-- This tracks which fans have received which benefits

CREATE TABLE IF NOT EXISTS benefit_deliveries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    benefit_id UUID NOT NULL REFERENCES venture_benefits(id) ON DELETE CASCADE,
    fan_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    delivery_status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, delivered, failed, cancelled
    delivery_method VARCHAR(50), -- digital, physical, experience
    delivery_date TIMESTAMP WITH TIME ZONE,
    delivery_notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT benefit_deliveries_status_check 
        CHECK (delivery_status IN ('pending', 'delivered', 'failed', 'cancelled')),
    CONSTRAINT benefit_deliveries_method_check 
        CHECK (delivery_method IN ('digital', 'physical', 'experience', 'revenue_share')),
    
    -- Indexes
    UNIQUE(benefit_id, fan_id), -- Un fan solo puede recibir un beneficio específico una vez
    INDEX idx_benefit_deliveries_fan_id (fan_id),
    INDEX idx_benefit_deliveries_venture_id (venture_id),
    INDEX idx_benefit_deliveries_status (delivery_status)
);

-- Add comments for documentation
COMMENT ON TABLE benefit_deliveries IS 'Tracks delivery of benefits to fans';
COMMENT ON COLUMN benefit_deliveries.delivery_status IS 'Current status of benefit delivery';
COMMENT ON COLUMN benefit_deliveries.delivery_method IS 'Method used to deliver the benefit';
COMMENT ON COLUMN benefit_deliveries.delivery_notes IS 'Notes about the delivery process'; 