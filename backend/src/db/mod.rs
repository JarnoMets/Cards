//! Database module - split into logical submodules for maintainability
//!
//! - `core`: Database connection, error types, and common types
//! - `games`: Game CRUD operations
//! - `players`: Player search and management
//! - `elo`: ELO/skill rating calculations and history
//! - `leaderboard`: Leaderboard statistics and caching
//! - `settings`: Player settings for email preferences

mod core;
mod elo;
mod games;
mod leaderboard;
mod players;
mod settings;

// Re-export types from core
pub use core::*;

// Re-export types from elo (these are used by routes)
#[allow(unused_imports)]
pub use elo::{EloHistoryEntry, GameEloChange, OverallEloRow, PlayerEloRow};

// Re-export types from leaderboard
pub use leaderboard::{LeaderboardStatRow, RecordScoreRow};

// Re-export types from settings
pub use settings::{EmailDailyStats, PlayerSettings};
