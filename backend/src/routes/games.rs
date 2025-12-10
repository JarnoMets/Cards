//! Game CRUD endpoints

use crate::db::{Database, GameEloChange};
use crate::models::{AddRoundRequest, AppState, CreateGameRequest, Game, GameType, UpdatePlayerRequest};
use crate::notifications::EmailNotifier;
use crate::services::{
    ColorWhistService, DoubleKingService, GameService, HeartsService, KingService, ManilleService,
    PressService, WhistService,
};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

use super::elo::invalidate_elo_cache;
use super::leaderboard::invalidate_leaderboard_cache;

/// Get all games (limited to 100)
pub async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let db = data.db.clone();
    match db.get_all_games().await {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(e) => {
            log::error!("Error fetching games: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch games"
            }))
        }
    }
}

/// Create a new game
pub async fn create_game(
    data: web::Data<AppState>,
    req: web::Json<CreateGameRequest>,
) -> impl Responder {
    if req.players.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "At least one player is required"
        }));
    }

    let db = data.db.clone();
    match db
        .create_game(req.game_type.clone(), req.players.clone())
        .await
    {
        Ok(game) => {
            log::info!(
                "Game created: {} with {} players",
                game.id,
                req.players.len()
            );
            HttpResponse::Created().json(game)
        }
        Err(e) => {
            log::error!("Error creating game: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create game"
            }))
        }
    }
}

/// Get a specific game by ID
pub async fn get_game(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let game_id = path.into_inner();
    let db = data.db.clone();

    log::debug!("Looking for game: {}", game_id);

    match db.get_game(&game_id).await {
        Ok(game) => {
            log::info!("Game found: {}", game_id);
            HttpResponse::Ok().json(game)
        }
        Err(e) => {
            log::warn!("Game not found: {} - {}", game_id, e);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Game not found"
            }))
        }
    }
}

/// Delete a game
pub async fn delete_game(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let game_id = path.into_inner();
    let db = data.db.clone();

    match db.delete_game(&game_id).await {
        Ok(_) => {
            log::info!("Game deleted: {}", game_id);
            // Invalidate caches since stats have changed
            invalidate_leaderboard_cache(&data).await;
            invalidate_elo_cache(&data).await;
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Game deleted successfully"
            }))
        }
        Err(e) => {
            log::warn!("Error deleting game: {} - {}", game_id, e);
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Game not found"
            }))
        }
    }
}

