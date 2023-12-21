use crate::r#match::{
    GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent,
    SteeringBehavior,
};
use crate::FloatUtils;
use nalgebra::Vector3;

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn detect_velocity(
        context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Vector3<f32> {
        SteeringBehavior::Seek {
            target: objects_positions.ball_positions,
        }
        .calculate(player)
        .velocity
    }
}
