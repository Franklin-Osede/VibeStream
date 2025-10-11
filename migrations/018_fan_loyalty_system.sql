-- Fan Loyalty System Database Schema
-- TDD REFACTOR PHASE - Real database implementation

-- ============================================================================
-- FAN VERIFICATION TABLES
-- ============================================================================

CREATE TABLE fan_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fan_id UUID NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    confidence_score DECIMAL(5,4) NOT NULL DEFAULT 0.0,
    verification_id VARCHAR(255) NOT NULL UNIQUE,
    wristband_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    benefits_unlocked JSONB DEFAULT '[]'::jsonb,
    biometric_data JSONB,
    device_fingerprint VARCHAR(255),
    location_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_fan_verifications_fan_id ON fan_verifications(fan_id);
CREATE INDEX idx_fan_verifications_verification_id ON fan_verifications(verification_id);
CREATE INDEX idx_fan_verifications_created_at ON fan_verifications(created_at);

-- ============================================================================
-- NFT WRISTBAND TABLES
-- ============================================================================

CREATE TABLE nft_wristbands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fan_id UUID NOT NULL,
    concert_id VARCHAR(255) NOT NULL,
    artist_id VARCHAR(255) NOT NULL,
    wristband_type VARCHAR(50) NOT NULL CHECK (wristband_type IN ('General', 'VIP', 'Backstage', 'MeetAndGreet')),
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    activated_at TIMESTAMP WITH TIME ZONE,
    nft_token_id VARCHAR(255),
    transaction_hash VARCHAR(255),
    ipfs_hash VARCHAR(255),
    blockchain_network VARCHAR(50) DEFAULT 'ethereum',
    contract_address VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_nft_wristbands_fan_id ON nft_wristbands(fan_id);
CREATE INDEX idx_nft_wristbands_concert_id ON nft_wristbands(concert_id);
CREATE INDEX idx_nft_wristbands_artist_id ON nft_wristbands(artist_id);
CREATE INDEX idx_nft_wristbands_wristband_type ON nft_wristbands(wristband_type);
CREATE INDEX idx_nft_wristbands_is_active ON nft_wristbands(is_active);
CREATE INDEX idx_nft_wristbands_created_at ON nft_wristbands(created_at);

-- ============================================================================
-- QR CODE TABLES
-- ============================================================================

CREATE TABLE qr_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(255) NOT NULL UNIQUE,
    wristband_id UUID NOT NULL REFERENCES nft_wristbands(id) ON DELETE CASCADE,
    is_valid BOOLEAN NOT NULL DEFAULT TRUE,
    signature VARCHAR(255),
    expires_at TIMESTAMP WITH TIME ZONE,
    used_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_qr_codes_code ON qr_codes(code);
CREATE INDEX idx_qr_codes_wristband_id ON qr_codes(wristband_id);
CREATE INDEX idx_qr_codes_is_valid ON qr_codes(is_valid);
CREATE INDEX idx_qr_codes_expires_at ON qr_codes(expires_at);

-- ============================================================================
-- ZK PROOF TABLES
-- ============================================================================

CREATE TABLE zk_proofs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proof_id UUID NOT NULL UNIQUE,
    proof_data TEXT NOT NULL,
    circuit_name VARCHAR(255) NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_timestamp TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_zk_proofs_proof_id ON zk_proofs(proof_id);
CREATE INDEX idx_zk_proofs_circuit_name ON zk_proofs(circuit_name);
CREATE INDEX idx_zk_proofs_is_verified ON zk_proofs(is_verified);
CREATE INDEX idx_zk_proofs_created_at ON zk_proofs(created_at);

-- ============================================================================
-- EVENT STORE TABLES
-- ============================================================================

CREATE TABLE fan_loyalty_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL UNIQUE,
    event_type VARCHAR(255) NOT NULL,
    aggregate_id VARCHAR(255) NOT NULL,
    aggregate_type VARCHAR(255) NOT NULL,
    event_data JSONB NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    version INTEGER NOT NULL DEFAULT 1,
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    processed_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_fan_loyalty_events_event_type ON fan_loyalty_events(event_type);
CREATE INDEX idx_fan_loyalty_events_aggregate_id ON fan_loyalty_events(aggregate_id);
CREATE INDEX idx_fan_loyalty_events_aggregate_type ON fan_loyalty_events(aggregate_type);
CREATE INDEX idx_fan_loyalty_events_occurred_at ON fan_loyalty_events(occurred_at);
CREATE INDEX idx_fan_loyalty_events_processed_at ON fan_loyalty_events(processed_at);

