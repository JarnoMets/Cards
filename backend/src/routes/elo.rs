//! ELO rating endpoints

use crate::db::EloConfig;
use crate::models::{AppState, GameType};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;

use super::common::{
    EloComparisonResponse, EloHistoryQueryParams, EloHistoryResponse, EloLeaderboardResponse,
    EloQueryParams, OverallEloLeaderboardResponse, OverallEloResponse, PlayerEloResponse,
};

/// Cache TTL in seconds (30 seconds default)
const CACHE_TTL_SECONDS: i64 = 30;

/// Get ELO ratings for a specific player
pub async fn get_player_elo(
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
    match db.get_player_all_elo(&player_name).await {
        Ok(ratings) => {
            let response: Vec<PlayerEloResponse> = ratings
                .into_iter()
                .filter_map(|row| {
                    let game_type: Result<GameType, _> = serde_json::from_str(&row.game_type);
                    game_type.ok().map(|gt| PlayerEloResponse {
                        player_name: row.player_name,
                        game_type: gt,
                        elo: row.elo,
                        games_played: row.games_played,
                        updated_at: row.updated_at,
                    })
                })
                .collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Error fetching ELO for player '{}': {}", player_name, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch ELO ratings"
            }))
        }
    }
}

/// Get ELO leaderboard, optionally filtered by game type
pub async fn get_elo_leaderboard(
    data: web::Data<AppState>,
    params: web::Query<EloQueryParams>,
) -> impl Responder {
    let db = data.db.clone();

    let game_type: Option<GameType> = params.game_type.as_ref().and_then(|gt_str| {
        // Try parsing as JSON string (e.g., "\"hearts\"")
        serde_json::from_str(&format!("\"{}\"", gt_str)).ok()
    });

    match db.get_elo_leaderboard(game_type.as_ref()).await {
        Ok(ratings) => {
            let response: Vec<PlayerEloResponse> = ratings
                .into_iter()
                .filter_map(|row| {
                    let game_type: Result<GameType, _> = serde_json::from_str(&row.game_type);
                    game_type.ok().map(|gt| PlayerEloResponse {
                        player_name: row.player_name,
                        game_type: gt,
                        elo: row.elo,
                        games_played: row.games_played,
                        updated_at: row.updated_at,
                    })
                })
                .collect();
            HttpResponse::Ok().json(EloLeaderboardResponse {
                generated_at: Utc::now(),
                ratings: response,
            })
        }
        Err(e) => {
            log::error!("Error fetching ELO leaderboard: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch ELO leaderboard"
            }))
        }
    }
}

/// Get overall ELO leaderboard (average across all game types) with caching
pub async fn get_overall_elo_leaderboard(data: web::Data<AppState>) -> impl Responder {
    // First, check if we have valid cached data
    {
        let cache = data.elo_leaderboard_cache.read().await;
        if cache.is_valid(CACHE_TTL_SECONDS) {
            if let Some(ref cached_data) = cache.data {
                log::debug!("Serving overall ELO leaderboard from cache");
                return HttpResponse::Ok().json(cached_data);
            }
        }
    }

    // Cache miss or expired - fetch fresh data
    log::debug!("Overall ELO leaderboard cache miss, fetching fresh data");
    let db = data.db.clone();

    match db.get_overall_elo_leaderboard().await {
        Ok(ratings) => {
            let response_data: Vec<OverallEloResponse> = ratings
                .into_iter()
                .map(|row| OverallEloResponse {
                    player_name: row.player_name,
                    elo: row.elo,
                    games_played: row.games_played,
                    updated_at: row.updated_at,
                })
                .collect();
            
            let response = OverallEloLeaderboardResponse {
                generated_at: Utc::now(),
                ratings: response_data,
            };

            // Update cache with the response
            if let Ok(json_value) = serde_json::to_value(&response) {
                let mut cache = data.elo_leaderboard_cache.write().await;
                cache.data = Some(json_value);
                cache.cached_at = Utc::now();
                log::debug!("Overall ELO leaderboard cache updated");
            }

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Error fetching overall ELO leaderboard: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch overall ELO leaderboard"
            }))
        }
    }
}

/// Invalidate the ELO leaderboard cache
pub async fn invalidate_elo_cache(data: &web::Data<AppState>) {
    let mut cache = data.elo_leaderboard_cache.write().await;
    cache.data = None;
    log::debug!("ELO leaderboard cache invalidated");
}

/// Get ELO history for a player
pub async fn get_player_elo_history(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<EloHistoryQueryParams>,
) -> impl Responder {
    let player_name = path.into_inner();
    let db = data.db.clone();

    let game_type: Option<GameType> = query
        .game_type
        .as_ref()
        .and_then(|gt| serde_json::from_str(&format!("\"{}\"", gt)).ok());

    match db
        .get_player_elo_history(&player_name, game_type.as_ref(), &query.time_range)
        .await
    {
        Ok(history) => HttpResponse::Ok().json(EloHistoryResponse {
            player_name,
            game_type: query.game_type.clone(),
            time_range: query.time_range.clone(),
            history,
        }),
        Err(e) => {
            log::error!("Error fetching ELO history: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch ELO history"
            }))
        }
    }
}

/// Get ELO comparison (current vs 7 days ago) for leaderboard
pub async fn get_player_elo_comparison(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<EloQueryParams>,
) -> impl Responder {
    let player_name = path.into_inner();
    let db = data.db.clone();

    let game_type: Option<GameType> = query
        .game_type
        .as_ref()
        .and_then(|gt| serde_json::from_str(&format!("\"{}\"", gt)).ok());

    // Get current ELO
    let current_elo = if let Some(ref gt) = game_type {
        db.get_player_elo(&player_name, gt).await.unwrap_or(1500)
    } else {
        db.get_player_overall_elo(&player_name)
            .await
            .ok()
            .flatten()
            .map(|(elo, _)| elo)
            .unwrap_or(1500)
    };

    // Get ELO from 7 days ago
    let elo_7_days_ago = db
        .get_player_elo_days_ago(&player_name, game_type.as_ref(), 7)
        .await
        .ok()
        .flatten();

    let elo_change = elo_7_days_ago.map(|old| current_elo - old);

    HttpResponse::Ok().json(EloComparisonResponse {
        player_name,
        current_elo,
        elo_7_days_ago,
        elo_change,
    })
}

/// Get ELO configuration
pub async fn get_elo_config(data: web::Data<AppState>) -> impl Responder {
    let db = data.db.clone();
    let config = db.get_elo_config().await;
    HttpResponse::Ok().json(config)
}

/// Update ELO configuration
pub async fn set_elo_config(
    data: web::Data<AppState>,
    config: web::Json<EloConfig>,
) -> impl Responder {
    let db = data.db.clone();

    match db.set_elo_config(&config).await {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "message": "ELO configuration updated successfully"
        })),
        Err(e) => {
            log::error!("Error updating ELO config: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update ELO configuration"
            }))
        }
    }
}
