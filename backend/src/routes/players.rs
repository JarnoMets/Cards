//! Player search, profile, and management endpoints

use crate::db::PlayerSearchRow;
use crate::models::{AppState, Game, GameType};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::common::{
    compute_placements, compute_score_margin, determine_winner_indices, leaderboard_game_types,
    normalize_player_name, GameParticipantSummary, PlayerDetailResponse, PlayerGameStat,
    PlayerGameSummary, PlayerGameTypeAggregate, PlayerScorePoint, PlayerSearchParams,
    PlayerSearchResult, RenamePlayerRequest, RenamePlayerResponse,
};

/// Search for players by name
pub async fn search_players(
    data: web::Data<AppState>,
    params: web::Query<PlayerSearchParams>,
) -> impl Responder {
    let limit = params.limit.unwrap_or(12).clamp(1, 50);
    let query = params.q.clone();
    let db = data.db.clone();

    match db.search_players(query.as_deref(), limit as i64).await {
        Ok(rows) => {
            let payload: Vec<PlayerSearchResult> = rows
                .into_iter()
                .map(|row: PlayerSearchRow| PlayerSearchResult {
                    player_name: row.name,
                    games_played: row.games_played as u32,
                    last_played_at: row.last_played_at,
                })
                .collect();
            HttpResponse::Ok().json(payload)
        }
        Err(e) => {
            log::error!("Error searching players: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to search players"
            }))
        }
    }
}

/// Get detailed profile for a player
pub async fn get_player_profile(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let player_name = path.into_inner();
    if player_name.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Player name is required"
        }));
    }

    let db = data.db.clone();
    match db.get_games_for_player(&player_name).await {
        Ok(games) => {
            if games.is_empty() {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Player not found"
                }));
            }

            // Fetch ELO changes for this player
            let elo_changes = db.get_player_elo_changes(&player_name).await.ok();

            match build_player_detail_response(&player_name, games, elo_changes) {
                Some(profile) => HttpResponse::Ok().json(profile),
                None => HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Player not found"
                })),
            }
        }
        Err(e) => {
            log::error!("Error fetching player profile for '{}': {}", player_name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to load player profile"
            }))
        }
    }
}

/// Get all games for a specific player
pub async fn get_player_games(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let player_name = path.into_inner();
    if player_name.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Player name is required"
        }));
    }

    let db = data.db.clone();
    match db.get_games_for_player(&player_name).await {
        Ok(games) => {
            if games.is_empty() {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Player not found"
                }));
            }
            HttpResponse::Ok().json(games)
        }
        Err(e) => {
            log::error!("Error fetching games for player '{}': {}", player_name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to load player games"
            }))
        }
    }
}

