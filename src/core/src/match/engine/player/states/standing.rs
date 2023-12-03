use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct StandingState {}

impl StandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if in_state_time > 1000 {
            return Some(PlayerState::Walking);
        }

        if context.time.time % 1000 == 0 {

        }

        None
    }
}
