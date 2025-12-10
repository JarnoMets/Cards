use super::GameService;
use crate::models::{AddRoundRequest, Game};

pub struct DoubleKingService;

impl DoubleKingService {
    pub fn new() -> Self {
        DoubleKingService
    }
}

impl GameService for DoubleKingService {
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String> {
        let raw_scores = request.scores.as_ref().ok_or_else(|| {
            format!(
                "Scores are required for Double King (expected {} entries)",
                game.players.len()
            )
        })?;

        if raw_scores.len() != game.players.len() {
            return Err(format!(
                "Score count {} must match player count {}",
                raw_scores.len(),
                game.players.len()
            ));
        }

        let round = game.current_round + 1; // 1-based
        let max_rounds = if game.players.len() == 3 { 18 } else { 20 };
        if round < 1 || round > max_rounds {
            return Err(format!(
                "Invalid round {} for Double King (max {})",
                round, max_rounds
            ));
        }

        // Check that all scores are non-negative
        if raw_scores.iter().any(|&s| s < 0) {
            return Err("All scores must be non-negative".to_string());
        }

        Ok(())
    }

    fn calculate_round_scores(
        &self,
        game: &Game,
        request: &AddRoundRequest,
    ) -> Result<Vec<i32>, String> {
        self.validate_round_input(game, request)?;

        let raw_scores = request.scores.clone().ok_or_else(|| {
            format!(
                "Scores are required for Double King (expected {} entries)",
                game.players.len()
            )
        })?;

        let player_count = game.players.len();
        let mut round_scores = vec![0; player_count];
        let total_tricks = if player_count == 3 { 17 } else { 13 };

        // Use provided game_index if available (for Double King with flexible order)
        // Otherwise, fall back to calculating from round number (for compatibility)
        let game_type = if let Some(idx) = request.game_index {
            // Direct mapping from game index to game type
            if idx < 6 {
                // Penalty games: indices 0-5 map to game types 1-6
                (idx as usize) + 1
            } else {
                // Positive games: indices 6-9 map to game types 7-10
                (idx as usize) - 6 + 7
            }
        } else {
            // Fallback: calculate from round number (for Hearts/King)
            let round = game.current_round + 1; // 1-based
            if round <= 12 {
                // Penalties: rounds 1-12 map to games 1-6 (with repetition)
                ((round - 1) / 2) + 1
            } else {
                // Positives: rounds 13+ map to games 7-10
                (((round - 13) / 2) + 7).min(10)
            }
        };

        // Validate and compute scores based on game type in a single match
        let total: i32 = raw_scores.iter().sum();
        let max_score: i32 = *raw_scores.iter().max().unwrap_or(&0);

        match game_type {
            1 => {
                // No tricks: max 0-13 per player, total 13
                if max_score > total_tricks || total != total_tricks {
                    return Err(format!(
                        "No Tricks: max score {total_tricks}, total must be {total_tricks} (got {total})"
                    ));
                }
                // Scoring: -20 per trick (4p), -15 per trick (3p)
                let per_trick = if player_count == 3 { -15 } else { -20 };
                for (i, &tricks) in raw_scores.iter().enumerate() {
                    round_scores[i] = per_trick * tricks;
                }
            }
            2 => {
                // No hearts: max 0-13 per player, total 13
                if max_score > 13 || total != 13 {
                    return Err(format!(
                        "No Hearts: max score 13, total must be 13 (got {})",
                        total
                    ));
                }
                // Scoring: -20 per heart taken
                for (i, &hearts) in raw_scores.iter().enumerate() {
                    round_scores[i] = -20 * hearts;
                }
            }
            3 => {
                // No kings & jacks: max 0-8 per player, total 8
                if max_score > 8 || total != 8 {
                    return Err(format!(
                        "No Kings & Jacks: max score 8, total must be 8 (got {})",
                        total
                    ));
                }
                // Scoring: -30 per card taken
                for (i, &cards) in raw_scores.iter().enumerate() {
                    round_scores[i] = -30 * cards;
                }
            }
            4 => {
                // No queens: max 0-4 per player, total 4
                if max_score > 4 || total != 4 {
                    return Err(format!(
                        "No Queens: max score 4, total must be 4 (got {})",
                        total
                    ));
                }
                // Scoring: -50 per queen taken
                for (i, &queens) in raw_scores.iter().enumerate() {
                    round_scores[i] = -50 * queens;
                }
            }
            5 => {
                // No king of hearts: max 0-1 per player, total 1
                if max_score > 1 || total != 1 {
                    return Err(format!(
                        "No King of Hearts: max score 1, total must be 1 (got {})",
                        total
                    ));
                }
                // Scoring: -160 to player who took the king of hearts
                for (i, &has_king_hearts) in raw_scores.iter().enumerate() {
                    round_scores[i] = if has_king_hearts > 0 { -160 } else { 0 };
                }
            }
            6 => {
                // Not 7th/last: max 0-2 per player, total 2
                if max_score > 2 || total != 2 {
                    return Err(format!(
                        "Not 7th/Last: max score 2, total must be 2 (got {})",
                        total
                    ));
                }
                // Scoring: -90 per penalty trick (4p) or -80 (3p)
                let per_trick = if player_count == 3 { -80 } else { -90 };
                for (i, &penalty_tricks) in raw_scores.iter().enumerate() {
                    round_scores[i] = per_trick * penalty_tricks;
                }
            }
            7 | 8 | 9 | 10 => {
                // Trump games: max 0-13 per player, total 13
                if max_score > total_tricks || total != total_tricks {
                    return Err(format!(
                        "Trump games: max score {total_tricks}, total must be {total_tricks} (got {total})"
                    ));
                }
                // Scoring: +25 per trick
                for (i, &tricks) in raw_scores.iter().enumerate() {
                    round_scores[i] = 25 * tricks;
                }
            }
            _ => {
                return Err(format!("Invalid game type {} for Double King", game_type));
            }
        }

        Ok(round_scores)
    }
}
