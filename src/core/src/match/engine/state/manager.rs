use crate::r#match::MatchState;

pub struct StateManager {
    current_state: MatchState,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current_state: MatchState::FirstHalf,
        }
    }

    pub fn current(&self) -> MatchState {
        self.current_state
    }

    pub fn next(&mut self) -> MatchState {
        match self.current_state {
            MatchState::Initial => {
                self.current_state = MatchState::FirstHalf;
                self.current_state
            }
            MatchState::FirstHalf => {
                self.current_state = MatchState::HalfTime;
                self.current_state
            }
            MatchState::HalfTime => {
                self.current_state = MatchState::SecondHalf;
                self.current_state
            }
            MatchState::SecondHalf => {
                self.current_state = MatchState::ExtraTime;
                self.current_state
            }
            MatchState::ExtraTime => {
                self.current_state = MatchState::PenaltyShootout;
                self.current_state
            }
            MatchState::PenaltyShootout => {
                self.current_state = MatchState::End;
                self.current_state
            }
            MatchState::End => self.current_state,
        }
    }
}