/// Add a round to a game
pub async fn add_round(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<AddRoundRequest>,
) -> impl Responder {
    let game_id = path.into_inner();
    let db = data.db.clone();
    let request = req.into_inner();

    // First, get the game to determine its type and check if it was already finished
    let game = match db.get_game(&game_id).await {
        Ok(game) => game,
        Err(_) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Game not found"
            }))
        }
    };

    let was_finished = game.game_over;

    // Select the appropriate service based on game type
    let game_service: Box<dyn GameService> = get_game_service(&game.game_type);

    match db.add_round(&game_id, request, game_service.as_ref()).await {
        Ok(game) => {
            log::info!("Round added to game: {}", game_id);

            // Check if game just finished (wasn't finished before, but is now)
            if !was_finished && game.game_over {
                log::info!("Game {} just finished, updating ELO and sending notification", game_id);
                
                // Update ELO ratings for all players and get the changes
                let db_for_elo = data.db.clone();
                let elo_changes = match db_for_elo.update_elo_for_game(&game).await {
                    Ok(changes) => Some(changes),
                    Err(e) => {
                        log::error!("Failed to update ELO ratings for game {}: {}", game_id, e);
                        None
                    }
                };
                
                // Send notification to admin with ELO changes
                let notifier = data.notifier.clone();
                let game_clone = game.clone();
                let elo_changes_clone = elo_changes.clone();
                tokio::spawn(async move {
                    notifier.notify_game_finished(&game_clone, elo_changes_clone.as_deref()).await;
                });

                // Send notifications to players who opted in
                let player_names: Vec<String> = game.players.iter().map(|p| p.name.clone()).collect();
                let db_for_settings = data.db.clone();
                let notifier_for_players = data.notifier.clone();
                let game_for_players = game.clone();
                tokio::spawn(async move {
                    send_player_notifications(
                        db_for_settings,
                        notifier_for_players,
                        &game_for_players,
                        &player_names,
                        elo_changes.as_deref(),
                    ).await;
                });

                // Invalidate caches so stats refresh on next request
                invalidate_leaderboard_cache(&data).await;
                invalidate_elo_cache(&data).await;
                log::debug!("Caches invalidated after game completion");
            }

            HttpResponse::Ok().json(game)
        }
        Err(e) => {
            log::error!("Error adding round to game {}: {}", game_id, e);
            // Try to parse the error string as JSON produced by services (structured error)
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&e.to_string()) {
                return HttpResponse::BadRequest().json(val);
            }
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

/// Update a specific round in a game
pub async fn update_round(
    data: web::Data<AppState>,
    path: web::Path<(String, usize)>,
    req: web::Json<AddRoundRequest>,
) -> impl Responder {
    let (game_id, round_index) = path.into_inner();
    let db = data.db.clone();

    let game = match db.get_game(&game_id).await {
        Ok(game) => game,
        Err(_) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Game not found"
            }))
        }
    };

    let game_service: Box<dyn GameService> = get_game_service(&game.game_type);

    match db
        .update_round(game, round_index, req.into_inner(), game_service.as_ref())
        .await
    {
        Ok(updated_game) => {
            log::info!("Round {} updated in game: {}", round_index, game_id);
            HttpResponse::Ok().json(updated_game)
        }
        Err(e) => {
            log::error!(
                "Error updating round {} in game {}: {}",
                round_index,
                game_id,
                e
            );
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&e.to_string()) {
                return HttpResponse::BadRequest().json(val);
            }
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

/// Update a player's name in a specific game
pub async fn update_player(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    req: web::Json<UpdatePlayerRequest>,
) -> impl Responder {
    let (game_id, player_id) = path.into_inner();
    let db = data.db.clone();

    match db
        .update_player_name(&game_id, &player_id, &req.name)
        .await
    {
        Ok(game) => {
            log::info!("Player {} name updated in game: {}", player_id, game_id);
            HttpResponse::Ok().json(game)
        }
        Err(e) => {
            log::error!(
                "Error updating player {} in game {}: {}",
                player_id,
                game_id,
                e
            );
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

/// Get ELO changes for a game
pub async fn get_game_elo_changes(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let game_id = path.into_inner();
    let db = data.db.clone();

    match db.get_game_elo_changes(&game_id).await {
        Ok(changes) => HttpResponse::Ok().json(changes),
        Err(e) => {
            log::error!("Error fetching ELO changes for game {}: {}", game_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch ELO changes"
            }))
        }
    }
}

/// Get the appropriate game service for a game type
fn get_game_service(game_type: &GameType) -> Box<dyn GameService> {
    match game_type {
        GameType::Hearts => Box::new(HeartsService::new()),
        GameType::King => Box::new(KingService::new()),
        GameType::DoubleKing => Box::new(DoubleKingService::new()),
        GameType::ColorWhist => Box::new(ColorWhistService::new()),
        GameType::Whist => Box::new(WhistService::new()),
        GameType::Manille => Box::new(ManilleService::new()),
        GameType::Press => Box::new(PressService::new()),
    }
}

/// Send email notifications to players who opted in
async fn send_player_notifications(
    db: Arc<Database>,
    notifier: EmailNotifier,
    game: &Game,
    player_names: &[String],
    elo_changes: Option<&[GameEloChange]>,
) {
    // Get daily email limit from env
    let daily_limit = std::env::var("EMAIL_DAILY_LIMIT")
        .ok()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(200);

    // Get players who have notifications enabled
    let recipients = match db.get_notification_recipients(player_names).await {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to get notification recipients: {}", e);
            return;
        }
    };

    if recipients.is_empty() {
        log::debug!("No players with notifications enabled for game {}", game.id);
        return;
    }

    for recipient in recipients {
        if let Some(ref email) = recipient.email {
            // Check and increment daily counter
            match db.check_and_increment_email_count(daily_limit).await {
                Ok((count, can_send)) => {
                    if !can_send {
                        log::warn!(
                            "Daily email limit ({}) reached, skipping email to {} for game {}",
                            daily_limit, email, game.id
                        );
                        continue;
                    }
                    log::debug!("Daily email count: {}/{}", count, daily_limit);
                }
                Err(e) => {
                    log::error!("Failed to check email count: {}", e);
                    continue;
                }
            }

            // Send the email
            if let Err(e) = notifier.send_player_game_email(game, email, elo_changes).await {
                log::error!(
                    "Failed to send game email to {} for game {}: {}",
                    email, game.id, e
                );
            }
        }
    }
}
