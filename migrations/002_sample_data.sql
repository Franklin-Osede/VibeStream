-- Sample data for VibeStream
-- Migration: 002_sample_data.sql

-- Insert sample users
INSERT INTO users (id, email, username, password_hash, wallet_address, role, is_verified) VALUES
(
    '550e8400-e29b-41d4-a716-446655440001',
    'alice@vibestream.com',
    'alice_music',
    '$2b$12$LQv3c1yqBwUlPZ/xbpHy5.QZQjGIGsUJvH8tXQjvZTCZQjGIGsUJv', -- hashed "password123"
    '0x1234567890abcdef1234567890abcdef12345678',
    'artist',
    true
),
(
    '550e8400-e29b-41d4-a716-446655440002', 
    'bob@vibestream.com',
    'bob_beats',
    '$2b$12$LQv3c1yqBwUlPZ/xbpHy5.QZQjGIGsUJvH8tXQjvZTCZQjGIGsUJv',
    '0xabcdef1234567890abcdef1234567890abcdef12',
    'artist',
    true
),
(
    '550e8400-e29b-41d4-a716-446655440003',
    'carol@vibestream.com', 
    'carol_fan',
    '$2b$12$LQv3c1yqBwUlPZ/xbpHy5.QZQjGIGsUJvH8tXQjvZTCZQjGIGsUJv',
    '0x9876543210fedcba9876543210fedcba98765432',
    'user',
    false
);

-- Insert sample artists
INSERT INTO artists (id, user_id, stage_name, bio, verified) VALUES
(
    '660e8400-e29b-41d4-a716-446655440001',
    '550e8400-e29b-41d4-a716-446655440001',
    'Alice Harmony',
    'Electronic music producer and DJ specializing in ambient and downtempo beats.',
    true
),
(
    '660e8400-e29b-41d4-a716-446655440002',
    '550e8400-e29b-41d4-a716-446655440002', 
    'Bob Bassline',
    'Hip-hop producer and beat maker with over 10 years of experience.',
    true
);

-- Insert sample songs
INSERT INTO songs (id, title, artist_id, duration_seconds, genre, ipfs_hash, royalty_percentage, is_minted) VALUES
(
    '770e8400-e29b-41d4-a716-446655440001',
    'Midnight Vibes',
    '660e8400-e29b-41d4-a716-446655440001',
    240,
    'Electronic',
    'QmX1eYQxRjkwKj8rQzT9vGdU5bC3nH7mP2sA4fE8gH6jK9L',
    12.50,
    true
),
(
    '770e8400-e29b-41d4-a716-446655440002',
    'Ocean Dreams', 
    '660e8400-e29b-41d4-a716-446655440001',
    195,
    'Ambient',
    'QmY2fZ3yS5rR8wL7nC4vB6mD8pH1qE9tF0sG3hI5jK8mN2',
    10.00,
    false
),
(
    '770e8400-e29b-41d4-a716-446655440003',
    'Street Symphony',
    '660e8400-e29b-41d4-a716-446655440002',
    180,
    'Hip-Hop',
    'QmZ3gA4xT6rS9wM8oD5vC7nE9qH2rF0tG4hJ6kL9mO3pQ',
    15.00,
    true
),
(
    '770e8400-e29b-41d4-a716-446655440004',
    'Urban Flow',
    '660e8400-e29b-41d4-a716-446655440002',
    220,
    'Hip-Hop', 
    'QmA4hB5yU7sT0xN9pE6wD8oF1rG3sH5jK7mL0nP4qR6tS',
    12.00,
    false
);

-- Insert sample playlists
INSERT INTO playlists (id, user_id, title, description, is_public) VALUES
(
    '880e8400-e29b-41d4-a716-446655440001',
    '550e8400-e29b-41d4-a716-446655440003',
    'Chill Study Mix',
    'Perfect ambient tracks for studying and concentration',
    true
),
(
    '880e8400-e29b-41d4-a716-446655440002',
    '550e8400-e29b-41d4-a716-446655440003',
    'Hip-Hop Favorites', 
    'Best hip-hop beats and flows',
    true
);

-- Insert playlist songs
INSERT INTO playlist_songs (playlist_id, song_id, position) VALUES
('880e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440001', 1),
('880e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440002', 2),
('880e8400-e29b-41d4-a716-446655440002', '770e8400-e29b-41d4-a716-446655440003', 1),
('880e8400-e29b-41d4-a716-446655440002', '770e8400-e29b-41d4-a716-446655440004', 2);

-- Insert sample transactions
INSERT INTO transactions (id, request_id, user_id, blockchain, transaction_type, from_address, to_address, amount, tx_hash, status) VALUES
(
    '990e8400-e29b-41d4-a716-446655440001',
    'req_mint_001',
    '550e8400-e29b-41d4-a716-446655440001',
    'Ethereum',
    'mint_nft',
    '0x1234567890abcdef1234567890abcdef12345678',
    '0x0000000000000000000000000000000000000000',
    0,
    '0xabcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234',
    'confirmed'
),
(
    '990e8400-e29b-41d4-a716-446655440002',
    'req_transfer_001',
    '550e8400-e29b-41d4-a716-446655440002',
    'Solana',
    'transfer',
    'Bob123ABC456DEF789GHI012JKL345MNO678PQR901STU234VWX567YZ',
    'Alice567DEF890GHI123JKL456MNO789PQR012STU345VWX678YZ901ABC',
    1000000000,
    'signature123456789abcdef123456789abcdef123456789abcdef123456789abcdef12',
    'confirmed'
);

-- Insert sample listen events
INSERT INTO listen_events (user_id, song_id, listen_duration_seconds, ip_address) VALUES
('550e8400-e29b-41d4-a716-446655440003', '770e8400-e29b-41d4-a716-446655440001', 240, '192.168.1.100'),
('550e8400-e29b-41d4-a716-446655440003', '770e8400-e29b-41d4-a716-446655440002', 180, '192.168.1.100'),
('550e8400-e29b-41d4-a716-446655440003', '770e8400-e29b-41d4-a716-446655440003', 175, '192.168.1.100'),
('550e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440003', 160, '10.0.0.50'); 