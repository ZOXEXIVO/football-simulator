use crate::r#match::{MatchObjectsPositions, MatchPlayer, MatchState, PlayerUpdateEvent};
use nalgebra::Vector2;

pub struct MidfielderStrategies {}

impl MidfielderStrategies {
    pub fn detect_velocity(
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }
}
