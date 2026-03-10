-- Fix player_settings for case-insensitive lookup and upsert
-- This migration ensures that the player_name is always unique when compared case-insensitively

-- 1. Create a unique index on LOWER(TRIM(player_name)) if it doesn't exist
CREATE UNIQUE INDEX IF NOT EXISTS idx_player_settings_name_unique ON player_settings (LOWER(TRIM(player_name)));
