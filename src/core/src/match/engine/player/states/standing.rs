use crate::r#match::{MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct StandingState {}

impl StandingState {
    pub fn process(
        _in_state_time: u64,
        _player: &mut MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        None
    }
}
