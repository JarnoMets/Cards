//! Leaderboard and statistics endpoints

use crate::models::{AppState, Game, GameType};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use std::collections::HashMap;

use super::common::{
    determine_winner_indices, leaderboard_game_types, update_records, GameRecordEntry,
    LeaderboardRecords, LeaderboardResponse, PlayerAggregate, PlayerGameStat,
    PlayerLeaderboardEntry,
};

/// Cache TTL in seconds (30 seconds default)
const CACHE_TTL_SECONDS: i64 = 30;

/// Get leaderboard statistics with in-memory caching
pub async fn get_leaderboard(data: web::Data<AppState>) -> impl Responder {
    // First, check if we have valid cached data
    {
        let cache = data.leaderboard_cache.read().await;
        if cache.is_valid(CACHE_TTL_SECONDS) {
            if let Some(ref cached_data) = cache.data {
                log::debug!("Serving leaderboard from cache");
                return HttpResponse::Ok().json(cached_data);
            }
        }
    }

    // Cache miss or expired - fetch fresh data
    log::debug!("Leaderboard cache miss, fetching fresh data");
    let db = data.db.clone();

    // Try to use optimized database view first
    let response_data = match db.get_leaderboard_stats().await {
        Ok(stats) => {
            match db.get_record_scores().await {
                Ok(record_rows) => {
                    let response = build_leaderboard_from_stats(stats, record_rows);
                    Some(response)
                }
                Err(e) => {
                    log::warn!(
                        "Error fetching record scores, falling back to legacy method: {}",
                        e
                    );
                    fallback_leaderboard_data(db.clone()).await
                }
            }
        }
        Err(e) => {
            log::warn!(
                "Error fetching optimized leaderboard stats, falling back to legacy method: {}",
                e
            );
            fallback_leaderboard_data(db.clone()).await
        }
    };

    match response_data {
        Some(response) => {
            // Update cache with the response
            if let Ok(json_value) = serde_json::to_value(&response) {
                let mut cache = data.leaderboard_cache.write().await;
                cache.data = Some(json_value);
                cache.cached_at = Utc::now();
                log::debug!("Leaderboard cache updated");
            }
            HttpResponse::Ok().json(response)
        }
        None => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to generate leaderboard"
            }))
        }
    }
}

/// Invalidate the leaderboard cache (call this after game updates)
pub async fn invalidate_leaderboard_cache(data: &web::Data<AppState>) {
    let mut cache = data.leaderboard_cache.write().await;
    cache.data = None;
    log::debug!("Leaderboard cache invalidated");
}

/// Fallback leaderboard data generation from raw game data
async fn fallback_leaderboard_data(
    db: std::sync::Arc<crate::db::Database>,
) -> Option<LeaderboardResponse> {
    match db.get_all_games_full().await {
        Ok(games) => Some(build_leaderboard_response(games)),
        Err(e) => {
            log::error!("Error generating leaderboard: {}", e);
            None
        }
    }
}

/// Build leaderboard from optimized database stats
fn build_leaderboard_from_stats(
    stats: Vec<crate::db::LeaderboardStatRow>,
    record_rows: Vec<crate::db::RecordScoreRow>,
) -> LeaderboardResponse {
    // Group stats by player
    let mut player_map: HashMap<String, Vec<crate::db::LeaderboardStatRow>> = HashMap::new();
    for stat in stats {
        player_map
            .entry(stat.player_name.clone())
            .or_insert_with(Vec::new)
            .push(stat);
    }

    let ordering = leaderboard_game_types();
    let mut players: Vec<PlayerLeaderboardEntry> = Vec::new();

    for (player_name, player_stats) in player_map {
        let total_wins: u32 = player_stats.iter().map(|s| s.wins).sum();

        let stats = ordering
            .iter()
            .map(|gt| {
                let gt_str = serde_json::to_string(gt).unwrap_or_default();
                let stat = player_stats.iter().find(|s| s.game_type == gt_str).cloned();

                match stat {
                    Some(s) => {
                        let total_pts = (s.average_score.unwrap_or(0.0) * s.games_played as f64) as i64;
                        PlayerGameStat {
                            game_type: gt.clone(),
                            wins: s.wins,
                            games_played: s.games_played,
                            // Leaderboard only uses finished games, so all games are finished
                            finished_games: s.games_played,
                            unfinished_games: 0,
                            total_rounds: 0, // Not tracked in leaderboard view
                            average_score: s.average_score.unwrap_or(0.0),
                            win_rate: if s.games_played > 0 {
                                s.wins as f64 / s.games_played as f64
                            } else {
                                0.0
                            },
                            highest_score: s.highest_score,
                            lowest_score: s.lowest_score,
                            total_points: total_pts,
                        }
                    }
                    None => PlayerGameStat {
                        game_type: gt.clone(),
                        wins: 0,
                        games_played: 0,
                        finished_games: 0,
                        unfinished_games: 0,
                        total_rounds: 0,
                        average_score: 0.0,
                        win_rate: 0.0,
                        highest_score: None,
                        lowest_score: None,
                        total_points: 0,
                    },
                }
            })
            .collect();

        players.push(PlayerLeaderboardEntry {
            player_name,
            total_wins,
            stats,
        });
    }

    players.sort_by(|a, b| {
        b.total_wins.cmp(&a.total_wins).then_with(|| {
            a.player_name
                .to_lowercase()
                .cmp(&b.player_name.to_lowercase())
        })
    });

    // Build records from record_rows
    let mut records = LeaderboardRecords::default();
    for row in record_rows {
        let game_type_result: Result<GameType, _> = serde_json::from_str(&row.game_type);
        if let Ok(game_type) = game_type_result {
            let entry = GameRecordEntry {
                game_type: game_type.clone(),
                player_name: row.player_name,
                score: row.score,
                game_id: row.game_id,
                occurred_at: row.occurred_at,
            };

            match (game_type, row.record_type.as_str()) {
                (GameType::King, "highest") => records.king_highest.push(entry),
                (GameType::King, "lowest") => records.king_lowest.push(entry),
                (GameType::DoubleKing, "highest") => records.double_king_highest.push(entry),
                (GameType::DoubleKing, "lowest") => records.double_king_lowest.push(entry),
                _ => {}
            }
        }
    }

    LeaderboardResponse {
        generated_at: Utc::now(),
        players,
        records,
    }
}

