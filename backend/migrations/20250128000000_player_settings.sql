-- Player settings table for email preferences
CREATE TABLE IF NOT EXISTS player_settings (
    player_name TEXT PRIMARY KEY,
    email TEXT,
    game_notifications BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Daily email counter table
CREATE TABLE IF NOT EXISTS email_daily_stats (
    date DATE PRIMARY KEY,
    emails_sent INTEGER NOT NULL DEFAULT 0,
    last_email_at TIMESTAMPTZ
);

-- Index for looking up players with notifications enabled
CREATE INDEX IF NOT EXISTS idx_player_settings_notifications ON player_settings(game_notifications) WHERE game_notifications = TRUE;
