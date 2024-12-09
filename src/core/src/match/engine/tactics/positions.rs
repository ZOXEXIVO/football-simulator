use crate::PlayerPositionType;
use crate::r#match::position::MatchTacticalPosition;

#[derive(Debug, Clone)]
pub struct TacticalPositions {
    pub current_position: PlayerPositionType,
    pub tactical_positions: Vec<MatchTacticalPosition>
}

impl TacticalPositions {
    pub fn new(current_position: PlayerPositionType) -> Self {
        TacticalPositions {
            current_position,
            tactical_positions: Vec::new()
        }
    }
}