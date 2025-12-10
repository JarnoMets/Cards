//! Background tasks that run periodically
//! 
//! This module contains scheduled tasks that run in the background:
//! - Cleanup: Delete empty games older than 24 hours
//! - ELO recalculation: Refresh all ELO ratings hourly
//! - ELO history backfill: Calculate ELO history for historic games

use crate::db::Database;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

/// Configuration for background tasks
pub struct TaskConfig {
    /// Interval between cleanup runs (default: 1 hour)
    pub cleanup_interval: Duration,
    /// Minimum age of empty games before deletion (default: 24 hours)
    pub empty_game_age_hours: i64,
    /// Whether to run ELO recalculation
    pub enable_elo_recalculation: bool,
    /// Whether to backfill ELO history on startup
    pub enable_elo_backfill: bool,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            cleanup_interval: Duration::from_secs(3600), // 1 hour
            empty_game_age_hours: 24,
            enable_elo_recalculation: true,
            enable_elo_backfill: true,
        }
    }
}

/// Start all background tasks
pub fn start_background_tasks(db: Arc<Database>, config: TaskConfig) {
    // Run ELO backfill once on startup
    if config.enable_elo_backfill {
        let db_clone = db.clone();
        tokio::spawn(async move {
            // Wait a bit for the server to start up
            tokio::time::sleep(Duration::from_secs(10)).await;
            run_elo_history_backfill(&db_clone).await;
        });
    }

    // Start the periodic cleanup task
    let db_clone = db.clone();
    let empty_game_age_hours = config.empty_game_age_hours;
    let enable_elo_recalculation = config.enable_elo_recalculation;
    let cleanup_interval = config.cleanup_interval;
    
    tokio::spawn(async move {
        let mut ticker = interval(cleanup_interval);
        
        loop {
            ticker.tick().await;
            
            log::info!("Running periodic background tasks...");
            
            // Task 1: Clean up empty games
            match cleanup_empty_games(&db_clone, empty_game_age_hours).await {
                Ok(count) => {
                    if count > 0 {
                        log::info!("Cleaned up {} empty games", count);
                    }
                }
                Err(e) => {
                    log::error!("Failed to cleanup empty games: {}", e);
                }
            }
            
            // Task 2: Recalculate ELO ratings
            if enable_elo_recalculation {
                match db_clone.recalculate_all_elo().await {
                    Ok((games, players)) => {
                        log::info!(
                            "ELO recalculation complete: {} games processed, {} players updated",
                            games,
                            players
                        );
                    }
                    Err(e) => {
                        log::error!("Failed to recalculate ELO: {}", e);
                    }
                }
            }
            
            log::info!("Periodic background tasks completed");
        }
    });
}

/// Delete games where all player scores are 0 and created more than `hours_old` hours ago
async fn cleanup_empty_games(db: &Database, hours_old: i64) -> Result<usize, String> {
    db.delete_empty_games(hours_old)
        .await
        .map_err(|e| e.to_string())
}

/// Backfill ELO history for finished games that don't have history entries
async fn run_elo_history_backfill(db: &Database) {
    log::info!("Starting ELO history backfill for historic games...");
    
    match db.backfill_elo_history().await {
        Ok(count) => {
            if count > 0 {
                log::info!("Backfilled ELO history for {} games", count);
            } else {
                log::info!("No games needed ELO history backfill");
            }
        }
        Err(e) => {
            log::error!("Failed to backfill ELO history: {}", e);
        }
    }
}
