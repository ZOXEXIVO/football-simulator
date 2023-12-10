use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, RunningState, SteeringBehavior};
use nalgebra::Vector3;

pub struct WalkingState {}

impl WalkingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if context.time.time % 1000 == 0 {
            let direction = SteeringBehavior::Seek {
                target: objects_positions.ball_positions
            }.calculate(player);

            player.velocity = Vector3::new(direction.velocity.x, direction.velocity.y, 0.0);

            if in_state_time > 30 {
                player.state = PlayerState::Running;
            }

            // if player.skills.physical.acceleration > 15.0 {
            //     player.state = PlayerState::Running;
            // }
        }

        None
    }
}
