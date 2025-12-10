//! Leaderboard statistics, records, and caching

use chrono::{DateTime, Utc};
use sqlx::Row;

use super::{Database, DbResult};

/// Leaderboard statistics for a player in a game type
#[derive(Debug, Clone, serde::Serialize)]
pub struct LeaderboardStatRow {
    pub player_name: String,
    pub game_type: String,
    pub games_played: u32,
    pub wins: u32,
    pub average_score: Option<f64>,
    pub highest_score: Option<i32>,
    pub lowest_score: Option<i32>,
}

/// Record score entry (highest or lowest)
#[derive(Debug, Clone, serde::Serialize)]
pub struct RecordScoreRow {
    pub player_name: String,
    pub game_type: String,
    pub game_id: String,
    pub occurred_at: DateTime<Utc>,
    pub score: i32,
    pub record_type: String,
}

impl Database {
    /// Query optimized leaderboard stats from materialized view
    /// This is significantly faster than processing all games in memory
    pub async fn get_leaderboard_stats(&self) -> DbResult<Vec<LeaderboardStatRow>> {
        let rows = sqlx::query(
            "SELECT 
                player_name,
                game_type,
                games_played,
                wins,
                average_score,
                highest_score,
                lowest_score
             FROM player_leaderboard_stats
             ORDER BY wins DESC, games_played DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|row| LeaderboardStatRow {
                player_name: row.get("player_name"),
                game_type: row.get("game_type"),
                games_played: row.get::<i64, _>("games_played") as u32,
                wins: row.get::<i32, _>("wins") as u32,
                average_score: row.get("average_score"),
                highest_score: row.get("highest_score"),
                lowest_score: row.get("lowest_score"),
            })
            .collect();

        Ok(results)
    }

    /// Get record scores (highest/lowest) from optimized view
    pub async fn get_record_scores(&self) -> DbResult<Vec<RecordScoreRow>> {
        let rows = sqlx::query(
            "SELECT 
                player_name,
                game_type,
                game_id,
                occurred_at,
                score,
                record_type
             FROM leaderboard_records
             ORDER BY game_type, record_type, 
                CASE WHEN record_type = 'highest' THEN -score ELSE score END",
        )
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|row| RecordScoreRow {
                player_name: row.get("player_name"),
                game_type: row.get("game_type"),
                game_id: row.get("game_id"),
                occurred_at: row.get("occurred_at"),
                score: row.get("score"),
                record_type: row.get("record_type"),
            })
            .collect();

        Ok(results)
    }

    /// Refresh the materialized view for leaderboard stats
    /// Should be called after significant game updates
    #[allow(dead_code)]
    pub async fn refresh_leaderboard_cache(&self) -> DbResult<()> {
        sqlx::query("SELECT refresh_leaderboard_stats()")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
