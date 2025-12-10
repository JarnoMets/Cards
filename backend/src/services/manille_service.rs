use super::GameService;
use crate::models::{AddRoundRequest, Game, ManilleTrump};
use serde_json::json;

/// Manille Service
/// 
/// Manille is a 4-player team game played with a 32-card piquet deck.
/// Teams: Player 0 & 2 vs Player 1 & 3 (sitting across from each other)
/// 
/// Card values (from highest to lowest):
/// - 10 (Manille): 5 points
/// - A: 4 points  
/// - K: 3 points
/// - Q: 2 points
/// - J: 1 point
/// - 9, 8, 7: 0 points
/// 
/// Total points per round: 60 (4 suits × 15 points per suit)
/// 
/// Scoring modifiers:
/// - No Trump: points are doubled
/// - "Going along" (tegengaan): points are doubled
/// - "Going against" (meegaan): points are quadrupled
/// - Max multiplier is 4 (no trump + going along cannot stack to 8)
/// 
/// Game ends when a team reaches the target score (default: 101)

pub struct ManilleService;

impl ManilleService {
    pub fn new() -> Self {
        Self
    }
}

fn error_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

impl GameService for ManilleService {
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String> {
        if game.players.len() != 4 {
            return Err(error_json("Manille must have exactly 4 players"));
        }

        let round = request.manille_round.as_ref().ok_or_else(|| {
            error_json("manilleRound payload is required for Manille rounds")
        })?;

        // Validate team1_points is between 0 and 60
        if round.team1_points > 60 {
            return Err(error_json("Team points cannot exceed 60 (total points in a round)"));
        }

        // Validate multiplier
        if round.multiplier == 0 || round.multiplier > 4 {
            return Err(error_json("Multiplier must be 1, 2, or 4"));
        }

        // No trump doubles points, going along/against also multiplies
        // But max multiplier is 4 (no stacking to 8)
        let base_multiplier = if round.trump == ManilleTrump::NoTrump { 2 } else { 1 };
        let effective_multiplier = base_multiplier * round.multiplier;
        if effective_multiplier > 4 {
            return Err(error_json("Maximum multiplier is 4 (no trump + going along cannot stack)"));
        }

        Ok(())
    }

    fn calculate_round_scores(
        &self,
        game: &Game,
        request: &AddRoundRequest,
    ) -> Result<Vec<i32>, String> {
        self.validate_round_input(game, request)?;
        
        let round = request.manille_round.as_ref().unwrap();
        
        let team1_points = round.team1_points as i32;
        let team2_points = 60 - team1_points;
        
        // Calculate the difference (positive if team 1 wins, negative if team 2 wins)
        let point_diff = team1_points - team2_points;
        
        // Apply multiplier
        // No trump doubles, going along/against multiplies further (max 4)
        let base_multiplier = if round.trump == ManilleTrump::NoTrump { 2 } else { 1 };
        let effective_multiplier = (base_multiplier * round.multiplier as i32).min(4);
        
        let score_diff = point_diff * effective_multiplier;
        
        // Team 1: players 0 and 2
        // Team 2: players 1 and 3
        // In Manille, scores are typically cumulative points scored, not deltas
        // But for consistency with other games, we'll record the round score difference
        let scores = vec![
            score_diff,      // Player 0 (team 1)
            -score_diff,     // Player 1 (team 2)
            score_diff,      // Player 2 (team 1)
            -score_diff,     // Player 3 (team 2)
        ];

        Ok(scores)
    }
}
