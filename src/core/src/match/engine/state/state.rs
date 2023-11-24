#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchState {
    Initial,
    FirstHalf,
    HalfTime,
    SecondHalf,
    ExtraTime,
    PenaltyShootout,
    End,
}

impl MatchState {
    pub fn is_end_state(&self) -> bool {
        *self == MatchState::End
    }
}
