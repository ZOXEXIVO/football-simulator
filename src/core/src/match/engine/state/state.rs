#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchState {
    FirstHalf,
    HalfTime,
    SecondHalf,
    ExtraTime,
    PenaltyShootout,
    End,
}
