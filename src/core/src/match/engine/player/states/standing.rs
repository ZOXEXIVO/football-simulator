use nalgebra::Vector3;
use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, SteeringBehavior};

pub struct StandingState {}

impl StandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if in_state_time > 20 {
            return Some(PlayerState::Walking);
        }

        if context.time.time % 1000 == 0 {}

        None
    }
}
