//! Core database types, error handling, and connection management

use chrono::{DateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Game not found")]
    GameNotFound,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Game logic error: {0}")]
    GameLogicError(String),
}

pub type DbResult<T> = Result<T, DbError>;

/// Main database handle - clone is cheap (Arc internally)
#[derive(Clone)]
pub struct Database {
    pub(crate) pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> DbResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Run migrations
        Self::run_migrations(&pool).await?;

        Ok(Database { pool })
    }

    async fn run_migrations(pool: &PgPool) -> DbResult<()> {
        // Games table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS games (
                id TEXT PRIMARY KEY,
                game_type TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL,
                current_round INTEGER NOT NULL DEFAULT 0,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add updated_at column if it doesn't exist (migration for existing databases)
        let _ = sqlx::query(
            "ALTER TABLE games ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()"
        )
        .execute(pool)
        .await;

        // Players table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS players (
                id TEXT PRIMARY KEY,
                game_id TEXT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                position INTEGER NOT NULL,
                scores JSONB NOT NULL DEFAULT '[]',
                game_indices JSONB NOT NULL DEFAULT '[]',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Indexes
        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_players_game_id ON players(game_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_games_created_at ON games(created_at DESC)",
        )
        .execute(pool)
        .await;

        // Migration: Add game_indices column if it doesn't exist
        let _ = sqlx::query(
            "ALTER TABLE players ADD COLUMN IF NOT EXISTS game_indices JSONB NOT NULL DEFAULT '[]'",
        )
        .execute(pool)
        .await;

        // Player ELO table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS player_elo (
                id SERIAL PRIMARY KEY,
                player_name TEXT NOT NULL,
                game_type TEXT NOT NULL,
                elo INTEGER NOT NULL DEFAULT 1000,
                games_played INTEGER NOT NULL DEFAULT 0,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE(player_name, game_type)
            )",
        )
        .execute(pool)
        .await?;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_player_elo_name ON player_elo(player_name)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_player_elo_game_type ON player_elo(game_type)")
            .execute(pool)
            .await;

        // ELO history table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS player_elo_history (
                id SERIAL PRIMARY KEY,
                player_name TEXT NOT NULL,
                game_type TEXT NOT NULL,
                elo INTEGER NOT NULL,
                game_id TEXT NOT NULL,
                recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_elo_history_player ON player_elo_history(player_name)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_elo_history_recorded ON player_elo_history(recorded_at)")
            .execute(pool)
            .await;

        // ELO config table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS elo_config (
                id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
                starting_elo INTEGER NOT NULL DEFAULT 800,
                target_rating INTEGER NOT NULL DEFAULT 1500,
                k_factor_new INTEGER NOT NULL DEFAULT 40,
                k_factor_mid INTEGER NOT NULL DEFAULT 32,
                k_factor_established INTEGER NOT NULL DEFAULT 24,
                games_until_mid INTEGER NOT NULL DEFAULT 10,
                games_until_established INTEGER NOT NULL DEFAULT 30,
                floor_elo INTEGER NOT NULL DEFAULT 100,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Add target_rating column if it doesn't exist (migration)
        let _ = sqlx::query(
            "ALTER TABLE elo_config ADD COLUMN IF NOT EXISTS target_rating INTEGER NOT NULL DEFAULT 1500"
        )
        .execute(pool)
        .await;

        let _ = sqlx::query(
            "INSERT INTO elo_config (id) VALUES (1) ON CONFLICT (id) DO NOTHING"
        )
        .execute(pool)
        .await;

        // Leaderboard cache table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS leaderboard_cache (
                id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
                stats_json JSONB,
                records_json JSONB,
                elo_json JSONB,
                cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        let _ = sqlx::query(
            "INSERT INTO leaderboard_cache (id) VALUES (1) ON CONFLICT (id) DO NOTHING"
        )
        .execute(pool)
        .await;

        // Game ELO changes table - stores ELO changes per player per game
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS game_elo_changes (
                id SERIAL PRIMARY KEY,
                game_id TEXT NOT NULL,
                player_name TEXT NOT NULL,
                elo_before INTEGER NOT NULL,
                elo_after INTEGER NOT NULL,
                elo_change INTEGER NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE(game_id, player_name)
            )",
        )
        .execute(pool)
        .await?;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_game_elo_changes_game ON game_elo_changes(game_id)")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_game_elo_changes_player ON game_elo_changes(player_name)")
            .execute(pool)
            .await;

        // Player settings table - email preferences
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS player_settings (
                player_name TEXT PRIMARY KEY,
                email TEXT,
                game_notifications BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        // Case-insensitive unique index for player names
        let _ = sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_player_settings_name_unique ON player_settings (LOWER(TRIM(player_name)))")
            .execute(pool)
            .await;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_player_settings_notifications ON player_settings(game_notifications) WHERE game_notifications = TRUE")
            .execute(pool)
            .await;

        // Daily email counter table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS email_daily_stats (
                date DATE PRIMARY KEY,
                emails_sent INTEGER NOT NULL DEFAULT 0,
                last_email_at TIMESTAMPTZ
            )",
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

/// Configuration for skill rating calculations
/// 
/// The rating system is designed to create a Gaussian distribution centered around
/// `target_rating` (default 1500) over time. Players start at `starting_rating` (default 800)
/// which is intentionally below the target - this means new players will gain more rating
/// than they lose on average, allowing them to "earn their way up" to the average.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EloConfig {
    /// Rating all new players start with (default: 800)
    pub starting_elo: i32,
    /// Target average rating the system trends toward (default: 1500)
    #[serde(default = "default_target_rating")]
    pub target_rating: i32,
    /// K-factor for new players (< games_until_mid games) - higher = faster changes
    pub k_factor_new: i32,
    /// K-factor for mid-level players
    pub k_factor_mid: i32,
    /// K-factor for established players (>= games_until_established games)
    pub k_factor_established: i32,
    /// Games needed to transition from new to mid K-factor
    pub games_until_mid: i32,
    /// Games needed to transition from mid to established K-factor
    pub games_until_established: i32,
    /// Minimum possible rating (floor)
    pub floor_elo: i32,
}

fn default_target_rating() -> i32 {
    1500
}

impl Default for EloConfig {
    fn default() -> Self {
        EloConfig {
            starting_elo: 800,
            target_rating: 1500,
            k_factor_new: 40,
            k_factor_mid: 32,
            k_factor_established: 24,
            games_until_mid: 10,
            games_until_established: 30,
            floor_elo: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerSearchRow {
    pub name: String,
    pub games_played: i64,
    pub last_played_at: Option<DateTime<Utc>>,
}
