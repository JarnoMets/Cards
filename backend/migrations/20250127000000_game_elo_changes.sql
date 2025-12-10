-- Store ELO rating changes per player per game
-- This allows displaying rating changes after game completion
CREATE TABLE IF NOT EXISTS game_elo_changes (
    id SERIAL PRIMARY KEY,
    game_id TEXT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    player_name TEXT NOT NULL,
    elo_before INTEGER NOT NULL,
    elo_after INTEGER NOT NULL,
    elo_change INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for fast lookups by game
CREATE INDEX IF NOT EXISTS idx_game_elo_changes_game_id ON game_elo_changes(game_id);

-- Index for player history
CREATE INDEX IF NOT EXISTS idx_game_elo_changes_player ON game_elo_changes(player_name);
