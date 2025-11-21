-- Migration: 020_user_follows_table.sql
-- Description: Crear tabla para relaciones de seguimiento entre usuarios
-- Author: AI Assistant
-- Created: Diciembre 2024

-- =====================================================
-- USER FOLLOWERS TABLE
-- =====================================================

-- Crear tabla user_followers si no existe
CREATE TABLE IF NOT EXISTS user_followers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followee_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(follower_id, followee_id),
    CHECK(follower_id != followee_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_user_followers_follower_id ON user_followers(follower_id);
CREATE INDEX IF NOT EXISTS idx_user_followers_followee_id ON user_followers(followee_id);
CREATE INDEX IF NOT EXISTS idx_user_followers_created_at ON user_followers(created_at);
CREATE INDEX IF NOT EXISTS idx_user_followers_follower_followee ON user_followers(follower_id, followee_id);

-- Comments
COMMENT ON TABLE user_followers IS 'Relaciones de seguimiento entre usuarios';
COMMENT ON COLUMN user_followers.follower_id IS 'ID del usuario que sigue';
COMMENT ON COLUMN user_followers.followee_id IS 'ID del usuario que es seguido';
COMMENT ON COLUMN user_followers.created_at IS 'Fecha en que se creó la relación de seguimiento';

