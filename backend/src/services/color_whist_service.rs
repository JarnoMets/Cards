use super::GameService;
use crate::models::{AddRoundRequest, ColorWhistContract, Game};
use serde_json::json;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct ValidatedSelection {
    player_index: usize,
    contract: ColorWhistContract,
    tricks: u8,
}

#[derive(Debug, Clone)]
enum PrimaryContract {
    Team {
        contract: ColorWhistContract,
        players: Vec<ValidatedSelection>,
        tricks: u8,
    },
    Solo {
        contract: ColorWhistContract,
        player: ValidatedSelection,
    },
}

#[derive(Debug, Clone)]
struct ParsedRound {
    primary: Option<PrimaryContract>,
    miseries: Vec<ValidatedSelection>,
}

pub struct ColorWhistService;

impl ColorWhistService {
    pub fn new() -> Self {
        Self
    }

    fn parse_round(&self, game: &Game, request: &AddRoundRequest) -> Result<ParsedRound, String> {
        if game.players.len() != 4 {
            return Err(error_json("Color Whist must have exactly 4 players"));
        }

        let round = request.color_whist_round.as_ref().ok_or_else(|| {
            error_json("colorWhistRound payload is required for Color Whist rounds")
        })?;

        if round.selections.is_empty() {
            return Err(error_json("At least one contract selection is required"));
        }

        let mut seen_players = vec![false; game.players.len()];
        let mut validated: Vec<ValidatedSelection> = Vec::new();
        for selection in &round.selections {
            if selection.player_index >= game.players.len() {
                return Err(error_json("Player index is out of range for this game"));
            }
            if seen_players[selection.player_index] {
                return Err(error_json(
                    "Each player can only select one contract per round",
                ));
            }
            seen_players[selection.player_index] = true;

            if matches!(selection.contract, ColorWhistContract::None) {
                return Err(error_json("'none' is not a valid contract submission"));
            }

            let tricks = selection
                .tricks
                .ok_or_else(|| error_json("Each contract requires the number of tricks played"))?;
            if tricks > 13 {
                return Err(error_json("Trick counts must be between 0 and 13"));
            }

            validated.push(ValidatedSelection {
                player_index: selection.player_index,
                contract: selection.contract,
                tricks,
            });
        }

        let mut primary_contract: Option<ColorWhistContract> = None;
        let mut primary_players: Vec<ValidatedSelection> = Vec::new();
        let mut miseries: Vec<ValidatedSelection> = Vec::new();

        for selection in &validated {
            if is_misery_contract(selection.contract) {
                miseries.push(selection.clone());
                continue;
            }

            if let Some(existing) = primary_contract {
                if existing != selection.contract {
                    return Err(error_json(
                        "Only one primary contract can be played per round",
                    ));
                }
            } else {
                primary_contract = Some(selection.contract);
            }
            primary_players.push(selection.clone());
        }

        if primary_contract.is_none() && miseries.is_empty() {
            return Err(error_json("No contracts were selected for this round"));
        }

        if primary_contract.is_some() && !miseries.is_empty() {
            return Err(error_json(
                "Misery contracts cannot be combined with other contracts",
            ));
        }

        let primary = match primary_contract {
            Some(contract) if is_team_contract(contract) => {
                if primary_players.len() != 2 {
                    return Err(error_json("Team contracts must have exactly two players"));
                }
                let first = primary_players[0].tricks;
                if primary_players[1].tricks != first {
                    return Err(error_json("Team players must record the same trick count"));
                }
                Some(PrimaryContract::Team {
                    contract,
                    players: primary_players,
                    tricks: first,
                })
            }
            Some(contract) if is_solo_contract(contract) => {
                if primary_players.len() != 1 {
                    return Err(error_json("Solo contracts must have exactly one player"));
                }
                Some(PrimaryContract::Solo {
                    contract,
                    player: primary_players[0].clone(),
                })
            }
            Some(contract) => {
                return Err(error_json(&format!(
                    "Unsupported contract {:?} for automated scoring",
                    contract
                )))
            }
            None => None,
        };

        Ok(ParsedRound { primary, miseries })
    }
}

