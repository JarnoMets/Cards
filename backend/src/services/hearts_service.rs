use super::GameService;
use crate::models::{AddRoundRequest, Game};
use serde_json::json;

pub struct HeartsService;

impl HeartsService {
    pub fn new() -> Self {
        HeartsService
    }
}

impl GameService for HeartsService {
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String> {
        let raw_scores = request.scores.as_ref().ok_or_else(|| {
            serde_json::to_string(&json!({
                "error_key": "hearts.missing_scores"
            }))
            .unwrap()
        })?;

        // Hearts validation:
        // 1. Scores must sum to 26 (Queen of spades is 13 points)
        // 2. Max score per player: 0-26 (if shooting the moon) or 0-13 (normal)
        // 3. Total must always equal 26 (all points must be distributed)

        if raw_scores.len() != game.players.len() {
            return Err(serde_json::to_string(&json!({
                "error_key": "hearts.invalid_score_count",
                "params": { "expected": game.players.len(), "got": raw_scores.len() }
            }))
            .unwrap());
        }

        let total_points: i32 = raw_scores.iter().sum();
        if total_points != 26 && total_points != 26 * (game.players.len() as i32 - 1) {
            return Err(serde_json::to_string(&json!({
                "error_key": "hearts.invalid_total",
                "params": { "expected": 26, "got": total_points }
            }))
            .unwrap());
        }

        let max_score = raw_scores.iter().max().unwrap_or(&0);

        // Check if it's a "shooting the moon" scenario (one player has 26 and others 0, or one has 0 and others 26)
        let shooter_count = raw_scores.iter().filter(|&&s| s == 26).count();
        let zero_count = raw_scores.iter().filter(|&&s| s == 0).count();

        if (shooter_count == 1 && zero_count == game.players.len() - 1)
            || (shooter_count == game.players.len() - 1 && zero_count == 1)
        {
            // Shooting the moon is valid
            if max_score > &26 {
                return Err(serde_json::to_string(&json!({
                    "error_key": "hearts.shooter_too_high",
                    "params": { "max_allowed": 26, "got": max_score }
                }))
                .unwrap());
            }
            return Ok(());
        }

        // Normal round: each player can have 0-26 points (shooting-the-moon handled above)
        if max_score > &26 {
            return Err(serde_json::to_string(&json!({
                "error_key": "hearts.max_exceeded",
                "params": { "max_allowed": 26, "got": max_score }
            }))
            .unwrap());
        }

        // At least one player must have 13 points (Queen of Spades)
        if max_score < &13 {
            return Err(serde_json::to_string(&json!({
                "error_key": "hearts.missing_queen",
                "params": { "required": 13, "got": max_score }
            }))
            .unwrap());
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
            serde_json::to_string(&json!({
                "error_key": "hearts.missing_scores"
            }))
            .unwrap()
        })?;

        // Check if it's a "shooting the moon": one player has 26 and others 0, or one has 0 and others 26
        let shooter_idx = if raw_scores.iter().filter(|&&s| s == 26).count() == 1
            && raw_scores.iter().filter(|&&s| s == 0).count() == game.players.len() - 1
        {
            raw_scores.iter().position(|&s| s == 26)
        } else if raw_scores.iter().filter(|&&s| s == 26).count() == game.players.len() - 1
            && raw_scores.iter().filter(|&&s| s == 0).count() == 1
        {
            raw_scores.iter().position(|&s| s == 0)
        } else {
            None
        };

        Ok(if shooter_idx.is_none() {
            // Normal round: scores are applied as-is
            raw_scores
        } else {
            // Shooting the moon: determine the scoring method
            let shooter = shooter_idx.unwrap();

            // Calculate what totals would be with shooter getting 0, others getting +26
            // NOTE: game end (who actually wins) is determined at the end of the game
            // by comparing totals, NOT by altering this per-round scoring.
            let current_totals: Vec<i32> = game.players.iter().map(|p| p.total_score).collect();
            let mut totals_with_default = current_totals.clone();
            for i in 0..totals_with_default.len() {
                if i != shooter {
                    totals_with_default[i] += 26;
                }
            }

            // Apply 100 reset rule: if someone hits exactly 100, they reset to 0
            let mut totals_after_reset = totals_with_default.clone();
            for i in 0..totals_after_reset.len() {
                if totals_after_reset[i] == 100 {
                    totals_after_reset[i] = 0;
                }
            }

            // Someone reaches or exceeds 100 with default scoring (before reset)
            let someone_reaches_100 = totals_with_default.iter().any(|&t| t >= 100);

            // Determine if shooter has the lowest score AFTER accounting for 100 reset
            let min_total = *totals_after_reset.iter().min().unwrap();
            let shooter_has_lowest = totals_after_reset[shooter] == min_total;

            let mut scores = vec![0; game.players.len()];
            if someone_reaches_100 && shooter_has_lowest {
                // Shooter wins the game; keep default moon scoring
                for i in 0..scores.len() {
                    if i != shooter {
                        scores[i] = 26;
                    }
                }
            } else if someone_reaches_100 && !shooter_has_lowest {
                // Shooter loses the game: shooter gets -26, others 0
                scores[shooter] = -26;
            } else {
                // No-one at 100 yet, default moon scoring
                for i in 0..scores.len() {
                    if i != shooter {
                        scores[i] = 26;
                    }
                }
            }

            scores
        })
    }
}
