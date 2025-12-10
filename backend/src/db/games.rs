//! Game CRUD operations

use crate::models::{AddRoundRequest, Game, GameType, Player};
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use super::{Database, DbError, DbResult};

impl Database {
    /// Create a new game with the given players
    /// Uses a transaction to ensure game and players are created atomically
    pub async fn create_game(
        &self,
        game_type: GameType,
        player_names: Vec<String>,
    ) -> DbResult<Game> {
        let game_id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let current_round = 0;
        let game_type_str = serde_json::to_string(&game_type)?;

        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Insert game
        sqlx::query(
            "INSERT INTO games (id, game_type, created_at, current_round) VALUES ($1, $2, $3, $4)",
        )
        .bind(&game_id)
        .bind(&game_type_str)
        .bind(created_at)
        .bind(current_round as i32)
        .execute(&mut *tx)
        .await?;

        // Insert players
        for (index, name) in player_names.iter().enumerate() {
            let player_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO players (id, game_id, name, position) VALUES ($1, $2, $3, $4)",
            )
            .bind(&player_id)
            .bind(&game_id)
            .bind(name)
            .bind(index as i32)
            .execute(&mut *tx)
            .await?;
        }

        // Commit the transaction
        tx.commit().await?;

        // Fetch and return the created game
        let game = self.get_game(&game_id).await?;
        Ok(game)
    }

    pub async fn get_game(&self, game_id: &str) -> DbResult<Game> {
        // Get game info
        let game_row =
            sqlx::query("SELECT id, game_type, created_at, current_round, COALESCE(updated_at, created_at) as updated_at FROM games WHERE id = $1")
                .bind(game_id)
                .fetch_optional(&self.pool)
                .await?
                .ok_or(DbError::GameNotFound)?;

        let game_id: String = game_row.get("id");
        let game_type_str: String = game_row.get("game_type");
        let game_type: GameType = serde_json::from_str(&game_type_str)
            .map_err(|e| DbError::InvalidData(format!("Invalid game_type: {}", e)))?;
        let created_at: DateTime<Utc> = game_row.get("created_at");
        let updated_at: DateTime<Utc> = game_row.get("updated_at");
        let current_round = game_row.get::<i32, _>("current_round") as usize;

        // Get players and their scores
        let player_rows = sqlx::query(
            "SELECT id, name, position, scores, game_indices FROM players WHERE game_id = $1 ORDER BY position"
        )
        .bind(&game_id)
        .fetch_all(&self.pool)
        .await?;

        let players: Vec<Player> = player_rows
            .into_iter()
            .map(|row| {
                let player_id: String = row.get("id");
                let name: String = row.get("name");
                let scores_value: serde_json::Value = row.get("scores");
                let scores: Vec<i32> = serde_json::from_value(scores_value).unwrap_or_default();
                let game_indices_value: serde_json::Value = row.get("game_indices");
                let game_indices: Vec<usize> =
                    serde_json::from_value(game_indices_value).unwrap_or_default();
                let total_score = compute_total_score(&game_type, &scores);

                Player {
                    id: player_id,
                    name,
                    scores,
                    game_indices,
                    total_score,
                }
            })
            .collect();

        // Determine if game is over based on game type and rounds played
        let game_over = match game_type {
            GameType::Hearts => {
                // Hearts ends when any player's total >= 100
                players.iter().any(|p| p.total_score >= 100)
            }
            GameType::King => {
                // King ends after exactly 10 rounds
                current_round >= 10
            }
            GameType::DoubleKing => {
                // Double King ends after all rounds: 20 for 4p, 18 for 3p
                let max_rounds = if players.len() == 3 { 18 } else { 20 };
                current_round >= max_rounds
            }
            GameType::ColorWhist => {
                // Color Whist has no fixed end - consider finished if inactive for 12 hours
                // and at least one round has been played
                if current_round == 0 {
                    false
                } else {
                    let hours_since_update = Utc::now().signed_duration_since(updated_at).num_hours();
                    hours_since_update >= 12
                }
            }
            GameType::Whist => {
                // Whist has no fixed end - consider finished if inactive for 12 hours
                // and at least one round has been played
                if current_round == 0 {
                    false
                } else {
                    let hours_since_update = Utc::now().signed_duration_since(updated_at).num_hours();
                    hours_since_update >= 12
                }
            }
            GameType::Manille => {
                // Manille ends when a team reaches the target score (default 101)
                // Check if either team has reached 101+
                // Team 1: players 0,2  Team 2: players 1,3
                if players.len() == 4 {
                    let team1_score = players[0].total_score;
                    let team2_score = players[1].total_score;
                    team1_score >= 101 || team2_score >= 101
                } else {
                    false
                }
            }
            GameType::Press => {
                // Press ends when a team reaches 42 points
                // Team 1: players 0,2  Team 2: players 1,3
                if players.len() == 4 {
                    let team1_score = players[0].total_score;
                    let team2_score = players[1].total_score;
                    team1_score >= 42 || team2_score >= 42
                } else {
                    false
                }
            }
        };

        Ok(Game {
            id: game_id,
            game_type,
            players,
            created_at,
            current_round,
            game_over,
        })
    }

    pub async fn get_all_games(&self) -> DbResult<Vec<Game>> {
        let game_rows = sqlx::query("SELECT id FROM games ORDER BY created_at DESC LIMIT 100")
            .fetch_all(&self.pool)
            .await?;

        let mut games = Vec::new();
        for row in game_rows {
            let game_id: String = row.get("id");
            if let Ok(game) = self.get_game(&game_id).await {
                games.push(game);
            }
        }

        Ok(games)
    }

    pub async fn get_all_games_full(&self) -> DbResult<Vec<Game>> {
        let game_rows = sqlx::query("SELECT id FROM games ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut games = Vec::new();
        for row in game_rows {
            let game_id: String = row.get("id");
            if let Ok(game) = self.get_game(&game_id).await {
                games.push(game);
            }
        }

        Ok(games)
    }

    /// Delete a game and all its players
    /// Uses a transaction to ensure atomic deletion
    pub async fn delete_game(&self, game_id: &str) -> DbResult<()> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Delete players first (foreign key constraint)
        sqlx::query("DELETE FROM players WHERE game_id = $1")
            .bind(game_id)
            .execute(&mut *tx)
            .await?;

        // Delete game
        let result = sqlx::query("DELETE FROM games WHERE id = $1")
            .bind(game_id)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::GameNotFound);
        }

        // Commit the transaction
        tx.commit().await?;

        Ok(())
    }
    
    /// Add a round to a game
    /// Uses a transaction to ensure all player scores and round counter
    /// are updated atomically
    pub async fn add_round(
        &self,
        game_id: &str,
        request: AddRoundRequest,
        game_service: &dyn crate::services::GameService,
    ) -> DbResult<Game> {
        // Get the current game state
        let game = self.get_game(game_id).await?;

        // Use the appropriate game service to calculate the final round scores
        let round_scores = game_service
            .calculate_round_scores(&game, &request)
            .map_err(DbError::GameLogicError)?;

        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // Update each player's scores in the database
        for (player, &score) in game.players.iter().zip(round_scores.iter()) {
            let mut new_scores = player.scores.clone();
            new_scores.push(score);
            let mut new_game_indices = player.game_indices.clone();
            if let Some(idx) = request.game_index {
                new_game_indices.push(idx);
            }

            let scores_json = serde_json::to_value(&new_scores)?;
            let game_indices_json = serde_json::to_value(&new_game_indices)?;
            sqlx::query("UPDATE players SET scores = $1, game_indices = $2 WHERE id = $3")
                .bind(scores_json)
                .bind(game_indices_json)
                .bind(&player.id)
                .execute(&mut *tx)
                .await?;
        }

        // Increment round counter and update timestamp
        sqlx::query("UPDATE games SET current_round = $1, updated_at = NOW() WHERE id = $2")
            .bind(game.current_round as i32 + 1)
            .bind(game_id)
            .execute(&mut *tx)
            .await?;

        // Commit the transaction
        tx.commit().await?;

        // Return the canonical game from the database so callers get the persisted state
        self.get_game(game_id).await
    }

    /// Update a specific round in a game
    /// Uses a transaction to ensure all player scores are updated atomically
    pub async fn update_round(
        &self,
        game: Game,
        round_index: usize,
        mut request: AddRoundRequest,
        game_service: &dyn crate::services::GameService,
    ) -> DbResult<Game> {
        if round_index >= game.current_round {
            return Err(DbError::InvalidData(format!(
                "Round {} does not exist for game {}",
                round_index + 1,
                game.id
            )));
        }

        if matches!(game.game_type, GameType::DoubleKing) && request.game_index.is_none() {
            if let Some(existing_index) = game
                .players
                .first()
                .and_then(|p| p.game_indices.get(round_index))
            {
                request.game_index = Some(*existing_index);
            }
        }

        if matches!(game.game_type, GameType::DoubleKing) && request.game_index.is_none() {
            return Err(DbError::InvalidData(
                "Double King edits require the game_index used for the round".to_string(),
            ));
        }

        let mut partial_game = game.clone();
        partial_game.current_round = round_index;
        partial_game.players = game
            .players
            .iter()
            .map(|player| {
                let truncated_scores = if round_index > player.scores.len() {
                    player.scores.clone()
                } else {
                    player.scores[..round_index].to_vec()
                };
                let truncated_indices = if round_index > player.game_indices.len() {
                    player.game_indices.clone()
                } else {
                    player.game_indices[..round_index].to_vec()
                };
                let total_score = compute_total_score(&game.game_type, &truncated_scores);

                Player {
                    id: player.id.clone(),
                    name: player.name.clone(),
                    scores: truncated_scores,
                    game_indices: truncated_indices,
                    total_score,
                }
            })
            .collect();

        let round_scores = game_service
            .calculate_round_scores(&partial_game, &request)
            .map_err(DbError::GameLogicError)?;

        if round_scores.len() != game.players.len() {
            return Err(DbError::InvalidData(
                "Calculated scores did not match player count".to_string(),
            ));
        }

        // Start a transaction
        let mut tx = self.pool.begin().await?;

        for (player, &score) in game.players.iter().zip(round_scores.iter()) {
            if round_index >= player.scores.len() {
                return Err(DbError::InvalidData(format!(
                    "Round {} missing score entry for player {}",
                    round_index + 1,
                    player.id
                )));
            }

            let mut updated_scores = player.scores.clone();
            updated_scores[round_index] = score;

            let mut updated_game_indices = player.game_indices.clone();
            if let Some(idx) = request.game_index {
                if round_index < updated_game_indices.len() {
                    updated_game_indices[round_index] = idx;
                } else if round_index == updated_game_indices.len() {
                    updated_game_indices.push(idx);
                } else {
                    return Err(DbError::InvalidData(
                        "Inconsistent game index history for player".to_string(),
                    ));
                }
            }

            let scores_json = serde_json::to_value(&updated_scores)?;
            let game_indices_json = serde_json::to_value(&updated_game_indices)?;
            sqlx::query("UPDATE players SET scores = $1, game_indices = $2 WHERE id = $3")
                .bind(scores_json)
                .bind(game_indices_json)
                .bind(&player.id)
                .execute(&mut *tx)
                .await?;
        }

        // Update the game's updated_at timestamp
        sqlx::query("UPDATE games SET updated_at = NOW() WHERE id = $1")
            .bind(&game.id)
            .execute(&mut *tx)
            .await?;

        // Commit the transaction
        tx.commit().await?;

        self.get_game(&game.id).await
    }
}

