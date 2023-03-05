pub struct MatchState {
    game_state: Option<GameState>,
}

impl MatchState {
    pub fn new() -> Self {
        MatchState { game_state: None }
    }

    pub fn set_state(&mut self, game_state: GameState) {
        self.game_state = Some(game_state);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    FirstHalf,
    SecondHalf,
    ExtraTime,
    PenaltyShootout,
}
