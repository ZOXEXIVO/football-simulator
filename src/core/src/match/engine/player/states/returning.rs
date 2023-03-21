use crate::r#match::position::FieldPosition;
use crate::r#match::{
    MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, SteeringBehavior,
};
use nalgebra::Vector2;

pub struct ReturningState {}

impl ReturningState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if player.position.distance_to(&player.start_position) < 10.0 {
            return Some(PlayerState::Standing);
        } else {
            if player.in_state_time == 0 {
                let calculated_steering = SteeringBehavior::Seek {
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