/// Compute total score for a player, handling Hearts' reset at 100 rule
pub(crate) fn compute_total_score(game_type: &GameType, scores: &[i32]) -> i32 {
    let mut total_score: i32 = 0;
    for s in scores {
        total_score += *s;
        if let GameType::Hearts = game_type {
            if total_score == 100 {
                total_score = 0;
            }
        }
    }
    total_score
}

impl Database {
    /// Delete games where all player scores sum to 0 and older than specified hours
    /// Returns the number of games deleted
    pub async fn delete_empty_games(&self, hours_old: i64) -> DbResult<usize> {
        // Find all games older than the threshold
        let game_rows = sqlx::query(
            "SELECT g.id FROM games g
             WHERE g.created_at < NOW() - ($1 || ' hours')::interval"
        )
        .bind(hours_old.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut deleted_count = 0;

        for row in game_rows {
            let game_id: String = row.get("id");
            
            // Get the game and check if it's empty
            if let Ok(game) = self.get_game(&game_id).await {
                // Check if all scores are 0
                let all_scores_zero = game.players.iter().all(|p| {
                    p.scores.is_empty() || p.scores.iter().all(|&s| s == 0)
                });
                
                // Also check if there are no rounds played
                let no_rounds_played = game.current_round == 0;
                
                if all_scores_zero && no_rounds_played {
                    // Delete the game
                    if self.delete_game(&game_id).await.is_ok() {
                        log::debug!("Deleted empty game: {}", game_id);
                        deleted_count += 1;
                    }
                }
            }
        }

        Ok(deleted_count)
    }

    /// Backfill ELO history for finished games that don't have history entries
    /// Uses the game's created_at timestamp as the recorded_at date
    /// Returns the number of games backfilled
    pub async fn backfill_elo_history(&self) -> DbResult<usize> {
        // Get all games that might need backfilling
        let game_rows = sqlx::query(
            "SELECT id FROM games ORDER BY created_at ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut backfilled_count = 0;
        let config = self.get_elo_config().await;

        for row in game_rows {
            let game_id: String = row.get("id");
            
            // Check if this game already has ELO history entries
            let existing_history = sqlx::query(
                "SELECT COUNT(*) as count FROM player_elo_history WHERE game_id = $1"
            )
            .bind(&game_id)
            .fetch_one(&self.pool)
            .await?;
            
            let count: i64 = existing_history.get("count");
            if count > 0 {
                // Already has history, skip
                continue;
            }

            // Get the game
            if let Ok(game) = self.get_game(&game_id).await {
                // Only process finished games
                if !game.game_over {
                    continue;
                }
                
                // Check that the game has actual scores (not empty)
                let has_scores = game.players.iter().any(|p| {
                    !p.scores.is_empty() && p.scores.iter().any(|&s| s != 0)
                });
                
                if !has_scores {
                    continue;
                }

                // Get current ELO for all players in this game type
                let game_type_str = serde_json::to_string(&game.game_type)?;
                
                for player in &game.players {
                    let normalized_name = player.name.trim();
                    
                    // Get or create the player's current ELO for this game type
                    let current_elo = sqlx::query(
                        "SELECT elo FROM player_elo WHERE player_name = $1 AND game_type = $2"
                    )
                    .bind(normalized_name)
                    .bind(&game_type_str)
                    .fetch_optional(&self.pool)
                    .await?;
                    
                    let elo = current_elo
                        .map(|r| r.get::<i32, _>("elo"))
                        .unwrap_or(config.starting_elo);
                    
                    // Insert history entry using the game's created_at date
                    sqlx::query(
                        "INSERT INTO player_elo_history (player_name, game_type, elo, game_id, recorded_at)
                         VALUES ($1, $2, $3, $4, $5)
                         ON CONFLICT DO NOTHING"
                    )
                    .bind(normalized_name)
                    .bind(&game_type_str)
                    .bind(elo)
                    .bind(&game_id)
                    .bind(game.created_at) // Use game.created_at from the Game object
                    .execute(&self.pool)
                    .await?;
                }
                
                backfilled_count += 1;
            }
        }

        Ok(backfilled_count)
    }
}
