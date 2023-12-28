use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent,
    SteeringBehavior,
};
use nalgebra::Vector3;

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn detect_velocity(
        context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Vector3<f32> {

        let vel = SteeringBehavior::Arrive {
            target: objects_positions.ball_position,
            slowing_distance: 10.0
        }
            .calculate(player)
            .velocity;

        vel
    }
}
