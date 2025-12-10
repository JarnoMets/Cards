//! Routes module - split into logical submodules for maintainability
//!
//! - `common`: Shared types and response structures
//! - `games`: Game CRUD endpoints
//! - `players`: Player search, profile, and management endpoints
//! - `leaderboard`: Leaderboard and statistics endpoints
//! - `elo`: ELO rating endpoints
//! - `admin`: Admin operations (recalculate, email, config)

mod admin;
mod common;
mod elo;
mod games;
mod leaderboard;
mod players;

// Re-export all route handlers for use in main.rs
pub use admin::*;
pub use common::*;
pub use elo::*;
pub use games::*;
pub use leaderboard::*;
pub use players::*;