-- ============================================================================
-- AUDIT TABLES
-- ============================================================================

CREATE TABLE fan_loyalty_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name VARCHAR(255) NOT NULL,
    record_id UUID NOT NULL,
    operation VARCHAR(50) NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')),
    old_values JSONB,
    new_values JSONB,
    changed_by VARCHAR(255),
    changed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_fan_loyalty_audit_log_table_name ON fan_loyalty_audit_log(table_name);
CREATE INDEX idx_fan_loyalty_audit_log_record_id ON fan_loyalty_audit_log(record_id);
CREATE INDEX idx_fan_loyalty_audit_log_operation ON fan_loyalty_audit_log(operation);
CREATE INDEX idx_fan_loyalty_audit_log_changed_at ON fan_loyalty_audit_log(changed_at);

-- ============================================================================
-- TRIGGERS FOR AUDIT LOG
-- ============================================================================

-- Function to create audit log entries
CREATE OR REPLACE FUNCTION fan_loyalty_audit_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO fan_loyalty_audit_log (table_name, record_id, operation, new_values, changed_at)
        VALUES (TG_TABLE_NAME, NEW.id, 'INSERT', to_jsonb(NEW), NOW());
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO fan_loyalty_audit_log (table_name, record_id, operation, old_values, new_values, changed_at)
        VALUES (TG_TABLE_NAME, NEW.id, 'UPDATE', to_jsonb(OLD), to_jsonb(NEW), NOW());
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO fan_loyalty_audit_log (table_name, record_id, operation, old_values, changed_at)
        VALUES (TG_TABLE_NAME, OLD.id, 'DELETE', to_jsonb(OLD), NOW());
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for audit logging
CREATE TRIGGER fan_verifications_audit_trigger
    AFTER INSERT OR UPDATE OR DELETE ON fan_verifications
    FOR EACH ROW EXECUTE FUNCTION fan_loyalty_audit_trigger();

CREATE TRIGGER nft_wristbands_audit_trigger
    AFTER INSERT OR UPDATE OR DELETE ON nft_wristbands
    FOR EACH ROW EXECUTE FUNCTION fan_loyalty_audit_trigger();

CREATE TRIGGER qr_codes_audit_trigger
    AFTER INSERT OR UPDATE OR DELETE ON qr_codes
    FOR EACH ROW EXECUTE FUNCTION fan_loyalty_audit_trigger();

CREATE TRIGGER zk_proofs_audit_trigger
    AFTER INSERT OR UPDATE OR DELETE ON zk_proofs
    FOR EACH ROW EXECUTE FUNCTION fan_loyalty_audit_trigger();

-- ============================================================================
-- SAMPLE DATA
-- ============================================================================

-- Insert sample fan verification
INSERT INTO fan_verifications (
    fan_id, is_verified, confidence_score, verification_id, wristband_eligible, benefits_unlocked
) VALUES (
    gen_random_uuid(),
    TRUE,
    0.95,
    'verification_sample_001',
    TRUE,
    '["Verified Fan Status", "VIP Access"]'::jsonb
);

-- Insert sample NFT wristband
INSERT INTO nft_wristbands (
    fan_id, concert_id, artist_id, wristband_type, is_active, nft_token_id, transaction_hash
) VALUES (
    gen_random_uuid(),
    'concert_123',
    'artist_456',
    'VIP',
    FALSE,
    'token_sample_001',
    '0x1234567890abcdef'
);

-- ============================================================================
-- VIEWS FOR REPORTING
-- ============================================================================

-- View for fan loyalty summary
CREATE VIEW fan_loyalty_summary AS
SELECT 
    fv.fan_id,
    fv.is_verified,
    fv.confidence_score,
    fv.wristband_eligible,
    COUNT(nw.id) as total_wristbands,
    COUNT(CASE WHEN nw.is_active THEN 1 END) as active_wristbands,
    COUNT(qc.id) as total_qr_codes,
    COUNT(CASE WHEN qc.is_valid THEN 1 END) as valid_qr_codes
