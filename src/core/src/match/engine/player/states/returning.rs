use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    SteeringBehavior,
};

pub struct ReturningState {}

impl ReturningState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if player.position.distance_to(&player.start_position) < 10.0 {
            return Some(PlayerState::Standing);
        }

        None
    }
}
