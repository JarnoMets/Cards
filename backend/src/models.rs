use crate::db::Database;
use crate::notifications::EmailNotifier;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub scores: Vec<i32>,
    pub game_indices: Vec<usize>,
    pub total_score: i32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            scores: Vec::new(),
            game_indices: Vec::new(),
            total_score: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    Hearts,
    King,
    #[serde(rename = "double_king")]
    DoubleKing,
    #[serde(rename = "color_whist")]
    ColorWhist,
    Whist,
    Manille,
    Press,
    // Future games can be added here
    // Spades,
    // Bridge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub game_type: GameType,
    pub players: Vec<Player>,
    pub created_at: DateTime<Utc>,
    pub current_round: usize,
    pub game_over: bool,
}

impl Game {
    #[allow(dead_code)]
    pub fn new(game_type: GameType, player_names: Vec<String>) -> Self {
        let players = player_names
            .into_iter()
            .map(|name| Player::new(name))
            .collect();

        Self {
            id: Uuid::new_v4().to_string(),
            game_type,
            players,
            created_at: Utc::now(),
            current_round: 0,
            game_over: false,
        }
    }
}

/// In-memory cache for leaderboard data
/// Reduces database load for frequently accessed data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardCache {
    pub data: Option<serde_json::Value>,
    pub cached_at: DateTime<Utc>,
}

impl LeaderboardCache {
    pub fn new() -> Self {
        LeaderboardCache {
            data: None,
            cached_at: Utc::now(),
        }
    }

    /// Check if cache is still valid (default 30 seconds TTL)
    pub fn is_valid(&self, ttl_seconds: i64) -> bool {
        if self.data.is_none() {
            return false;
        }
        let elapsed = Utc::now().signed_duration_since(self.cached_at);
        elapsed.num_seconds() < ttl_seconds
    }
}

impl Default for LeaderboardCache {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AppState {
    pub db: Arc<Database>,
    pub notifier: EmailNotifier,
    pub leaderboard_cache: Arc<RwLock<LeaderboardCache>>,
    pub elo_leaderboard_cache: Arc<RwLock<LeaderboardCache>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    #[serde(alias = "gameType")]
    pub game_type: GameType,

