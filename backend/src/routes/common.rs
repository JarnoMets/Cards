//! Common types, helpers, and shared functionality for route handlers

use crate::models::{Game, GameType, Player};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// ELO Types
// =============================================================================

#[derive(Debug, Serialize)]
pub struct PlayerEloResponse {
    pub player_name: String,
    pub game_type: GameType,
    pub elo: i32,
    pub games_played: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct EloLeaderboardResponse {
    pub generated_at: DateTime<Utc>,
    pub ratings: Vec<PlayerEloResponse>,
}

#[derive(Debug, Serialize)]
pub struct OverallEloResponse {
    pub player_name: String,
    pub elo: i32,
    pub games_played: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OverallEloLeaderboardResponse {
    pub generated_at: DateTime<Utc>,
    pub ratings: Vec<OverallEloResponse>,
}

#[derive(Debug, Deserialize)]
pub struct EloQueryParams {
    pub game_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EloHistoryResponse {
    pub player_name: String,
    pub game_type: Option<String>,
    pub time_range: String,
    pub history: Vec<crate::db::EloHistoryEntry>,
}

#[derive(Debug, Deserialize)]
pub struct EloHistoryQueryParams {
    pub game_type: Option<String>,
    #[serde(default = "default_time_range")]
    pub time_range: String,
}

fn default_time_range() -> String {
    "all".to_string()
}

#[derive(Debug, Serialize)]
pub struct EloComparisonResponse {
    pub player_name: String,
    pub current_elo: i32,
    pub elo_7_days_ago: Option<i32>,
    pub elo_change: Option<i32>,
}

// =============================================================================
// Leaderboard Types
// =============================================================================

#[derive(Debug, Serialize)]
pub struct LeaderboardResponse {
    pub generated_at: DateTime<Utc>,
    pub players: Vec<PlayerLeaderboardEntry>,
    pub records: LeaderboardRecords,
}

#[derive(Debug, Serialize)]
pub struct PlayerLeaderboardEntry {
    pub player_name: String,
    pub total_wins: u32,
    pub stats: Vec<PlayerGameStat>,
}

#[derive(Debug, Serialize)]
pub struct PlayerGameStat {
    pub game_type: GameType,
    pub wins: u32,
    pub games_played: u32,
    pub finished_games: u32,
    pub unfinished_games: u32,
    pub total_rounds: u32,
    pub average_score: f64,
    pub win_rate: f64,
    pub highest_score: Option<i32>,
    pub lowest_score: Option<i32>,
    pub total_points: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct LeaderboardRecords {
    pub king_highest: Vec<GameRecordEntry>,
    pub king_lowest: Vec<GameRecordEntry>,
    pub double_king_highest: Vec<GameRecordEntry>,
    pub double_king_lowest: Vec<GameRecordEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameRecordEntry {
    pub game_type: GameType,
    pub player_name: String,
    pub score: i32,
    pub game_id: String,
    pub occurred_at: DateTime<Utc>,
}

// =============================================================================
// Player Types
// =============================================================================

#[derive(Debug, Serialize)]
pub struct PlayerDetailResponse {
    pub player_name: String,
    pub total_games: u32,
    pub finished_games: u32,
    pub unfinished_games: u32,
    pub total_wins: u32,
    #[serde(rename = "win_rate")]
    pub overall_win_rate: f64,
    pub total_points: i64,
    pub total_rounds: u32,
    pub first_played_at: Option<DateTime<Utc>>,
    pub last_played_at: Option<DateTime<Utc>>,
    #[serde(rename = "game_type_stats")]
    pub stats_by_game_type: Vec<PlayerGameStat>,
    pub score_timeline: Vec<PlayerScorePoint>,
    pub recent_games: Vec<PlayerGameSummary>,
}

#[derive(Debug, Serialize)]
pub struct PlayerScorePoint {
    pub game_id: String,
    pub game_type: GameType,
    pub occurred_at: DateTime<Utc>,
    pub total_score: i32,
    pub placement: usize,
    pub score_margin: i32,
    pub did_win: bool,
}

#[derive(Debug, Serialize)]
pub struct PlayerGameSummary {
    pub game_id: String,
    pub game_type: GameType,
    pub occurred_at: DateTime<Utc>,
    pub player_score: i32,
    pub placement: usize,
    pub total_players: usize,
    pub did_win: bool,
    pub score_margin: i32,
    pub players: Vec<GameParticipantSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elo_change: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GameParticipantSummary {
    pub name: String,
    pub total_score: i32,
    pub placement: usize,
    pub is_target: bool,
}

#[derive(Debug, Serialize)]
pub struct PlayerSearchResult {
    pub player_name: String,
    pub games_played: u32,
    pub last_played_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSearchParams {
    pub q: Option<String>,
    pub limit: Option<u32>,
}

// =============================================================================
// Admin Types
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct RenamePlayerRequest {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, Serialize)]
pub struct RenamePlayerResponse {
    pub message: String,
    pub games_updated: u64,
}

#[derive(Debug, Serialize)]
pub struct RecalculateEloResponse {
    pub message: String,
    pub games_processed: usize,
    pub players_updated: usize,
}

// =============================================================================
// Internal Aggregation Types
// =============================================================================

#[derive(Default)]
pub(crate) struct PlayerAggregate {
    pub wins: HashMap<GameType, u32>,
    pub games_played: HashMap<GameType, u32>,
    pub total_scores: HashMap<GameType, i64>,
    pub highest_scores: HashMap<GameType, i32>,
    pub lowest_scores: HashMap<GameType, i32>,
}

#[derive(Default)]
pub(crate) struct PlayerGameTypeAggregate {
    pub wins: u32,
    pub games_played: u32,
    pub finished_games: u32,
    pub unfinished_games: u32,
    pub total_rounds: u32,
    pub total_points: i64,
    pub highest_score: Option<i32>,
    pub lowest_score: Option<i32>,
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Ordered list of game types for leaderboard display
pub fn leaderboard_game_types() -> Vec<GameType> {
    vec![
        GameType::King,
        GameType::DoubleKing,
        GameType::Hearts,
        GameType::ColorWhist,
        GameType::Whist,
        GameType::Manille,
    ]
}

/// Normalize player name for comparison (lowercase, trimmed)
pub fn normalize_player_name(name: &str) -> String {
    name.trim().to_lowercase()
}

/// Determine winner indices for a game
pub fn determine_winner_indices(game: &Game) -> Option<Vec<usize>> {
    if !game.game_over || game.players.is_empty() {
        return None;
    }

    let is_lower_better = matches!(game.game_type, GameType::Hearts);

    let extreme = game.players.iter().map(|p| p.total_score);
    let extreme = if is_lower_better {
        extreme.min()
    } else {
        extreme.max()
    }?;

    let indices: Vec<usize> = game
        .players
        .iter()
        .enumerate()
        .filter(|(_, p)| p.total_score == extreme)
        .map(|(i, _)| i)
        .collect();

    if indices.is_empty() {
        None
    } else {
        Some(indices)
    }
}

/// Compute placements (1st, 2nd, etc.) for all players
pub fn compute_placements(game: &Game) -> Vec<usize> {
    if game.players.is_empty() {
        return Vec::new();
    }

    let is_lower_better = matches!(game.game_type, GameType::Hearts);
    let mut sorted_indices: Vec<usize> = (0..game.players.len()).collect();

    sorted_indices.sort_by(|a, b| {
        let score_a = game.players[*a].total_score;
        let score_b = game.players[*b].total_score;
        if is_lower_better {
            score_a.cmp(&score_b)
        } else {
            score_b.cmp(&score_a)
        }
    });

    let mut placements = vec![0; game.players.len()];
    let mut current_rank = 1usize;
    let mut last_score: Option<i32> = None;

    for (position, idx) in sorted_indices.iter().enumerate() {
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

/// Compute score margin (difference from next player)
pub fn compute_score_margin(game: &Game, player_score: i32) -> i32 {
    let is_lower_better = matches!(game.game_type, GameType::Hearts);

    let mut sorted_scores: Vec<i32> = game.players.iter().map(|p| p.total_score).collect();
    if is_lower_better {
        sorted_scores.sort_unstable();
    } else {
        sorted_scores.sort_unstable_by(|a, b| b.cmp(a));
    }

    let pos = sorted_scores.iter().position(|&s| s == player_score);
    if let Some(idx) = pos {
        if idx + 1 < sorted_scores.len() {
            (player_score - sorted_scores[idx + 1]).abs()
        } else if idx > 0 {
            (player_score - sorted_scores[idx - 1]).abs()
        } else {
            0
        }
    } else {
        0
    }
}

/// Update record entries for King/DoubleKing games
pub fn update_records(
    records: &mut LeaderboardRecords,
    game_type: &GameType,
    player: &Player,
    game: &Game,
) {
    match game_type {
        GameType::King => {
            update_record_list(
                &mut records.king_highest,
                game_type,
                player,
                game,
                false, // higher is better
            );
            update_record_list(&mut records.king_lowest, game_type, player, game, true);
        }
        GameType::DoubleKing => {
            update_record_list(
                &mut records.double_king_highest,
                game_type,
                player,
                game,
                false,
            );
            update_record_list(&mut records.double_king_lowest, game_type, player, game, true);
        }
        _ => {}
    }
}

fn update_record_list(
    list: &mut Vec<GameRecordEntry>,
    game_type: &GameType,
    player: &Player,
    game: &Game,
    lower_is_better: bool,
) {
    const MAX_RECORDS: usize = 10;

    // Check if this entry already exists (same player, same score, same game)
    let already_present = list
        .iter()
        .any(|entry| entry.player_name == player.name && entry.game_id == game.id);

    if already_present {
        return;
    }

    // Add the new entry
    list.push(GameRecordEntry {
        game_type: game_type.clone(),
        player_name: player.name.clone(),
        score: player.total_score,
        game_id: game.id.clone(),
        occurred_at: game.created_at,
    });

    // Sort the list (best scores first)
    if lower_is_better {
        list.sort_by(|a, b| a.score.cmp(&b.score));
    } else {
        list.sort_by(|a, b| b.score.cmp(&a.score));
    }

    // Keep only top N records
    list.truncate(MAX_RECORDS);
}

/// Health check endpoint
pub async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "card-game-scorer"
    }))
}