FROM fan_verifications fv
LEFT JOIN nft_wristbands nw ON fv.fan_id = nw.fan_id
LEFT JOIN qr_codes qc ON nw.id = qc.wristband_id
GROUP BY fv.fan_id, fv.is_verified, fv.confidence_score, fv.wristband_eligible;

-- View for wristband analytics
CREATE VIEW wristband_analytics AS
SELECT 
    wristband_type,
    COUNT(*) as total_wristbands,
    COUNT(CASE WHEN is_active THEN 1 END) as active_wristbands,
    COUNT(CASE WHEN activated_at IS NOT NULL THEN 1 END) as activated_wristbands,
    AVG(EXTRACT(EPOCH FROM (activated_at - created_at))/3600) as avg_activation_hours
FROM nft_wristbands
GROUP BY wristband_type;

-- ============================================================================
-- FUNCTIONS FOR BUSINESS LOGIC
-- ============================================================================

-- Function to get fan benefits
CREATE OR REPLACE FUNCTION get_fan_benefits(fan_uuid UUID)
RETURNS JSONB AS $$
DECLARE
    benefits JSONB := '[]'::jsonb;
    wristband_record RECORD;
BEGIN
    -- Get benefits from wristband type
    FOR wristband_record IN 
        SELECT wristband_type FROM nft_wristbands 
        WHERE fan_id = fan_uuid AND is_active = TRUE
    LOOP
        CASE wristband_record.wristband_type
            WHEN 'General' THEN
                benefits := benefits || '["Concert Access"]'::jsonb;
            WHEN 'VIP' THEN
                benefits := benefits || '["Concert Access", "VIP Lounge", "Priority Entry"]'::jsonb;
            WHEN 'Backstage' THEN
                benefits := benefits || '["Concert Access", "VIP Lounge", "Priority Entry", "Backstage Access", "Artist Meet & Greet"]'::jsonb;
            WHEN 'MeetAndGreet' THEN
                benefits := benefits || '["Concert Access", "VIP Lounge", "Priority Entry", "Backstage Access", "Artist Meet & Greet", "Photo Opportunity", "Autograph Session"]'::jsonb;
        END CASE;
    END LOOP;
    
    RETURN benefits;
END;
$$ LANGUAGE plpgsql;

-- Function to validate QR code
CREATE OR REPLACE FUNCTION validate_qr_code(qr_code VARCHAR(255))
RETURNS JSONB AS $$
DECLARE
    result JSONB;
    wristband_record RECORD;
BEGIN
    SELECT 
        qc.id,
        qc.wristband_id,
        qc.is_valid,
        qc.expires_at,
        nw.fan_id,
        nw.concert_id,
        nw.artist_id,
        nw.wristband_type,
        nw.is_active
    INTO wristband_record
    FROM qr_codes qc
    JOIN nft_wristbands nw ON qc.wristband_id = nw.id
    WHERE qc.code = qr_code;
    
    IF NOT FOUND THEN
        RETURN jsonb_build_object('is_valid', false, 'error', 'QR code not found');
    END IF;
    
    IF NOT wristband_record.is_valid THEN
        RETURN jsonb_build_object('is_valid', false, 'error', 'QR code is invalid');
    END IF;
    
    IF wristband_record.expires_at IS NOT NULL AND wristband_record.expires_at < NOW() THEN
        RETURN jsonb_build_object('is_valid', false, 'error', 'QR code has expired');
    END IF;
    
    IF NOT wristband_record.is_active THEN
        RETURN jsonb_build_object('is_valid', false, 'error', 'Wristband is not active');
    END IF;
    
    RETURN jsonb_build_object(
        'is_valid', true,
        'wristband_id', wristband_record.wristband_id,
        'fan_id', wristband_record.fan_id,
        'concert_id', wristband_record.concert_id,
        'artist_id', wristband_record.artist_id,
        'wristband_type', wristband_record.wristband_type,
        'benefits', get_fan_benefits(wristband_record.fan_id)
    );
END;
$$ LANGUAGE plpgsql;
