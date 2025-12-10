use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::RwLock;

mod db;
mod models;
mod notifications;
mod routes;
mod services;
mod tasks;

use db::Database;
use models::{AppState, LeaderboardCache};
use notifications::EmailNotifier;
use tasks::{start_background_tasks, TaskConfig};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgresuser:postgrespwd@localhost:5432/postgresdb".to_string()
    });

    // Initialize database connection
    let db = match Database::new(&database_url).await {
        Ok(db) => {
            log::info!("Connected to PostgreSQL database");
            db
        }
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            panic!("Database connection failed: {}", e);
        }
    };

    // Initialize email notifier
    let notifier = EmailNotifier::from_env();
    log::info!("Email notifications enabled: {}", std::env::var("EMAIL_ENABLED").unwrap_or_default() == "true");

    // Initialize in-memory caches
    let leaderboard_cache = Arc::new(RwLock::new(LeaderboardCache::new()));
    let elo_leaderboard_cache = Arc::new(RwLock::new(LeaderboardCache::new()));
    log::info!("In-memory caches initialized");

    let app_state = web::Data::new(AppState {
        db: Arc::new(db),
        notifier,
        leaderboard_cache,
        elo_leaderboard_cache,
    });

    // Start background tasks (cleanup, ELO recalculation)
    let db_for_tasks = app_state.db.clone();
    let task_config = TaskConfig::default();
    start_background_tasks(db_for_tasks, task_config);

    log::info!("Starting Card Game Scorer API on http://0.0.0.0:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:34923")
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(routes::health_check))
                    .route("/games", web::get().to(routes::get_games))
                    .route("/games", web::post().to(routes::create_game))
                    .route("/games/{id}", web::get().to(routes::get_game))
                    .route("/games/{id}", web::delete().to(routes::delete_game))
                    .route("/games/{id}/elo-changes", web::get().to(routes::get_game_elo_changes))
                    .route("/games/{id}/round", web::post().to(routes::add_round))
                    .route(
                        "/games/{id}/round/{round_index}",
                        web::patch().to(routes::update_round),
                    )
                    .route(
                        "/games/{id}/players/{player_id}",
                        web::patch().to(routes::update_player),
                    )
                    .route("/leaderboard", web::get().to(routes::get_leaderboard))
                    .route("/players/search", web::get().to(routes::search_players))
                    .route(
                        "/players/{player_name}",
                        web::get().to(routes::get_player_profile),
                    )
                    .route(
                        "/players/{player_name}/games",
                        web::get().to(routes::get_player_games),
                    )
                    .route(
                        "/players/{player_name}/elo",
                        web::get().to(routes::get_player_elo),
                    )
                    .route(
                        "/players/{player_name}/elo/history",
                        web::get().to(routes::get_player_elo_history),
                    )
                    .route(
                        "/players/{player_name}/elo/comparison",
                        web::get().to(routes::get_player_elo_comparison),
                    )
                    .route(
                        "/players/{player_name}/settings",
                        web::get().to(routes::get_player_settings),
                    )
                    .route(
                        "/players/{player_name}/settings",
                        web::put().to(routes::update_player_settings),
                    )
                    .route("/elo", web::get().to(routes::get_elo_leaderboard))
                    .route("/elo/overall", web::get().to(routes::get_overall_elo_leaderboard))
                    // Admin routes
                    .route("/admin/rename-player", web::post().to(routes::rename_player_globally))
                    .route("/admin/test-email", web::post().to(routes::send_test_email))
                    .route("/admin/email-status", web::get().to(routes::get_email_status))
                    .route("/admin/email-daily-stats", web::get().to(routes::get_email_daily_stats))
                    .route("/admin/recalculate-elo", web::post().to(routes::recalculate_elo))
                    .route("/admin/cleanup-empty-games", web::post().to(routes::cleanup_empty_games))
                    .route("/admin/backfill-elo-history", web::post().to(routes::backfill_elo_history))
                    .route("/admin/elo-config", web::get().to(routes::get_elo_config))
                    .route("/admin/elo-config", web::put().to(routes::set_elo_config)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
