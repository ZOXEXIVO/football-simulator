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
