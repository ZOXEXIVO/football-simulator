use crate::r#match::{GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent};
use nalgebra::Vector3;
use crate::FloatUtils;

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn detect_velocity(
        context: &mut MatchContext,
        _player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Vector3<f32> {
        Vector3::new(FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.1, 0.3), FloatUtils::random(-0.4, 0.3))
    }
}
