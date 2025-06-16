-- Migration para agregar OAuth y Wallets Automáticas

-- Agregar campos OAuth a la tabla users
ALTER TABLE users ADD COLUMN IF NOT EXISTS provider VARCHAR(50);
ALTER TABLE users ADD COLUMN IF NOT EXISTS provider_id VARCHAR(255);
ALTER TABLE users ADD COLUMN IF NOT EXISTS profile_picture TEXT;

-- Hacer password_hash opcional para usuarios OAuth
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- Crear índices para mejor performance
CREATE INDEX IF NOT EXISTS idx_users_provider ON users(provider, provider_id);
CREATE INDEX IF NOT EXISTS idx_users_wallet ON users(wallet_address);

-- Crear tabla de wallets custodiadas (para claves privadas encriptadas)
CREATE TABLE IF NOT EXISTS custodial_wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blockchain VARCHAR(50) NOT NULL, -- 'ethereum', 'solana', etc.
    public_address VARCHAR(255) NOT NULL,
    private_key_encrypted TEXT NOT NULL, -- Clave privada encriptada
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(user_id, blockchain)
);

-- Crear tabla de earnings para tracking de ganancias
CREATE TABLE IF NOT EXISTS user_earnings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount_usd DECIMAL(10,2) NOT NULL DEFAULT 0.00,
    amount_vibers DECIMAL(18,8) NOT NULL DEFAULT 0.00,
    earning_type VARCHAR(50) NOT NULL, -- 'listen_to_earn', 'referral', 'campaign_boost'
    song_id UUID, -- NULL si no es por canción específica
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insertar datos de ejemplo
INSERT INTO users (email, username, password_hash, wallet_address, role) 
VALUES 
    ('test@example.com', 'testuser', 'hashed_password', '0x1234567890abcdef', 'user'),
    ('artist@example.com', 'testartist', 'hashed_password', '0xabcdef1234567890', 'artist')
ON CONFLICT (email) DO NOTHING; 