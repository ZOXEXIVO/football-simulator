use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    SteeringBehavior,
};
use nalgebra::Vector3;

pub struct WalkingState {}

impl WalkingState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        let direction = SteeringBehavior::Seek {
            target: Vector3::new(848.0, 275.0, 0.0),
        }
        .calculate(player);

        player.velocity = Vector3::new(direction.velocity.x, direction.velocity.y, 0.0);

        None
    }
}
