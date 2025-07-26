-- Migration: 015_venture_categories.sql
-- Add categories and tags to ventures

-- Add category and tags columns to artist_ventures
ALTER TABLE artist_ventures 
ADD COLUMN IF NOT EXISTS category VARCHAR(20) DEFAULT 'other' CHECK (category IN ('music', 'visual_arts', 'film', 'gaming', 'technology', 'fashion', 'food', 'travel', 'education', 'health', 'other')),
ADD COLUMN IF NOT EXISTS tags TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS risk_level VARCHAR(10) DEFAULT 'medium' CHECK (risk_level IN ('low', 'medium', 'high', 'very_high')),
ADD COLUMN IF NOT EXISTS expected_return DOUBLE PRECISION DEFAULT 0.0,
ADD COLUMN IF NOT EXISTS artist_rating DOUBLE PRECISION DEFAULT 0.0,
ADD COLUMN IF NOT EXISTS artist_previous_ventures INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS artist_success_rate DOUBLE PRECISION DEFAULT 0.0;

-- Create indexes for better search performance
CREATE INDEX IF NOT EXISTS idx_artist_ventures_category ON artist_ventures(category);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_risk_level ON artist_ventures(risk_level);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_artist_rating ON artist_ventures(artist_rating);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_expected_return ON artist_ventures(expected_return);
CREATE INDEX IF NOT EXISTS idx_artist_ventures_tags ON artist_ventures USING GIN(tags);

-- Create fan_preferences table
CREATE TABLE IF NOT EXISTS fan_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fan_id UUID NOT NULL,
    favorite_categories TEXT[] DEFAULT '{}',
    preferred_investment_types TEXT[] DEFAULT '{}',
    risk_tolerance VARCHAR(10) DEFAULT 'medium' CHECK (risk_tolerance IN ('low', 'medium', 'high', 'very_high')),
    min_investment DOUBLE PRECISION DEFAULT 0.0,
    max_investment DOUBLE PRECISION DEFAULT 10000.0,
    favorite_artists UUID[] DEFAULT '{}',
    interests TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for fan_preferences
CREATE INDEX IF NOT EXISTS idx_fan_preferences_fan_id ON fan_preferences(fan_id);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_favorite_categories ON fan_preferences USING GIN(favorite_categories);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_preferred_investment_types ON fan_preferences USING GIN(preferred_investment_types);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_favorite_artists ON fan_preferences USING GIN(favorite_artists);
CREATE INDEX IF NOT EXISTS idx_fan_preferences_interests ON fan_preferences USING GIN(interests);

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_fan_preferences_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_fan_preferences_updated_at
    BEFORE UPDATE ON fan_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_fan_preferences_updated_at();

-- Add comments for documentation
COMMENT ON COLUMN artist_ventures.category IS 'Category of the venture: music, visual_arts, film, gaming, technology, fashion, food, travel, education, health, other';
COMMENT ON COLUMN artist_ventures.tags IS 'Array of tags for the venture';
COMMENT ON COLUMN artist_ventures.risk_level IS 'Risk level of the venture: low, medium, high, very_high';
COMMENT ON COLUMN artist_ventures.expected_return IS 'Expected return percentage for the venture';
COMMENT ON COLUMN artist_ventures.artist_rating IS 'Average rating of the artist';
COMMENT ON COLUMN artist_ventures.artist_previous_ventures IS 'Number of previous ventures by the artist';
COMMENT ON COLUMN artist_ventures.artist_success_rate IS 'Success rate percentage of the artist';
COMMENT ON TABLE fan_preferences IS 'Stores fan preferences for venture recommendations'; 