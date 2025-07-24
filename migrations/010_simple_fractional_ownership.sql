-- =============================================================================
-- FAN VENTURES - DATABASE SCHEMA (Reemplazando Fractional Ownership)
-- =============================================================================

-- Artist Ventures Table
CREATE TABLE IF NOT EXISTS artist_ventures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    investment_type VARCHAR(50) NOT NULL,
    min_investment DECIMAL(10,2) NOT NULL CHECK (min_investment > 0),
    max_investment DECIMAL(10,2) NOT NULL CHECK (max_investment >= min_investment),
    total_goal DECIMAL(12,2) NOT NULL CHECK (total_goal > 0),
    current_amount DECIMAL(12,2) NOT NULL DEFAULT 0 CHECK (current_amount >= 0),
    max_investors INTEGER,
    current_investors INTEGER NOT NULL DEFAULT 0 CHECK (current_investors >= 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'Open', 'Funded', 'Active', 'Completed', 'Cancelled', 'Expired')),
    
    CONSTRAINT artist_ventures_current_amount_check 
        CHECK (current_amount <= total_goal)
);

-- Fan Investments Table
CREATE TABLE IF NOT EXISTS fan_investments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    artist_id UUID NOT NULL,
    fan_id UUID NOT NULL,
    investment_amount DECIMAL(10,2) NOT NULL CHECK (investment_amount > 0),
    investment_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Active', 'Completed', 'Cancelled', 'Failed')),
    expected_return DECIMAL(10,2) NOT NULL DEFAULT 0 CHECK (expected_return >= 0),
    duration_months INTEGER NOT NULL DEFAULT 12 CHECK (duration_months > 0)
);

-- Venture Benefits Table
CREATE TABLE IF NOT EXISTS venture_benefits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    benefit_type VARCHAR(50) NOT NULL,
    delivery_date TIMESTAMP WITH TIME ZONE,
    is_delivered BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Revenue Distributions Table (for ventures with revenue sharing)
CREATE TABLE IF NOT EXISTS revenue_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    venture_id UUID NOT NULL REFERENCES artist_ventures(id) ON DELETE CASCADE,
    total_revenue DECIMAL(12,2) NOT NULL CHECK (total_revenue >= 0),
    artist_share DECIMAL(12,2) NOT NULL CHECK (artist_share >= 0),
    fan_share DECIMAL(12,2) NOT NULL CHECK (fan_share >= 0),
    platform_fee DECIMAL(12,2) NOT NULL CHECK (platform_fee >= 0),
    distributed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    
    CONSTRAINT revenue_distributions_total_check 
        CHECK (total_revenue = artist_share + fan_share + platform_fee),
    CONSTRAINT revenue_distributions_period_check
        CHECK (period_start < period_end)
);

-- Indexes for Performance
CREATE INDEX IF NOT EXISTS idx_artist_ventures_artist_id ON artist_ventures(artist_id);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_status ON artist_ventures(status);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_created_at ON artist_ventures(created_at);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_expires_at ON artist_ventures(expires_at);

CREATE INDEX IF NOT EXISTS idx_fan_investments_artist_id ON fan_investments(artist_id);
CREATE INDEX IF NOT EXISTS idx_fan_investments_fan_id ON fan_investments(fan_id);
CREATE INDEX IF NOT EXISTS idx_fan_investments_created_at ON fan_investments(created_at);
CREATE INDEX IF NOT EXISTS idx_fan_investments_status ON fan_investments(status);

CREATE INDEX IF NOT EXISTS idx_venture_benefits_venture_id ON venture_benefits(venture_id);
CREATE INDEX IF NOT EXISTS idx_venture_benefits_delivery_date ON venture_benefits(delivery_date);

CREATE INDEX IF NOT EXISTS idx_revenue_distributions_venture_id ON revenue_distributions(venture_id);
CREATE INDEX IF NOT EXISTS idx_revenue_distributions_distributed_at ON revenue_distributions(distributed_at);

-- Sample Data for Testing
INSERT INTO artist_ventures (artist_id, title, description, investment_type, min_investment, max_investment, total_goal, current_amount, max_investors, current_investors, expires_at, status) VALUES
    ('550e8400-e29b-41d4-a716-446655440001', 'Exclusive Album Pre-Release', 'Get early access to my new album before anyone else', 'EarlyAccess', 10.00, 100.00, 5000.00, 1200.00, 200, 45, NOW() + INTERVAL '30 days', 'Open'),
    ('550e8400-e29b-41d4-a716-446655440002', 'VIP Concert Experience', 'Exclusive concert with meet & greet', 'MeetAndGreet', 50.00, 500.00, 10000.00, 0.00, 50, 0, NOW() + INTERVAL '60 days', 'Open'),
    ('550e8400-e29b-41d4-a716-446655440003', 'Revenue Sharing Project', 'Invest in my music and share the profits', 'RevenueShare', 25.00, 1000.00, 25000.00, 25000.00, 500, 500, NOW() + INTERVAL '90 days', 'Funded')
ON CONFLICT DO NOTHING;

INSERT INTO venture_benefits (venture_id, title, description, benefit_type, delivery_date, is_delivered) VALUES
    ('550e8400-e29b-41d4-a716-446655440001', 'Digital Album Download', 'Early access to the full album', 'DigitalContent', NOW() + INTERVAL '15 days', FALSE),
    ('550e8400-e29b-41d4-a716-446655440001', 'Behind the Scenes Video', 'Exclusive studio footage', 'DigitalContent', NOW() + INTERVAL '20 days', FALSE),
    ('550e8400-e29b-41d4-a716-446655440002', 'VIP Concert Ticket', 'Exclusive concert access', 'Experience', NOW() + INTERVAL '45 days', FALSE),
    ('550e8400-e29b-41d4-a716-446655440002', 'Meet & Greet Session', 'Personal meet and greet', 'Experience', NOW() + INTERVAL '45 days', FALSE),
    ('550e8400-e29b-41d4-a716-446655440003', 'Monthly Revenue Share', 'Share in monthly streaming revenue', 'RevenueShare', NOW() + INTERVAL '30 days', FALSE)
ON CONFLICT DO NOTHING;

INSERT INTO fan_investments (artist_id, fan_id, investment_amount, investment_type, status, expected_return, duration_months) VALUES
    ('550e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440010', 25.00, 'EarlyAccess', 'Active', 0.00, 12),
    ('550e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440011', 50.00, 'EarlyAccess', 'Active', 0.00, 12),
    ('550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440012', 100.00, 'RevenueShare', 'Active', 150.00, 24)
ON CONFLICT DO NOTHING;

INSERT INTO revenue_distributions (venture_id, total_revenue, artist_share, fan_share, platform_fee, period_start, period_end) VALUES
    ('550e8400-e29b-41d4-a716-446655440003', 1000.00, 600.00, 350.00, 50.00, NOW() - INTERVAL '30 days', NOW()),
    ('550e8400-e29b-41d4-a716-446655440003', 2500.00, 1500.00, 875.00, 125.00, NOW() - INTERVAL '60 days', NOW() - INTERVAL '30 days')
ON CONFLICT DO NOTHING; 