use nalgebra::Vector3;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, SteeringBehavior};

pub struct RunningState {}

impl RunningState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        Self::check_collision(player, objects_positions, result);

        if in_state_time > 100 {

        }

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
