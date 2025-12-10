//! Admin operations (recalculate ELO, email, cleanup, etc.)

use crate::models::AppState;
use actix_web::{web, HttpResponse, Responder};

use super::common::RecalculateEloResponse;

/// Send a test email
pub async fn send_test_email(data: web::Data<AppState>) -> impl Responder {
    // Get the latest game to use as a preview
    let db = data.db.clone();
    let latest_game = match db.get_all_games().await {
        Ok(games) => games.into_iter().next(),
        Err(_) => None,
    };

    match data.notifier.send_test_email(latest_game.as_ref()).await {
        Ok(_) => {
            log::info!("Test email sent successfully");
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Test email sent successfully"
            }))
        }
        Err(e) => {
            log::error!("Failed to send test email: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Failed to send test email: {}", e)
            }))
        }
    }
}

/// Get email notification status
pub async fn get_email_status(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "enabled": data.notifier.is_enabled()
    }))
}

/// Recalculate all ELO ratings from game history
pub async fn recalculate_elo(data: web::Data<AppState>) -> impl Responder {
    let db = data.db.clone();

    log::info!("Starting ELO recalculation from game history...");

    match db.recalculate_all_elo().await {
        Ok((games, players)) => {
            log::info!(
                "ELO recalculation complete: {} games processed, {} players updated",
                games,
                players
            );
            HttpResponse::Ok().json(RecalculateEloResponse {
                message: format!(
                    "ELO recalculated successfully from {} games for {} players",
                    games, players
                ),
                games_processed: games,
                players_updated: players,
            })
        }
        Err(e) => {
            log::error!("ELO recalculation failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to recalculate ELO: {}", e)
            }))
        }
    }
}

/// Manually trigger cleanup of empty games
/// Deletes games where all scores are 0 and older than 24 hours
pub async fn cleanup_empty_games(data: web::Data<AppState>) -> impl Responder {
    let db = data.db.clone();

    log::info!("Starting manual cleanup of empty games...");

    match db.delete_empty_games(24).await {
        Ok(count) => {
            log::info!("Cleanup complete: {} empty games deleted", count);
            HttpResponse::Ok().json(serde_json::json!({
                "message": format!("Cleaned up {} empty games", count),
                "games_deleted": count
            }))
        }
        Err(e) => {
            log::error!("Cleanup failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to cleanup empty games: {}", e)
            }))
        }
    }
}

/// Manually trigger ELO history backfill for historic games
pub async fn backfill_elo_history(data: web::Data<AppState>) -> impl Responder {
    let db = data.db.clone();

    log::info!("Starting ELO history backfill...");

    match db.backfill_elo_history().await {
        Ok(count) => {
            log::info!("ELO history backfill complete: {} games backfilled", count);
            HttpResponse::Ok().json(serde_json::json!({
                "message": format!("Backfilled ELO history for {} games", count),
                "games_backfilled": count
            }))
        }
        Err(e) => {
            log::error!("ELO history backfill failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to backfill ELO history: {}", e)
            }))
        }
    }
}

/// Get email daily statistics
pub async fn get_email_daily_stats(data: web::Data<AppState>) -> impl Responder {
    let db = data.db.clone();
    
    let daily_limit = std::env::var("EMAIL_DAILY_LIMIT")
        .ok()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(200);
    
    match db.get_email_daily_stats().await {
        Ok(stats) => {
            let remaining = (daily_limit - stats.emails_sent).max(0);
            HttpResponse::Ok().json(serde_json::json!({
                "date": stats.date,
                "emails_sent": stats.emails_sent,
                "daily_limit": daily_limit,
                "remaining": remaining
            }))
        }
        Err(e) => {
            log::error!("Failed to get email daily stats: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get email statistics"
            }))
        }
    }
}
