use crate::r#match::MatchState;

pub struct GameState {
    pub match_state: MatchState,
    pub ball_state: Option<BallState>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            match_state: MatchState::Initial,
            ball_state: None,
        }
    }

    pub fn set(&mut self, match_state: MatchState) {
        self.match_state = match_state;
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
