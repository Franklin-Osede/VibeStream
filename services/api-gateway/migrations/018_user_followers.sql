-- Migration: 018_user_followers.sql
-- Description: Creates table for user followers relationship
-- Author: Antigravity
-- Created: 2024

-- =====================================================
-- USER FOLLOWERS TABLE
-- =====================================================
CREATE TABLE user_followers (
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    followee_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Composite primary key to prevent duplicate follows
    PRIMARY KEY (follower_id, followee_id)
);

-- Indexes for performance
CREATE INDEX idx_user_followers_follower_id ON user_followers(follower_id);
CREATE INDEX idx_user_followers_followee_id ON user_followers(followee_id);
CREATE INDEX idx_user_followers_created_at ON user_followers(created_at);