/// Helper to build a JSON error string that frontend can parse
fn error_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

impl GameService for ColorWhistService {
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String> {
        self.parse_round(game, request).map(|_| ())
    }

    fn calculate_round_scores(
        &self,
        game: &Game,
        request: &AddRoundRequest,
    ) -> Result<Vec<i32>, String> {
        let parsed = self.parse_round(game, request)?;
        let mut scores = vec![0; game.players.len()];

        if let Some(primary) = parsed.primary {
            match primary {
                PrimaryContract::Team {
                    contract,
                    players,
                    tricks,
                } => {
                    let value = match contract {
                        ColorWhistContract::Accept8 => compute_accept_value(8, tricks as i32),
                        ColorWhistContract::Accept9 => compute_accept_value(9, tricks as i32),
                        ColorWhistContract::Accept10 => compute_accept_value(10, tricks as i32),
                        ColorWhistContract::Accept11 => compute_accept_value(11, tricks as i32),
                        ColorWhistContract::Accept12 => compute_accept_value(12, tricks as i32),
                        ColorWhistContract::Accept13 => compute_accept_value(13, tricks as i32),
                        ColorWhistContract::Trull => compute_trull_value(tricks as i32),
                        _ => return Err(error_json("Unsupported team contract")),
                    };
                    apply_team_score(&mut scores, game.players.len(), &players, value);
                }
                PrimaryContract::Solo { contract, player } => {
                    let base_value = match contract {
                        ColorWhistContract::Solo5 => compute_solo_value(5, player.tricks as i32),
                        ColorWhistContract::Solo6 => compute_solo_value(6, player.tricks as i32),
                        ColorWhistContract::Solo7 => compute_solo_value(7, player.tricks as i32),
                        ColorWhistContract::Solo8 => compute_solo_value(8, player.tricks as i32),
                        ColorWhistContract::Abondance9 => {
                            compute_abondance_value(9, player.tricks as i32)
                        }
                        ColorWhistContract::Abondance10 => {
                            compute_abondance_value(10, player.tricks as i32)
                        }
                        ColorWhistContract::Abondance11 => {
                            compute_abondance_value(11, player.tricks as i32)
                        }
                        ColorWhistContract::Abondance12 => {
                            compute_abondance_value(12, player.tricks as i32)
                        }
                        ColorWhistContract::Piccolo => compute_piccolo_value(player.tricks as i32)?,
                        ColorWhistContract::GrandSlam => {
                            compute_grand_slam_value(player.tricks as i32)?
                        }
                        _ => return Err(error_json("Unsupported solo contract")),
                    };
                    apply_solo_score(
                        &mut scores,
                        game.players.len(),
                        player.player_index,
                        base_value,
                    );
                }
            }
        }

        for selection in parsed.miseries {
            let value = misery_points(selection.contract)?;
            let success = selection.tricks == 0;
            apply_misery_score(
                &mut scores,
                game.players.len(),
                selection.player_index,
                value,
                success,
            );
        }

        let total: i32 = scores.iter().sum();
        if total != 0 {
            return Err(error_json(
                "Internal scoring error: totals did not balance to zero",
            ));
        }

        Ok(scores)
    }
}

fn is_team_contract(contract: ColorWhistContract) -> bool {
    matches!(
        contract,
        ColorWhistContract::Accept8
            | ColorWhistContract::Accept9
            | ColorWhistContract::Accept10
            | ColorWhistContract::Accept11
            | ColorWhistContract::Accept12
            | ColorWhistContract::Accept13
            | ColorWhistContract::Trull
    )
}

fn is_solo_contract(contract: ColorWhistContract) -> bool {
    matches!(
        contract,
        ColorWhistContract::Solo5
            | ColorWhistContract::Solo6
            | ColorWhistContract::Solo7
            | ColorWhistContract::Solo8
            | ColorWhistContract::Abondance9
            | ColorWhistContract::Abondance10
            | ColorWhistContract::Abondance11
            | ColorWhistContract::Abondance12
            | ColorWhistContract::Piccolo
            | ColorWhistContract::GrandSlam
    )
}

