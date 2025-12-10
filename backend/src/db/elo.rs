//! ELO/skill rating calculations, history, and configuration

use chrono::{DateTime, Utc};
use sqlx::Row;

use super::{Database, DbResult, EloConfig};
use crate::models::{Game, GameType};

/// ELO history entry for tracking rating changes over time
#[derive(Debug, Clone, serde::Serialize)]
pub struct EloHistoryEntry {
    pub player_name: String,
    pub game_type: String,
    pub elo: i32,
    pub game_id: String,
    pub recorded_at: DateTime<Utc>,
}

/// ELO change for a player in a specific game
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameEloChange {
    pub player_name: String,
    pub elo_before: i32,
    pub elo_after: i32,
    pub elo_change: i32,
}

/// Player ELO data for a specific game type
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerEloRow {
    pub player_name: String,
    pub game_type: String,
    pub elo: i32,
    pub games_played: i32,
    pub updated_at: DateTime<Utc>,
}

/// Overall ELO data across all game types
#[derive(Debug, Clone, serde::Serialize)]
pub struct OverallEloRow {
    pub player_name: String,
    pub elo: i32,
    pub games_played: i32,
    pub updated_at: DateTime<Utc>,
}

impl Database {
    /// Get skill rating configuration
    pub async fn get_elo_config(&self) -> EloConfig {
        let row = sqlx::query(
            "SELECT starting_elo, target_rating, k_factor_new, k_factor_mid, k_factor_established,
                    games_until_mid, games_until_established, floor_elo
             FROM elo_config WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await;

        match row {
            Ok(Some(r)) => EloConfig {
                starting_elo: r.get("starting_elo"),
                target_rating: r.try_get("target_rating").unwrap_or(1500),
                k_factor_new: r.get("k_factor_new"),
                k_factor_mid: r.get("k_factor_mid"),
                k_factor_established: r.get("k_factor_established"),
                games_until_mid: r.get("games_until_mid"),
                games_until_established: r.get("games_until_established"),
                floor_elo: r.get("floor_elo"),
            },
            _ => EloConfig::default(),
        }
    }

    /// Update skill rating configuration
    pub async fn set_elo_config(&self, config: &EloConfig) -> DbResult<()> {
        sqlx::query(
            "UPDATE elo_config SET 
                starting_elo = $1,
                target_rating = $2,
                k_factor_new = $3,
                k_factor_mid = $4,
                k_factor_established = $5,
                games_until_mid = $6,
                games_until_established = $7,
                floor_elo = $8,
                updated_at = NOW()
             WHERE id = 1",
        )
        .bind(config.starting_elo)
        .bind(config.target_rating)
        .bind(config.k_factor_new)
        .bind(config.k_factor_mid)
        .bind(config.k_factor_established)
        .bind(config.games_until_mid)
        .bind(config.games_until_established)
        .bind(config.floor_elo)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a player's current ELO rating for a specific game type
    pub async fn get_player_elo(&self, player_name: &str, game_type: &GameType) -> DbResult<i32> {
        let game_type_str = serde_json::to_string(game_type)?;
        let normalized_name = player_name.trim().to_lowercase();

        let row = sqlx::query(
            "SELECT elo FROM player_elo WHERE LOWER(player_name) = $1 AND game_type = $2",
        )
        .bind(&normalized_name)
        .bind(&game_type_str)
        .fetch_optional(&self.pool)
        .await?;

        let starting_elo = self.get_elo_config().await.starting_elo;
        Ok(row.map(|r| r.get::<i32, _>("elo")).unwrap_or(starting_elo))
    }

    /// Get number of ELO-rated games for a player in a specific game type
    pub async fn get_player_elo_games_count(
        &self,
        player_name: &str,
        game_type: &GameType,
    ) -> DbResult<i32> {
        let game_type_str = serde_json::to_string(game_type)?;
        let normalized_name = player_name.trim().to_lowercase();

        let row = sqlx::query(
            "SELECT games_played FROM player_elo WHERE LOWER(player_name) = $1 AND game_type = $2",
        )
        .bind(&normalized_name)
        .bind(&game_type_str)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.get::<i32, _>("games_played")).unwrap_or(0))
    }

