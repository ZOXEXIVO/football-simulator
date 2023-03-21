pub struct MatchState {
    pub game_state: Option<GameState>,
    pub ball_state: Option<BallState>,
}

impl MatchState {
    pub fn new() -> Self {
        MatchState {
            game_state: None,
            ball_state: None,
        }
    }

    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = Some(game_state);
    }

    pub fn set_ball_state(&mut self, ball_state: BallState) {
        self.ball_state = Some(ball_state);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    FirstHalf,
    SecondHalf,
    ExtraTime,
    PenaltyShootout,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BallState {
    HomeSide,
    AwaySide,
}
