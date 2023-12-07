use nalgebra::Vector3;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, SteeringBehavior};

pub struct RunningState {}

impl RunningState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        Self::check_collision(player, objects_positions, result);

        let direction = SteeringBehavior::Arrive {
            target: objects_positions.ball_positions,
            slowing_distance: 10.0,
        }.calculate(player);

        player.velocity = Vector3::new(direction.velocity.x, direction.velocity.y, 0.0);

        None
    }

    fn check_collision(player: &mut MatchPlayer, objects_positions: &MatchObjectsPositions, result: &mut Vec<PlayerUpdateEvent>) {
        if objects_positions
            .ball_positions
            .distance_to(&player.position)
            < 5.0
        {
            result.push(PlayerUpdateEvent::TacklingBall(player.player_id))
        }
    }
}
