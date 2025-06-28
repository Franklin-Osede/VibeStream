-- Migration for Fractional Ownership Context
-- Creates tables for fractional song ownership, share transactions, and related data

-- Table for fractional songs (songs that can be owned fractionally)
CREATE TABLE fractional_songs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    song_id UUID NOT NULL, -- Reference to songs table in music context
    artist_id UUID NOT NULL, -- Reference to users table
    title VARCHAR(255) NOT NULL,
    total_shares INTEGER NOT NULL CHECK (total_shares > 0),
    artist_reserved_shares INTEGER NOT NULL DEFAULT 0,
    fan_available_shares INTEGER NOT NULL,
    artist_revenue_percentage DECIMAL(5,4) NOT NULL DEFAULT 0.0 CHECK (artist_revenue_percentage >= 0.0 AND artist_revenue_percentage <= 1.0),
    available_shares INTEGER NOT NULL CHECK (available_shares >= 0),
    current_price_per_share DECIMAL(15,6) NOT NULL CHECK (current_price_per_share > 0),
    accumulated_revenue DECIMAL(15,6) NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT fk_fractional_songs_artist FOREIGN KEY (artist_id) REFERENCES users(id),
    CONSTRAINT valid_shares CHECK (available_shares <= total_shares),
    CONSTRAINT valid_reserved_shares CHECK (artist_reserved_shares <= total_shares),
    CONSTRAINT valid_fan_shares CHECK (fan_available_shares = total_shares - artist_reserved_shares)
);

-- Table for share ownership records
CREATE TABLE share_ownerships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    fractional_song_id UUID NOT NULL,
    shares_owned INTEGER NOT NULL CHECK (shares_owned > 0),
    ownership_percentage DECIMAL(8,6) NOT NULL CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    purchase_price DECIMAL(15,6) NOT NULL CHECK (purchase_price > 0),
    total_earnings DECIMAL(15,6) NOT NULL DEFAULT 0.0,
    purchase_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_earning_date TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT fk_share_ownerships_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_share_ownerships_song FOREIGN KEY (fractional_song_id) REFERENCES fractional_songs(id) ON DELETE CASCADE,
    CONSTRAINT unique_user_song_ownership UNIQUE (user_id, fractional_song_id)
);

-- Table for share transactions (purchases, transfers, etc.)
CREATE TABLE share_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    buyer_id UUID, -- NULL for sales from original artist
    seller_id UUID, -- NULL for initial purchases
    fractional_song_id UUID NOT NULL,
    shares_quantity INTEGER NOT NULL CHECK (shares_quantity > 0),
    price_per_share DECIMAL(15,6) NOT NULL CHECK (price_per_share > 0),
    total_amount DECIMAL(15,6) NOT NULL CHECK (total_amount > 0),
    transaction_type VARCHAR(20) NOT NULL CHECK (transaction_type IN ('Purchase', 'Transfer')),
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Completed', 'Failed', 'Cancelled')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT fk_share_transactions_buyer FOREIGN KEY (buyer_id) REFERENCES users(id),
    CONSTRAINT fk_share_transactions_seller FOREIGN KEY (seller_id) REFERENCES users(id),
    CONSTRAINT fk_share_transactions_song FOREIGN KEY (fractional_song_id) REFERENCES fractional_songs(id) ON DELETE CASCADE,
    CONSTRAINT valid_transaction_parties CHECK (
        (transaction_type = 'Purchase' AND seller_id IS NULL) OR
        (transaction_type = 'Transfer' AND buyer_id IS NOT NULL AND seller_id IS NOT NULL)
    )
);

-- Indexes for performance
CREATE INDEX idx_fractional_songs_artist ON fractional_songs(artist_id);
CREATE INDEX idx_fractional_songs_song ON fractional_songs(song_id);
CREATE INDEX idx_fractional_songs_available_shares ON fractional_songs(available_shares) WHERE available_shares > 0;

CREATE INDEX idx_share_ownerships_user ON share_ownerships(user_id);
CREATE INDEX idx_share_ownerships_song ON share_ownerships(fractional_song_id);
CREATE INDEX idx_share_ownerships_user_song ON share_ownerships(user_id, fractional_song_id);

CREATE INDEX idx_share_transactions_buyer ON share_transactions(buyer_id);
CREATE INDEX idx_share_transactions_seller ON share_transactions(seller_id);
CREATE INDEX idx_share_transactions_song ON share_transactions(fractional_song_id);
CREATE INDEX idx_share_transactions_status ON share_transactions(status);
CREATE INDEX idx_share_transactions_date ON share_transactions(created_at);

-- Trigger to update updated_at on fractional_songs
CREATE OR REPLACE FUNCTION update_fractional_songs_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_fractional_songs_updated_at
    BEFORE UPDATE ON fractional_songs
    FOR EACH ROW
    EXECUTE FUNCTION update_fractional_songs_updated_at();

-- Insert some sample data for testing
INSERT INTO fractional_songs (
    id, song_id, artist_id, title, total_shares, artist_reserved_shares, 
    fan_available_shares, artist_revenue_percentage, available_shares, current_price_per_share
) VALUES
(
    '550e8400-e29b-41d4-a716-446655440001', 
    '550e8400-e29b-41d4-a716-446655440002', -- Assuming this song exists
    '550e8400-e29b-41d4-a716-446655440003', -- Assuming this user/artist exists
    'Test Fractional Song',
    1000,
    200, -- Artist keeps 20%
    800, -- 80% available for fans
    0.1, -- Artist gets additional 10% of all revenue
    800, -- All fan shares currently available
    10.00
),
(
    '550e8400-e29b-41d4-a716-446655440004',
    '550e8400-e29b-41d4-a716-446655440005',
    '550e8400-e29b-41d4-a716-446655440006',
    'Popular Hit Song',
    2000,
    400, -- Artist keeps 20%
    1600, -- 80% available for fans
    0.15, -- Artist gets additional 15% of all revenue
    1200, -- 400 shares already sold to fans
    25.50
);

-- Comments for documentation
COMMENT ON TABLE fractional_songs IS 'Songs that can be owned fractionally by fans';
COMMENT ON TABLE share_ownerships IS 'Records of user ownership in fractional songs';
COMMENT ON TABLE share_transactions IS 'History of all share purchases and transfers';

COMMENT ON COLUMN fractional_songs.artist_reserved_shares IS 'Number of shares the artist keeps for themselves';
COMMENT ON COLUMN fractional_songs.fan_available_shares IS 'Number of shares available for fans to purchase';
COMMENT ON COLUMN fractional_songs.artist_revenue_percentage IS 'Additional percentage of revenue that goes to artist (beyond their share ownership)';
COMMENT ON COLUMN share_ownerships.ownership_percentage IS 'Percentage of the song owned by this user';
COMMENT ON COLUMN share_transactions.transaction_type IS 'Type of transaction: Purchase (from artist) or Transfer (between users)'; 