/// Rename a player globally across all games
pub async fn rename_player_globally(
    data: web::Data<AppState>,
    req: web::Json<RenamePlayerRequest>,
) -> impl Responder {
    let db = data.db.clone();

    match db
        .rename_player_globally(&req.old_name, &req.new_name)
        .await
    {
        Ok(count) => {
            log::info!(
                "Player '{}' renamed to '{}' in {} game(s)",
                req.old_name,
                req.new_name,
                count
            );
            HttpResponse::Ok().json(RenamePlayerResponse {
                message: format!(
                    "Player '{}' renamed to '{}' in {} game(s)",
                    req.old_name, req.new_name, count
                ),
                games_updated: count,
            })
        }
        Err(e) => {
            log::error!("Error renaming player: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

/// Build detailed player profile from games data
fn build_player_detail_response(
    player_name: &str,
    games: Vec<Game>,
    elo_changes: Option<HashMap<String, i32>>,
) -> Option<PlayerDetailResponse> {
    let normalized = normalize_player_name(player_name);
    if normalized.is_empty() {
        return None;
    }

    let mut matches: Vec<(Game, usize)> = Vec::new();
    let mut canonical_name: Option<String> = None;

    for game in games.into_iter() {
        // Skip empty games (no rounds or all zero scores)
        if is_empty_game(&game) {
            continue;
        }
        
        if let Some(idx) = game
            .players
            .iter()
            .position(|player| normalize_player_name(&player.name) == normalized)
        {
            if canonical_name.is_none() {
                canonical_name = game.players.get(idx).map(|p| p.name.clone());
            }
            matches.push((game, idx));
        }
    }

    if matches.is_empty() {
        return None;
    }

    matches.sort_by(|(left, _), (right, _)| left.created_at.cmp(&right.created_at));

    let mut total_games = 0u32;
    let mut finished_games = 0u32;
    let mut unfinished_games = 0u32;
    let mut total_wins = 0u32;
    let mut total_points: i64 = 0;
    let mut total_rounds = 0u32;
    let mut first_played_at: Option<DateTime<Utc>> = None;
    let mut last_played_at: Option<DateTime<Utc>> = None;
    let mut stats_map: HashMap<GameType, PlayerGameTypeAggregate> = HashMap::new();
    let mut score_timeline: Vec<PlayerScorePoint> = Vec::new();
    let mut recent_games: Vec<PlayerGameSummary> = Vec::new();

    for (game, player_index) in matches.iter() {
        let player_idx = *player_index;
        let Some(player) = game.players.get(player_idx) else {
            continue;
        };

        total_games += 1;
        total_points += player.total_score as i64;
        
        // Count rounds for this player
        let player_rounds = player.scores.len() as u32;
        total_rounds += player_rounds;
        
        // Track finished vs unfinished
        if game.game_over {
            finished_games += 1;
        } else {
            unfinished_games += 1;
        }

        let occurred_at = game.created_at;
        if first_played_at.map_or(true, |ts| occurred_at < ts) {
            first_played_at = Some(occurred_at);
        }
        if last_played_at.map_or(true, |ts| occurred_at > ts) {
            last_played_at = Some(occurred_at);
        }

        let placements = compute_placements(game);
        let placement = placements
            .get(player_idx)
            .copied()
            .unwrap_or_else(|| game.players.len());
        
        // Only count wins for finished games
        let did_win = if game.game_over {
            determine_winner_indices(game)
                .map(|winners| winners.contains(&player_idx))
                .unwrap_or(false)
        } else {
            false
        };
        if did_win {
            total_wins += 1;
        }

        let margin = compute_score_margin(game, player.total_score);

        let entry = stats_map
            .entry(game.game_type.clone())
            .or_insert_with(PlayerGameTypeAggregate::default);
        entry.games_played += 1;
        entry.total_points += player.total_score as i64;
        entry.total_rounds += player_rounds;
        
        if game.game_over {
            entry.finished_games += 1;
            if did_win {
                entry.wins += 1;
            }
        } else {
            entry.unfinished_games += 1;
        }
        
        match entry.highest_score {
            Some(existing) if player.total_score <= existing => {}
            _ => entry.highest_score = Some(player.total_score),
        }
        match entry.lowest_score {
            Some(existing) if player.total_score >= existing => {}
            _ => entry.lowest_score = Some(player.total_score),
        }

        score_timeline.push(PlayerScorePoint {
            game_id: game.id.clone(),
            game_type: game.game_type.clone(),
            occurred_at,
            total_score: player.total_score,
            placement,
            score_margin: margin,
            did_win,
        });

        let participants: Vec<GameParticipantSummary> = game
            .players
            .iter()
            .enumerate()
            .map(|(idx, participant)| GameParticipantSummary {
                name: participant.name.clone(),
                total_score: participant.total_score,
                placement: placements.get(idx).copied().unwrap_or(game.players.len()),
                is_target: idx == player_idx,
            })
            .collect();

        // Get ELO change for this game if available
        let elo_change = elo_changes
            .as_ref()
            .and_then(|changes| changes.get(&game.id).copied());

        recent_games.push(PlayerGameSummary {
            game_id: game.id.clone(),
            game_type: game.game_type.clone(),
            occurred_at,
            player_score: player.total_score,
            placement,
            total_players: game.players.len(),
            did_win,
            score_margin: margin,
            players: participants,
            elo_change,
        });
    }

    recent_games.sort_by(|a, b| b.occurred_at.cmp(&a.occurred_at));
    recent_games.truncate(20);

    let ordering = leaderboard_game_types();
    let stats_by_game_type: Vec<PlayerGameStat> = ordering
        .into_iter()
        .filter_map(|gt| {
            stats_map.get(&gt).map(|agg| PlayerGameStat {
                game_type: gt,
                wins: agg.wins,
                games_played: agg.games_played,
                finished_games: agg.finished_games,
                unfinished_games: agg.unfinished_games,
                total_rounds: agg.total_rounds,
                average_score: if agg.games_played > 0 {
                    agg.total_points as f64 / agg.games_played as f64
                } else {
                    0.0
                },
                // Win rate is based on finished games only
                win_rate: if agg.finished_games > 0 {
                    agg.wins as f64 / agg.finished_games as f64
                } else {
                    0.0
                },
                highest_score: agg.highest_score,
                lowest_score: agg.lowest_score,
                total_points: agg.total_points,
            })
        })
        .collect();

    // Win rate is based on finished games only
    let overall_win_rate = if finished_games > 0 {
        total_wins as f64 / finished_games as f64
    } else {
        0.0
    };

    Some(PlayerDetailResponse {
        player_name: canonical_name.unwrap_or_else(|| player_name.to_string()),
        total_games,
        finished_games,
        unfinished_games,
        total_wins,
        overall_win_rate,
        total_points,
        total_rounds,
        first_played_at,
        last_played_at,
        stats_by_game_type,
        score_timeline,
        recent_games,
    })
}

/// Check if a game is "empty" (no rounds played or all scores are 0)
/// Empty games should be excluded from statistics
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

// =============================================================================
// Player Settings Endpoints
// =============================================================================

use crate::db::PlayerSettings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdatePlayerSettingsRequest {
    pub email: Option<String>,
    pub game_notifications: bool,
}

/// Get player settings
pub async fn get_player_settings(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let player_name = path.into_inner();
    if player_name.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Player name is required"
        }));
    }

    let db = data.db.clone();
    match db.get_player_settings(&player_name).await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(e) => {
            log::error!("Error fetching player settings for '{}': {}", player_name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to load player settings"
            }))
        }
    }
}

/// Update player settings
pub async fn update_player_settings(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<UpdatePlayerSettingsRequest>,
) -> impl Responder {
    let player_name = path.into_inner();
    if player_name.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Player name is required"
        }));
    }

    // Validate email format if provided
    if let Some(ref email) = req.email {
        if !email.is_empty() && !email.contains('@') {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid email format"
            }));
        }
    }

    let settings = PlayerSettings {
        player_name: player_name.clone(),
        email: req.email.clone().filter(|e| !e.is_empty()),
        game_notifications: req.game_notifications,
    };

    let db = data.db.clone();
    match db.update_player_settings(&settings).await {
        Ok(_) => {
            log::info!("Updated settings for player '{}'", player_name);
            HttpResponse::Ok().json(settings)
        }
        Err(e) => {
            log::error!("Error updating player settings for '{}': {}", player_name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update player settings"
            }))
        }
    }
}

