use crate::r#match::{MatchObjectsPositions, MatchPlayer, MatchState, PlayerUpdateEvent};
use nalgebra::Vector3;

pub struct MidfielderStrategies {}

impl MidfielderStrategies {
    pub fn detect_velocity(
        current_time: u64,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