    /// Get a player's overall ELO rating (average across all game types they've played)
    pub async fn get_player_overall_elo(&self, player_name: &str) -> DbResult<Option<(i32, i32)>> {
        let normalized_name = player_name.trim().to_lowercase();

        let row = sqlx::query(
            "SELECT AVG(elo)::integer as avg_elo, SUM(games_played)::integer as total_games
             FROM player_elo 
             WHERE LOWER(player_name) = $1",
        )
        .bind(&normalized_name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => {
                let avg_elo: Option<i32> = r.get("avg_elo");
                let total_games: Option<i32> = r.get("total_games");
                match (avg_elo, total_games) {
                    (Some(elo), Some(games)) if games > 0 => Ok(Some((elo, games))),
                    _ => {
                        let starting_elo = self.get_elo_config().await.starting_elo;
                        Ok(Some((starting_elo, 0)))
                    }
                }
            }
            None => {
                let starting_elo = self.get_elo_config().await.starting_elo;
                Ok(Some((starting_elo, 0)))
            }
        }
    }

    /// Get overall ELO leaderboard (averaged across all game types per player)
    pub async fn get_overall_elo_leaderboard(&self) -> DbResult<Vec<OverallEloRow>> {
        let rows = sqlx::query(
            "SELECT player_name, 
                    AVG(elo)::integer as avg_elo, 
                    SUM(games_played)::integer as total_games,
                    MAX(updated_at) as last_updated
             FROM player_elo 
             GROUP BY player_name
             HAVING SUM(games_played) > 0
             ORDER BY avg_elo DESC, total_games DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|row| OverallEloRow {
                player_name: row.get("player_name"),
                elo: row.get("avg_elo"),
                games_played: row.get("total_games"),
                updated_at: row.get("last_updated"),
            })
            .collect();

        Ok(results)
    }