fn is_misery_contract(contract: ColorWhistContract) -> bool {
    matches!(
        contract,
        ColorWhistContract::SmallMisery
            | ColorWhistContract::LargeMisery
            | ColorWhistContract::OpenMisery
    )
}

fn compute_accept_value(target: i32, actual: i32) -> i32 {
    if actual == 13 {
        return 30;
    }
    if actual >= target {
        let base = if target == 13 {
            30
        } else {
            8 + 3 * (target - 8)
        };
        return base + 3 * (actual - target);
    }

    let diff = target - actual;
    let base_penalty = 11 + 3 * (target - 8);
    -(base_penalty + 3 * (diff - 1))
}

fn compute_solo_value(target: i32, actual: i32) -> i32 {
    if actual >= target {
        if target == 8 {
            return 7;
        }
        let capped = actual.min(8);
        let base = 3 + (target - 5);
        return base + (capped - target);
    }

    let diff = target - actual;
    let base_penalty = if target == 8 { 8 } else { target - 1 };
    -(base_penalty + (diff - 1))
}

fn compute_abondance_value(target: i32, actual: i32) -> i32 {
    if actual < target {
        return match target {
            9 => -10,
            10 => -15,
            11 => -20,
            12 => -30,
            _ => -10,
        };
    }

    let values = match target {
        9 => vec![10, 15, 20, 30],
        10 => vec![15, 20, 30],
        11 => vec![20, 30],
        12 => vec![30],
        _ => vec![10],
    };

    let idx = ((actual - target) as usize).min(values.len() - 1);
    values[idx]
}

fn compute_piccolo_value(actual: i32) -> Result<i32, String> {
    // Piccolo: must win exactly 1 trick
    // Success: +8, Failure: -8
    if actual == 1 {
        Ok(8)
    } else {
        Ok(-8)
    }
}

fn compute_grand_slam_value(actual: i32) -> Result<i32, String> {
    // Grand Slam: must win all 13 tricks
    // Success: +60, Failure: -60
    if actual == 13 {
        Ok(60)
    } else {
        Ok(-60)
    }
}

fn compute_trull_value(actual: i32) -> i32 {
    if actual == 13 {
        30
    } else if actual >= 8 {
        16
    } else {
        -16
    }
}

fn misery_points(contract: ColorWhistContract) -> Result<i32, String> {
    match contract {
        ColorWhistContract::SmallMisery => Ok(6),
        ColorWhistContract::LargeMisery => Ok(12),
        ColorWhistContract::OpenMisery => Ok(24),
        _ => Err(error_json("Unsupported misery contract")),
    }
}

fn apply_team_score(
    scores: &mut [i32],
    player_count: usize,
    participants: &[ValidatedSelection],
    value: i32,
) {
    let opponent_indices = opponents(player_count, participants.iter().map(|p| p.player_index));
    for player in participants {
        scores[player.player_index] += value;
    }
    for idx in opponent_indices {
        scores[idx] -= value;
    }
}

fn apply_solo_score(scores: &mut [i32], player_count: usize, solo_index: usize, base_value: i32) {
    let opponent_indices = opponents(player_count, std::iter::once(solo_index));
    for idx in &opponent_indices {
        scores[*idx] -= base_value;
    }
    scores[solo_index] += base_value * opponent_indices.len() as i32;
}

fn apply_misery_score(
    scores: &mut [i32],
    player_count: usize,
    player_index: usize,
    value: i32,
    success: bool,
) {
    let opponent_indices = opponents(player_count, std::iter::once(player_index));
    let delta = value * opponent_indices.len() as i32;
    if success {
        scores[player_index] += delta;
        for idx in opponent_indices {
            scores[idx] -= value;
        }
    } else {
        scores[player_index] -= delta;
        for idx in opponent_indices {
            scores[idx] += value;
        }
    }
}

fn opponents<'a, I>(player_count: usize, participants: I) -> Vec<usize>
where
    I: IntoIterator<Item = usize>,
{
    let participant_set: HashSet<usize> = participants.into_iter().collect();
    (0..player_count)
        .filter(|idx| !participant_set.contains(idx))
        .collect()
}
