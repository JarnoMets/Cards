use super::GameService;
use crate::models::{AddRoundRequest, Game};

pub struct KingService;

impl KingService {
    pub fn new() -> Self {
        KingService
    }
}

impl GameService for KingService {
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String> {
        let raw_scores = request.scores.as_ref().ok_or_else(|| {
            format!(
                "Scores are required for King rounds (expected {} entries)",
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
        if round < 1 || round > 10 {
            return Err(format!("Invalid round {} for King", round));
        }

        let total: i32 = raw_scores.iter().sum();
        let max_score: i32 = *raw_scores.iter().max().unwrap_or(&0);
        let player_count = game.players.len();
        let total_tricks = if player_count == 3 { 17 } else { 13 };

        // Validate based on round/game type
        match round {
            1 => {
                // No tricks: total tricks equal number of cards dealt (17 for 3p, 13 for 4p)
                if max_score > total_tricks || total != total_tricks {
                    return Err(format!(
                        "No Tricks: max score per player {total_tricks}, total must be {total_tricks} (got total {total})"
                    ));
                }
            }
            2 => {
                // No hearts: max 0-13 hearts per player, total must equal 13
                if max_score > 13 || total != 13 {
                    return Err(format!(
                        "No Hearts: max score per player 13, total must be 13 (got total {})",
                        total
                    ));
                }
            }
            3 => {
                // No kings & jacks: max 0-8 per player, total must equal 8
                if max_score > 8 || total != 8 {
                    return Err(format!(
                        "No Kings & Jacks: max score per player 8, total must be 8 (got total {})",
                        total
                    ));
                }
            }
            4 => {
                // No queens: max 0-4 per player, total must equal 4
                if max_score > 4 || total != 4 {
                    return Err(format!(
                        "No Queens: max score per player 4, total must be 4 (got total {})",
                        total
                    ));
                }
            }
            5 => {
                // No king of hearts: max 0-1 per player, total must equal 1
                if max_score > 1 || total != 1 {
                    return Err(format!(
                        "No King of Hearts: max score per player 1, total must be 1 (got total {})",
                        total
                    ));
                }
            }
            6 => {
                // Not 7th/last: max 0-2 per player, total must equal 2
                if max_score > 2 || total != 2 {
                    return Err(format!(
                        "Not 7th/Last: max score per player 2, total must be 2 (got total {})",
                        total
                    ));
                }
            }
            7..=10 => {
                // Trump games: number of tricks equals dealt cards (17 for 3p, 13 for 4p)
                if max_score > total_tricks || total != total_tricks {
                    return Err(format!(
                        "Trump games: max score per player {total_tricks}, total must be {total_tricks} (got total {total})"
                    ));
                }
            }
            _ => return Err(format!("Invalid round {} for King", round)),
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
                "Scores are required for King rounds (expected {} entries)",
                game.players.len()
            )
        })?;

        let round = game.current_round + 1; // 1-based
        let player_count = game.players.len();
        let mut round_scores = vec![0; player_count];

        match round {
            1 => {
                // No tricks: -20 per trick taken (4 players), -15 per trick taken (3 players)
                let per_trick = if player_count == 3 { -15 } else { -20 };
                for (i, &tricks) in raw_scores.iter().enumerate() {
                    round_scores[i] = per_trick * tricks;
                }
            }
            2 => {
                // No hearts: -20 per heart taken
                for (i, &hearts) in raw_scores.iter().enumerate() {
                    round_scores[i] = -20 * hearts;
                }
            }
            3 => {
                // No kings and jacks: -30 per card taken
                for (i, &cards) in raw_scores.iter().enumerate() {
                    round_scores[i] = -30 * cards;
                }
            }
            4 => {
                // No queens: -50 per queen taken
                for (i, &queens) in raw_scores.iter().enumerate() {
                    round_scores[i] = -50 * queens;
                }
            }
            5 => {
                // No king of hearts: -160 if player took the king of hearts
                for (i, &has_king_hearts) in raw_scores.iter().enumerate() {
                    round_scores[i] = if has_king_hearts > 0 { -160 } else { 0 };
                }
            }
            6 => {
                // Not the 7th or last trick: -90 for each penalty trick taken (4 players), -80 (3 players)
                let per_trick = if player_count == 3 { -80 } else { -90 };
                for (i, &penalty_tricks) in raw_scores.iter().enumerate() {
                    round_scores[i] = per_trick * penalty_tricks;
                }
            }
            7 | 8 | 9 | 10 => {
                // Positive rounds: +25 per trick
                for (i, &tricks) in raw_scores.iter().enumerate() {
                    round_scores[i] = 25 * tricks;
                }
            }
            _ => {
                return Err(format!("Invalid round {} for King", round));
            }
        }

        Ok(round_scores)
    }
}
