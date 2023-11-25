use crate::r#match::{GameState, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent};
use nalgebra::Vector3;

pub struct MidfielderStrategies {}

impl MidfielderStrategies {
    pub fn detect_velocity(
        _current_time: u64,
        _player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
        _state: &GameState,
    ) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
