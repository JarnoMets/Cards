use super::GameService;
use crate::models::{AddRoundRequest, Game};
use serde_json::json;

/// Press (Pressen/Zetten) Service
/// 
/// Press is a 4-player team trick-taking game popular in the Flemish Kempen region.
/// Also known as "Zetten" or "Jokken". Similar to Klaverjas (Dutch) and Euchre (American).
/// 
/// Teams: Player 0 & 2 vs Player 1 & 3 (sitting across from each other)
/// 
/// Uses 32 cards (piquet deck): 7, 8, 9, 10, J, Q, K, A of each suit
/// Each player gets 8 cards.
/// 
/// Card ranking in trump suit:
/// - Jack of trump (Right Bower/Bovenjok): highest
/// - Jack of same color (Left Bower/Onderjok): second highest
///   (e.g., if Hearts is trump, Jack of Diamonds is the "left bower" and counts as a heart)
/// - Then: A, K, Q, 10, 9, 8, 7
/// 
/// Bidding:
/// - Each player bids how many tricks their team can make (1-8)
/// - Highest bidder chooses trump
/// - Team must make their bid to score positive
/// 
/// Scoring:
/// - If bidding team makes their bid: +bid points
/// - If bidding team fails: -bid points
/// - Non-bidding team always scores 0 for the round (they just defend)
/// 
/// Winning:
/// - First team to reach 42 points wins

pub struct PressService;

impl PressService {
    pub fn new() -> Self {
        Self
    }
}

fn error_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

impl GameService for PressService {
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String> {
        if game.players.len() != 4 {
            return Err(error_json("Press must have exactly 4 players"));
        }

        let round = request.press_round.as_ref().ok_or_else(|| {
            error_json("pressRound payload is required for Press rounds")
        })?;

        // Validate bid (1-8)
        if round.bid < 1 || round.bid > 8 {
            return Err(error_json("Bid must be between 1 and 8 tricks"));
        }

        // Validate bidding team (1 or 2)
        if round.bidding_team != 1 && round.bidding_team != 2 {
            return Err(error_json("Bidding team must be 1 or 2"));
        }

        // Validate tricks won (0-8)
        if round.tricks_won > 8 {
            return Err(error_json("Tricks won cannot exceed 8"));
        }

        Ok(())
    }

    fn calculate_round_scores(
        &self,
        game: &Game,
        request: &AddRoundRequest,
    ) -> Result<Vec<i32>, String> {
        self.validate_round_input(game, request)?;
        
        let round = request.press_round.as_ref().unwrap();
        
        let bid = round.bid as i32;
        let tricks_won = round.tricks_won as i32;
        
        // Check if bidding team made their bid
        let bidding_team_score = if tricks_won >= bid {
            bid  // Success: earn the bid
        } else {
            -bid  // Failure: lose the bid
        };
        
        // Team 1: players 0 and 2
        // Team 2: players 1 and 3
        let (team1_score, team2_score) = if round.bidding_team == 1 {
            (bidding_team_score, 0)
        } else {
            (0, bidding_team_score)
        };
        
        let scores = vec![
            team1_score,   // Player 0 (team 1)
            team2_score,   // Player 1 (team 2)
            team1_score,   // Player 2 (team 1)
            team2_score,   // Player 3 (team 2)
        ];

        Ok(scores)
    }
}
