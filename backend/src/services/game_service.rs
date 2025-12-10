use crate::models::{AddRoundRequest, Game};

pub trait GameService: Send + Sync {
    /// Calculate round scores based on game-specific rules
    fn calculate_round_scores(
        &self,
        game: &Game,
        request: &AddRoundRequest,
    ) -> Result<Vec<i32>, String>;

    /// Get game-specific validation rules
    fn validate_round_input(&self, game: &Game, request: &AddRoundRequest) -> Result<(), String>;
}
