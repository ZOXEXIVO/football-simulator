use crate::r#match::{MatchObjectsPositions, MatchPlayer, MatchState, PlayerUpdateEvent};
use nalgebra::Vector2;

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn move_to(
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
    }
}