    #[serde(alias = "playerNames")]
    pub players: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddRoundRequest {
    #[serde(default)]
    pub scores: Option<Vec<i32>>,
    #[serde(default, alias = "gameIndex")]
    pub game_index: Option<usize>, // For Double King games, indicates which game was played
    #[serde(default, alias = "colorWhistRound")]
    pub color_whist_round: Option<ColorWhistRoundRequest>,
    #[serde(default, alias = "whistRound")]
    pub whist_round: Option<WhistRoundRequest>,
    #[serde(default, alias = "manilleRound")]
    pub manille_round: Option<ManilleRoundRequest>,
    #[serde(default, alias = "pressRound")]
    pub press_round: Option<PressRoundRequest>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlayerRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ColorWhistRoundRequest {
    pub selections: Vec<ColorWhistSelection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorWhistSelection {
    pub player_index: usize,
    pub contract: ColorWhistContract,
    pub tricks: Option<u8>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ColorWhistContract {
    None,
    #[serde(rename = "accept_8")]
    Accept8,
    #[serde(rename = "accept_9")]
    Accept9,
    #[serde(rename = "accept_10")]
    Accept10,
    #[serde(rename = "accept_11")]
    Accept11,
    #[serde(rename = "accept_12")]
    Accept12,
    #[serde(rename = "accept_13")]
    Accept13,
    #[serde(rename = "solo_5")]
    Solo5,
    #[serde(rename = "solo_6")]
    Solo6,
    #[serde(rename = "solo_7")]
    Solo7,
    #[serde(rename = "solo_8")]
    Solo8,
    #[serde(rename = "abondance_9")]
    Abondance9,
    #[serde(rename = "abondance_10")]
    Abondance10,
    #[serde(rename = "abondance_11")]
    Abondance11,
    #[serde(rename = "abondance_12")]
    Abondance12,
    Trull,
    Piccolo,
    #[serde(rename = "small_misery")]
    SmallMisery,
    #[serde(rename = "large_misery")]
    LargeMisery,
    #[serde(rename = "open_misery")]
    OpenMisery,
    #[serde(rename = "grand_slam")]
    GrandSlam,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WhistRoundRequest {
    pub selections: Vec<WhistSelection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhistSelection {
    pub player_index: usize,
    pub contract: WhistContract,
    pub tricks: Option<u8>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WhistContract {
    None,
    #[serde(rename = "accept_8")]
    Accept8,
    #[serde(rename = "accept_9")]
    Accept9,
    #[serde(rename = "accept_10")]
    Accept10,
    #[serde(rename = "accept_11")]
    Accept11,
    #[serde(rename = "accept_12")]
    Accept12,
    #[serde(rename = "accept_13")]
    Accept13,
    #[serde(rename = "solo_5")]
    Solo5,
    #[serde(rename = "solo_6")]
    Solo6,
    #[serde(rename = "solo_7")]
    Solo7,
    #[serde(rename = "solo_8")]
    Solo8,
    #[serde(rename = "abondance_9")]
    Abondance9,
    #[serde(rename = "abondance_10")]
    Abondance10,
    #[serde(rename = "abondance_11")]
    Abondance11,
    #[serde(rename = "abondance_12")]
    Abondance12,
    Trull,
    Piccolo,
    #[serde(rename = "small_misery")]
    SmallMisery,
    #[serde(rename = "large_misery")]
    LargeMisery,
    #[serde(rename = "open_misery")]
    OpenMisery,
    #[serde(rename = "grand_slam")]
    GrandSlam,
    /// Slam: Player chooses trump suit, must win all 13 tricks
    /// Similar to Grand Slam but with a chosen trump
    Slam,
}

/// Manille game types and structures
/// Manille is a team-based trick-taking game where card points are counted
/// Card values: 10=5pts, A=4pts, K=3pts, Q=2pts, J=1pt, 9/8/7=0pts
/// Total points per round: 60 (all cards)

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ManilleTrump {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    #[serde(rename = "no_trump")]
    NoTrump,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ManilleRoundRequest {
    /// Points scored by team 1 (players 0 and 2)
    pub team1_points: u8,
    /// Trump suit chosen by the dealer
    pub trump: ManilleTrump,
    /// Multiplier: 1=normal, 2=went along, 4=went against
    #[serde(default = "default_multiplier")]
    pub multiplier: u8,
}

fn default_multiplier() -> u8 {
    1
}

/// Press (Pressen/Zetten) game types and structures
/// Press is a 4-player team trick-taking game popular in the Flemish Kempen region.
/// Similar to Klaverjas/Euchre. Uses a 32-card piquet deck.
/// 
/// Teams: Player 0 & 2 vs Player 1 & 3 (sitting across from each other)
/// 
/// Card ranking in trump suit:
/// - Jack of trump (Right Bower): highest
/// - Jack of same color (Left Bower): second highest  
/// - Then: A, K, Q, 10, 9, 8, 7
/// 
/// Players bid how many tricks their team will make (max 8).
/// Highest bidder chooses trump and must make their bid.
/// Score = bid if successful, -bid if failed.
/// First team to 42 points wins.

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PressTrump {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PressRoundRequest {
    /// The bid made by the bidding team (1-8 tricks)
    pub bid: u8,
    /// Which team made the bid: 1 for team 1 (players 0 & 2), 2 for team 2 (players 1 & 3)
    pub bidding_team: u8,
    /// Tricks actually won by the bidding team
    pub tricks_won: u8,
    /// Trump suit chosen by the highest bidder
    #[allow(dead_code)]
    pub trump: PressTrump,
}