    /// Get all ELO ratings for a player
    pub async fn get_player_all_elo(&self, player_name: &str) -> DbResult<Vec<PlayerEloRow>> {
        let normalized_name = player_name.trim().to_lowercase();

        let rows = sqlx::query(
            "SELECT player_name, game_type, elo, games_played, updated_at 
             FROM player_elo 
             WHERE LOWER(player_name) = $1
             ORDER BY game_type",
        )
        .bind(&normalized_name)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .into_iter()
            .map(|row| PlayerEloRow {
                player_name: row.get("player_name"),
                game_type: row.get("game_type"),
                elo: row.get("elo"),
                games_played: row.get("games_played"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(results)
    }

    /// Get ELO leaderboard for a specific game type
    pub async fn get_elo_leaderboard(
        &self,
        game_type: Option<&GameType>,
    ) -> DbResult<Vec<PlayerEloRow>> {
        let rows = if let Some(gt) = game_type {
            let game_type_str = serde_json::to_string(gt)?;
            sqlx::query(
                "SELECT player_name, game_type, elo, games_played, updated_at 
                 FROM player_elo 
                 WHERE game_type = $1
                 ORDER BY elo DESC, games_played DESC",
            )
            .bind(&game_type_str)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                "SELECT player_name, game_type, elo, games_played, updated_at 
                 FROM player_elo 
                 ORDER BY elo DESC, games_played DESC",
            )
            .fetch_all(&self.pool)
            .await?
        };

        let results = rows
            .into_iter()
            .map(|row| PlayerEloRow {
                player_name: row.get("player_name"),
                game_type: row.get("game_type"),
                elo: row.get("elo"),
                games_played: row.get("games_played"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(results)
    }

    /// Update skill ratings for all players after a game finishes
    /// 
    /// Uses relative finishing position and includes an inflation mechanism
    /// to push the average rating toward the target_rating over time.
    /// 
    /// Key insight: When players start below target_rating, winners gain slightly
    /// more than losers lose, causing the average to drift upward toward target.
    /// 
    /// This operation runs in a transaction to prevent race conditions when
    /// multiple games finish simultaneously with overlapping players.
    /// 
    /// Returns the ELO changes for each player.
    pub async fn update_elo_for_game(&self, game: &Game) -> DbResult<Vec<GameEloChange>> {
        if game.players.is_empty() {
            return Ok(vec![]);
        }

        let game_type_str = serde_json::to_string(&game.game_type)?;
        let num_players = game.players.len() as f64;

        // Get rating config
        let config = self.get_elo_config().await;
        let target = config.target_rating as f64;

        // Calculate placements (1st, 2nd, 3rd, etc.)
        let placements = compute_placements_for_elo(game);

        // Start a transaction to ensure atomic updates
        let mut tx = self.pool.begin().await?;

        // Get current ratings for all players (with FOR UPDATE to lock rows)
        let mut current_ratings: Vec<i32> = Vec::with_capacity(game.players.len());
        let mut games_counts: Vec<i32> = Vec::with_capacity(game.players.len());
        
        for player in &game.players {
            let normalized_name = player.name.trim().to_lowercase();
            let row = sqlx::query(
                "SELECT elo, games_played FROM player_elo 
                 WHERE LOWER(player_name) = $1 AND game_type = $2
                 FOR UPDATE",
            )
            .bind(&normalized_name)
            .bind(&game_type_str)
            .fetch_optional(&mut *tx)
            .await?;

            match row {
                Some(r) => {
                    current_ratings.push(r.get::<i32, _>("elo"));
                    games_counts.push(r.get::<i32, _>("games_played"));
                }
                None => {
                    current_ratings.push(config.starting_elo);
                    games_counts.push(0);
                }
            }
        }

        // Calculate average rating of players in this game
        let avg_rating: f64 = current_ratings.iter().map(|&r| r as f64).sum::<f64>() / num_players;
        
        // Inflation factor: positive when avg is below target, negative when above
        // This creates asymmetry that pushes ratings toward the target over time
        // The factor is scaled to be subtle but effective
        let inflation_factor = (target - avg_rating) / 400.0;

        // Collect rating changes for all players
        let mut elo_changes: Vec<GameEloChange> = Vec::with_capacity(game.players.len());

        for (i, player) in game.players.iter().enumerate() {
            // Dynamic K-factor: higher for new players (faster calibration)
            let player_games = games_counts[i];
            let k_factor = if player_games < config.games_until_mid {
                config.k_factor_new as f64
            } else if player_games < config.games_until_established {
                config.k_factor_mid as f64
            } else {
                config.k_factor_established as f64
            };
            
            let player_rating = current_ratings[i] as f64;
            let placement = placements[i] as f64;

            // Actual score: 1.0 for first place, 0.0 for last, linear between
            let actual_score = if num_players > 1.0 {
                (num_players - placement) / (num_players - 1.0)
            } else {
                1.0
            };

            // Expected score: average of expected results against all opponents
            let mut expected_score = 0.0;
            for (j, _) in game.players.iter().enumerate() {
                if i != j {
                    let opponent_rating = current_ratings[j] as f64;
                    // Standard expected score formula
                    let exp = 1.0 / (1.0 + 10.0_f64.powf((opponent_rating - player_rating) / 400.0));
                    expected_score += exp;
                }
            }
            expected_score /= (num_players - 1.0).max(1.0);

            // Base rating change
            let base_change = k_factor * (actual_score - expected_score);
            
            // Apply inflation bonus/penalty based on performance
            // Winners get a boost when avg is below target (and vice versa)
            // The inflation is proportional to how well you did (actual_score)
            let inflation_bonus = k_factor * inflation_factor * actual_score * 0.5;
            
            let rating_change = (base_change + inflation_bonus).round() as i32;
            let elo_before = player_rating as i32;
            let new_rating = (elo_before + rating_change).max(config.floor_elo);
            let actual_change = new_rating - elo_before;

            // Track the change
            elo_changes.push(GameEloChange {
                player_name: player.name.trim().to_string(),
                elo_before,
                elo_after: new_rating,
                elo_change: actual_change,
            });

            // Upsert the rating
            let normalized_name = player.name.trim();
            sqlx::query(
                "INSERT INTO player_elo (player_name, game_type, elo, games_played, updated_at)
                 VALUES ($1, $2, $3, 1, NOW())
                 ON CONFLICT (player_name, game_type) 
                 DO UPDATE SET elo = $3, games_played = player_elo.games_played + 1, updated_at = NOW()",
            )
            .bind(normalized_name)
            .bind(&game_type_str)
            .bind(new_rating)
            .execute(&mut *tx)
            .await?;

            // Record history for tracking trends (use game's created_at for historical accuracy)
            sqlx::query(
                "INSERT INTO player_elo_history (player_name, game_type, elo, game_id, recorded_at)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(normalized_name)
            .bind(&game_type_str)
            .bind(new_rating)
            .bind(&game.id)
            .bind(game.created_at)
            .execute(&mut *tx)
            .await?;

            // Store the rating change in the game_elo_changes table
            sqlx::query(
                "INSERT INTO game_elo_changes (game_id, player_name, elo_before, elo_after, elo_change)
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&game.id)
            .bind(normalized_name)
            .bind(elo_before)
            .bind(new_rating)
            .bind(actual_change)
            .execute(&mut *tx)
            .await?;
        }

        // Commit the transaction
        tx.commit().await?;

        Ok(elo_changes)
    }

    /// Get ELO changes for a specific game
    pub async fn get_game_elo_changes(&self, game_id: &str) -> DbResult<Vec<GameEloChange>> {
        let rows = sqlx::query(
            "SELECT player_name, elo_before, elo_after, elo_change 
             FROM game_elo_changes 
             WHERE game_id = $1 
             ORDER BY created_at ASC"
        )
        .bind(game_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row| GameEloChange {
            player_name: row.get("player_name"),
            elo_before: row.get("elo_before"),
            elo_after: row.get("elo_after"),
            elo_change: row.get("elo_change"),
        }).collect())
    }

    /// Get ELO changes for a player across all games
    /// Returns a map of game_id -> elo_change for the given player
    pub async fn get_player_elo_changes(&self, player_name: &str) -> DbResult<std::collections::HashMap<String, i32>> {
        let rows = sqlx::query(
            "SELECT game_id, elo_change 
             FROM game_elo_changes 
             WHERE player_name = $1"
        )
        .bind(player_name)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|row| {
            let game_id: String = row.get("game_id");
            let elo_change: i32 = row.get("elo_change");
            (game_id, elo_change)
        }).collect())
    }

    /// Recalculate all ELO ratings from game history
    /// This clears all existing ELO data and recalculates from scratch
    pub async fn recalculate_all_elo(&self) -> DbResult<(usize, usize)> {
        // Clear existing ELO data, history, and game changes
        sqlx::query("DELETE FROM player_elo")
            .execute(&self.pool)
            .await?;
        sqlx::query("DELETE FROM player_elo_history")
            .execute(&self.pool)
            .await?;
        sqlx::query("DELETE FROM game_elo_changes")
            .execute(&self.pool)
            .await?;

        // Get all games sorted by date (oldest first for chronological processing)
        // We check game_over in code since there's no 'finished' column in the DB
        let game_rows = sqlx::query("SELECT id FROM games ORDER BY created_at ASC")
            .fetch_all(&self.pool)
            .await?;

        let mut games_processed = 0;
        let mut players_updated = std::collections::HashSet::new();

        for row in game_rows {
            let game_id: String = row.get("id");
            if let Ok(game) = self.get_game(&game_id).await {
                // Only process finished games
                if !game.game_over {
                    continue;
                }
                
                // Skip empty games (no rounds or all zero scores)
                if is_empty_game(&game) {
                    continue;
                }

                // Track unique players
                for player in &game.players {
                    players_updated.insert(player.name.trim().to_lowercase());
                }

                // Update ELO for this game
                if let Err(e) = self.update_elo_for_game(&game).await {
                    log::warn!("Failed to update ELO for game {}: {}", game_id, e);
                } else {
                    games_processed += 1;
                }
            }
        }

        Ok((games_processed, players_updated.len()))
    }

    /// Rename a player in the ELO table
    pub async fn rename_player_elo(&self, old_name: &str, new_name: &str) -> DbResult<u64> {
        let old_normalized = old_name.trim().to_lowercase();
        let new_trimmed = new_name.trim();

        // Check if new name already has ELO entries (case-insensitive)
        let new_normalized = new_trimmed.to_lowercase();

        // Get all ELO entries for old name
        let old_entries = sqlx::query(
            "SELECT game_type, elo, games_played FROM player_elo WHERE LOWER(player_name) = $1",
        )
        .bind(&old_normalized)
        .fetch_all(&self.pool)
        .await?;

        if old_entries.is_empty() {
            return Ok(0);
        }

        // For each game type, merge or rename
        for entry in &old_entries {
            let game_type: String = entry.get("game_type");
            let old_elo: i32 = entry.get("elo");
            let old_games: i32 = entry.get("games_played");

            // Check if new name already has an entry for this game type
            let existing = sqlx::query(
                "SELECT elo, games_played FROM player_elo WHERE LOWER(player_name) = $1 AND game_type = $2",
            )
            .bind(&new_normalized)
            .bind(&game_type)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(existing_row) = existing {
                // Merge: weighted average based on games played
                let existing_elo: i32 = existing_row.get("elo");
                let existing_games: i32 = existing_row.get("games_played");
                let total_games = old_games + existing_games;
                let merged_elo = if total_games > 0 {
                    (old_elo * old_games + existing_elo * existing_games) / total_games
                } else {
                    1000
                };

                // Update the existing entry with merged values
                sqlx::query(
                    "UPDATE player_elo SET elo = $1, games_played = $2, updated_at = NOW() 
                     WHERE LOWER(player_name) = $3 AND game_type = $4",
                )
                .bind(merged_elo)
                .bind(total_games)
                .bind(&new_normalized)
                .bind(&game_type)
                .execute(&self.pool)
                .await?;
            } else {
                // Just rename
                sqlx::query(
                    "UPDATE player_elo SET player_name = $1, updated_at = NOW() 
                     WHERE LOWER(player_name) = $2 AND game_type = $3",
                )
                .bind(new_trimmed)
                .bind(&old_normalized)
                .bind(&game_type)
                .execute(&self.pool)
                .await?;
            }
        }

        // Delete any remaining entries for old name (in case of merge)
        let _result = sqlx::query("DELETE FROM player_elo WHERE LOWER(player_name) = $1")
            .bind(&old_normalized)
            .execute(&self.pool)
            .await?;

        // Also rename in ELO history
        sqlx::query("UPDATE player_elo_history SET player_name = $1 WHERE LOWER(player_name) = $2")
            .bind(new_trimmed)
            .bind(&old_normalized)
            .execute(&self.pool)
            .await?;

        Ok(old_entries.len() as u64)
    }

    /// Get ELO history for a player, optionally filtered by game type and time range
    /// time_range: "month", "year", or "all"
    pub async fn get_player_elo_history(
        &self,
        player_name: &str,
        game_type: Option<&GameType>,
        time_range: &str,
    ) -> DbResult<Vec<EloHistoryEntry>> {
        let normalized_name = player_name.trim().to_lowercase();

        let time_filter = match time_range {
            "month" => "AND recorded_at >= NOW() - INTERVAL '30 days'",
            "year" => "AND recorded_at >= NOW() - INTERVAL '365 days'",
            _ => "", // "all" or any other value = no filter
        };

        let rows = if let Some(gt) = game_type {
            let game_type_str = serde_json::to_string(gt)?;
            let query = format!(
                "SELECT player_name, game_type, elo, game_id, recorded_at 
                 FROM player_elo_history 
                 WHERE LOWER(player_name) = $1 AND game_type = $2 {}
                 ORDER BY recorded_at ASC",
                time_filter
            );
            sqlx::query(&query)
                .bind(&normalized_name)
                .bind(&game_type_str)
                .fetch_all(&self.pool)
                .await?
        } else {
            // For overall: compute running average of all game types at each point in time
            // We need to calculate the average ELO across all game types at each history entry
            let query = format!(
                "WITH ordered_history AS (
                    SELECT player_name, game_type, elo, game_id, recorded_at,
                           ROW_NUMBER() OVER (PARTITION BY game_type ORDER BY recorded_at DESC) as rn
                    FROM player_elo_history 
                    WHERE LOWER(player_name) = $1 {}
                ),
                latest_per_type AS (
                    SELECT DISTINCT ON (game_type) game_type, elo
                    FROM player_elo_history
                    WHERE LOWER(player_name) = $1 {}
                    ORDER BY game_type, recorded_at DESC
                ),
                all_entries AS (
                    SELECT player_name, 'overall' as game_type, 
                           (SELECT AVG(l2.elo)::integer 
                            FROM (
                                SELECT DISTINCT ON (game_type) game_type, elo
                                FROM player_elo_history h2
                                WHERE LOWER(h2.player_name) = $1 
                                  AND h2.recorded_at <= h.recorded_at
                                ORDER BY game_type, recorded_at DESC
                            ) l2
                           ) as elo, 
                           game_id, 
                           recorded_at
                    FROM player_elo_history h
                    WHERE LOWER(player_name) = $1 {}
                )
                SELECT DISTINCT ON (DATE_TRUNC('day', recorded_at)) 
                       player_name, game_type, elo, game_id, recorded_at
                FROM all_entries
                WHERE elo IS NOT NULL
                ORDER BY DATE_TRUNC('day', recorded_at) ASC, recorded_at DESC",
                time_filter, time_filter, time_filter
            );
            sqlx::query(&query)
                .bind(&normalized_name)
                .fetch_all(&self.pool)
                .await?
        };

        let results = rows
            .into_iter()
            .map(|row| EloHistoryEntry {
                player_name: row.get("player_name"),
                game_type: row.get("game_type"),
                elo: row.get("elo"),
                game_id: row.get("game_id"),
                recorded_at: row.get("recorded_at"),
            })
            .collect();

        Ok(results)
    }

    /// Get player's ELO from N days ago for comparison (returns None if no history that far back)
    pub async fn get_player_elo_days_ago(
        &self,
        player_name: &str,
        game_type: Option<&GameType>,
        days_ago: i32,
    ) -> DbResult<Option<i32>> {
        let normalized_name = player_name.trim().to_lowercase();

        // Get the most recent ELO recorded before the target date
        let row = if let Some(gt) = game_type {
            let game_type_str = serde_json::to_string(gt)?;
            sqlx::query(
                "SELECT elo FROM player_elo_history 
                 WHERE LOWER(player_name) = $1 AND game_type = $2 
                   AND recorded_at <= NOW() - ($3 || ' days')::interval
                 ORDER BY recorded_at DESC
                 LIMIT 1",
            )
            .bind(&normalized_name)
            .bind(&game_type_str)
            .bind(days_ago.to_string())
            .fetch_optional(&self.pool)
            .await?
        } else {
            // For overall, we get the average of the most recent ELO per game type before target date
            sqlx::query(
                "SELECT AVG(sub.elo)::integer as elo FROM (
                    SELECT DISTINCT ON (game_type) elo 
                    FROM player_elo_history 
                    WHERE LOWER(player_name) = $1 
                      AND recorded_at <= NOW() - ($2 || ' days')::interval
                    ORDER BY game_type, recorded_at DESC
                 ) sub",
            )
            .bind(&normalized_name)
            .bind(days_ago.to_string())
            .fetch_optional(&self.pool)
            .await?
        };

        Ok(row.and_then(|r| r.get::<Option<i32>, _>("elo")))
    }
}

/// Compute placements for ELO calculation (1 = first, 2 = second, etc.)
fn compute_placements_for_elo(game: &Game) -> Vec<usize> {
    if game.players.is_empty() {
        return Vec::new();
    }

    let mut indices: Vec<usize> = (0..game.players.len()).collect();

    // Sort by score - for Hearts lower is better, for others higher is better
    indices.sort_by(|a, b| match game.game_type {
        GameType::Hearts => game.players[*a]
            .total_score
            .cmp(&game.players[*b].total_score),
        _ => game.players[*b]
            .total_score
            .cmp(&game.players[*a].total_score),
    });

    let mut placements = vec![0; game.players.len()];
    let mut current_rank = 1usize;
    let mut last_score: Option<i32> = None;

    for (position, idx) in indices.iter().enumerate() {
        let score = game.players[*idx].total_score;
        if let Some(previous) = last_score {
            if previous != score {
                current_rank = position + 1;
            }
        }
        placements[*idx] = current_rank;
        last_score = Some(score);
    }

    placements
}

/// Check if a game is "empty" (no rounds played or all scores are 0)
/// Empty games should be excluded from ELO calculations
fn is_empty_game(game: &Game) -> bool {
    // No rounds played
    if game.current_round == 0 {
        return true;
    }
    
    // All players have empty scores or all zeros
    game.players.iter().all(|player| {
        player.scores.is_empty() || player.scores.iter().all(|&s| s == 0)
    })
}
