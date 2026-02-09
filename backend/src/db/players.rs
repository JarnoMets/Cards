//! Player search and management operations

use chrono::{DateTime, Utc};
use sqlx::Row;

use super::{Database, DbError, DbResult, PlayerSearchRow};
use crate::models::Game;

impl Database {
    pub async fn get_games_for_player(&self, player_name: &str) -> DbResult<Vec<Game>> {
        let trimmed = player_name.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let rows =
            sqlx::query("SELECT DISTINCT game_id FROM players WHERE LOWER(name) = LOWER($1)")
                .bind(trimmed)
                .fetch_all(&self.pool)
                .await?;

        let mut games = Vec::new();
        for row in rows {
            let game_id: String = row.get("game_id");
            if let Ok(game) = self.get_game(&game_id).await {
                games.push(game);
            }
        }

        Ok(games)
    }

    pub async fn search_players(
        &self,
        query: Option<&str>,
        limit: i64,
    ) -> DbResult<Vec<PlayerSearchRow>> {
        let limit = limit.clamp(1, 50);
        let sanitized = query.and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        });

        let rows = if let Some(term) = sanitized {
            let pattern = format!("%{}%", escape_like_pattern(&term));
            sqlx::query(
                "SELECT p.name, COUNT(*)::bigint AS games_played, MAX(g.created_at) AS last_played_at
                 FROM players p
                 INNER JOIN games g ON g.id = p.game_id
                 WHERE p.name ILIKE $1 ESCAPE '\\'
                 GROUP BY p.name
                 ORDER BY games_played DESC, last_played_at DESC NULLS LAST
                 LIMIT $2",
            )
            .bind(pattern)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                "SELECT p.name, COUNT(*)::bigint AS games_played, MAX(g.created_at) AS last_played_at
                 FROM players p
                 INNER JOIN games g ON g.id = p.game_id
                 GROUP BY p.name
                 ORDER BY games_played DESC, last_played_at DESC NULLS LAST
                 LIMIT $1",
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };

        let results = rows
            .into_iter()
            .map(|row| {
                let name: String = row.get("name");
                let games_played: i64 = row.get("games_played");
                let last_played_at: Option<DateTime<Utc>> = row.get("last_played_at");
                PlayerSearchRow {
                    name,
                    games_played,
                    last_played_at,
                }
            })
            .collect();

        Ok(results)
    }

    pub async fn update_player_name(
        &self,
        game_id: &str,
        player_id: &str,
        new_name: &str,
    ) -> DbResult<Game> {
        if new_name.trim().is_empty() {
            return Err(DbError::InvalidData(
                "Player name cannot be empty".to_string(),
            ));
        }

        let result = sqlx::query("UPDATE players SET name = $1 WHERE id = $2 AND game_id = $3")
            .bind(new_name)
            .bind(player_id)
            .bind(game_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::InvalidData(
                "Player not found for this game".to_string(),
            ));
        }

        self.get_game(game_id).await
    }

    /// Rename a player globally across all games they've played
    pub async fn rename_player_globally(&self, old_name: &str, new_name: &str) -> DbResult<u64> {
        if new_name.trim().is_empty() {
            return Err(DbError::InvalidData(
                "New player name cannot be empty".to_string(),
            ));
        }

        if old_name.trim().is_empty() {
            return Err(DbError::InvalidData(
                "Old player name cannot be empty".to_string(),
            ));
        }

        // Use case-insensitive matching for the old name
        let old_normalized = old_name.trim().to_lowercase();

        let result = sqlx::query("UPDATE players SET name = $1 WHERE LOWER(TRIM(name)) = $2")
            .bind(new_name.trim())
            .bind(&old_normalized)
            .execute(&self.pool)
            .await?;

        // Also rename in the ELO table
        let _ = self.rename_player_elo(old_name, new_name).await;

        // Also rename in the settings table
        let _ = self.rename_player_settings(old_name, new_name).await;

        Ok(result.rows_affected())
    }
}

/// Escape special characters in LIKE patterns
fn escape_like_pattern(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '%' | '_' | '\\' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}
