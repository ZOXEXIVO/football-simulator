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

    pub fn next(&mut self) -> Option<MatchState> {
        match self.current_state {
            MatchState::Initial => {
                self.current_state = MatchState::FirstHalf;
                Some(self.current_state)
            }
            MatchState::FirstHalf => {
                self.current_state = MatchState::HalfTime;
                Some(self.current_state)
            }
            MatchState::HalfTime => {
                self.current_state = MatchState::SecondHalf;
                Some(self.current_state)
            }
            MatchState::SecondHalf => {
                self.current_state = MatchState::ExtraTime;
                Some(self.current_state)
            }
            MatchState::ExtraTime => {
                self.current_state = MatchState::PenaltyShootout;
                Some(self.current_state)
            }
            MatchState::PenaltyShootout => {
                self.current_state = MatchState::End;
                Some(self.current_state)
            }
            MatchState::End => None
        }
    }
}
