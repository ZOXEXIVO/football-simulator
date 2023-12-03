use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct StandingState {}

impl StandingState {
    pub fn process(
        _in_state_time: u64,
        _player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        None
    }
}
