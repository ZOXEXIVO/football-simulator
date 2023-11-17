use crate::r#match::MatchState;

pub struct MatchGameState {
    pub game_state: Option<MatchState>,
    pub ball_state: Option<BallState>,
}

impl MatchGameState {
    pub fn new() -> Self {
        MatchGameState {
            game_state: None,
            ball_state: None,
        }
    }

    pub fn set_state(&mut self, game_state: MatchState) {
        self.game_state = Some(game_state);
    }

    pub fn set_ball_state(&mut self, ball_state: BallState) {
        self.ball_state = Some(ball_state);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BallState {
    HomeSide,
    AwaySide,
}
