use crate::r#match::{MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct StandingState {}

impl StandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        None
    }
}
