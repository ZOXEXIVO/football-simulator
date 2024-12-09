use crate::r#match::position::MatchTacticalPosition;

#[derive(Debug, Clone)]
pub struct MatchTactics {
    pub tactical_positions: Vec<MatchTacticalPosition>
}

impl MatchTactics {
    pub fn new(tactical_positions: Vec<MatchTacticalPosition>) -> Self {
        MatchTactics {
            tactical_positions
        }
    }
}