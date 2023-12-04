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
        } else {
            if player.in_state_time == 0 {
                let _calculated_steering = SteeringBehavior::Seek {
                    target: player.start_position,
                }
                .calculate(player);

                // return Vector2::new(
                //     calculated_steering.velocity.x,
                //     calculated_steering.velocity.y,
                // );
            }
        }

        None
    }
}
