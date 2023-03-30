use crate::r#match::position::VectorExtensions;
use crate::r#match::{MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct RunningState {}

impl RunningState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if objects_positions
            .ball_positions
            .distance_to(&player.position)
            < 5.0
        {
            result.push(PlayerUpdateEvent::TacklingBall(player.player_id))
        }

        None
    }
}