/// Build leaderboard from raw game data (legacy method)
/// Only includes finished games with non-zero scores in all statistics
fn build_leaderboard_response(games: Vec<Game>) -> LeaderboardResponse {
    let mut aggregates: HashMap<String, PlayerAggregate> = HashMap::new();
    let mut records = LeaderboardRecords::default();

    for game in games.iter() {
        // Only include finished games in leaderboard statistics
        if !game.game_over {
            continue;
        }
        
        // Skip empty games (no rounds or all scores are 0)
        if is_empty_game(game) {
            continue;
        }
        
        let game_type = game.game_type.clone();
        if game.players.is_empty() {
            continue;
        }

        for player in game.players.iter() {
            let entry = aggregates
                .entry(player.name.clone())
                .or_insert_with(PlayerAggregate::default);
            *entry.games_played.entry(game_type.clone()).or_insert(0) += 1;
            *entry.total_scores.entry(game_type.clone()).or_insert(0) += player.total_score as i64;

            entry
                .highest_scores
                .entry(game_type.clone())
                .and_modify(|best| {
                    if player.total_score > *best {
                        *best = player.total_score;
                    }
                })
                .or_insert(player.total_score);

            entry
                .lowest_scores
                .entry(game_type.clone())
                .and_modify(|worst| {
                    if player.total_score < *worst {
                        *worst = player.total_score;
                    }
                })
                .or_insert(player.total_score);

            update_records(&mut records, &game_type, player, game);
        }

        // Count wins (game is already confirmed as finished)
        if let Some(winner_indices) = determine_winner_indices(game) {
            for idx in winner_indices {
                if let Some(player) = game.players.get(idx) {
                    if let Some(entry) = aggregates.get_mut(&player.name) {
                        *entry.wins.entry(game_type.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let ordering = leaderboard_game_types();

    let mut players: Vec<PlayerLeaderboardEntry> = aggregates
        .into_iter()
        .map(|(name, aggregate)| {
            let stats = ordering
                .iter()
                .map(|gt| {
                    let wins = *aggregate.wins.get(gt).unwrap_or(&0);
                    let games_played = *aggregate.games_played.get(gt).unwrap_or(&0);
                    let total_score = *aggregate.total_scores.get(gt).unwrap_or(&0);
                    let highest_score = aggregate.highest_scores.get(gt).copied();
                    let lowest_score = aggregate.lowest_scores.get(gt).copied();
                    PlayerGameStat {
                        game_type: gt.clone(),
                        wins,
                        games_played,
                        // All games in build_leaderboard_from_games are finished (filtered earlier)
                        finished_games: games_played,
                        unfinished_games: 0,
                        total_rounds: 0, // Not tracked in this path
                        average_score: if games_played > 0 {
                            total_score as f64 / games_played as f64
                        } else {
                            0.0
                        },
                        win_rate: if games_played > 0 {
                            wins as f64 / games_played as f64
                        } else {
                            0.0
                        },
                        highest_score,
                        lowest_score,
                        total_points: total_score as i64,
                    }
                })
                .collect();

            let total_wins: u32 = aggregate.wins.values().sum();

            PlayerLeaderboardEntry {
                player_name: name,
                total_wins,
                stats,
            }
        })
        .collect();

    players.sort_by(|a, b| {
        b.total_wins.cmp(&a.total_wins).then_with(|| {
            a.player_name
                .to_lowercase()
                .cmp(&b.player_name.to_lowercase())
        })
    });

    LeaderboardResponse {
        generated_at: Utc::now(),
        players,
        records,
    }
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
