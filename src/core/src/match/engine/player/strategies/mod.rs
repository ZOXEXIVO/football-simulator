pub mod defenders;
pub mod fordwarders;
pub mod goalkeeper;
pub mod midfielders;

pub use defenders::*;
pub use fordwarders::*;
pub use goalkeeper::*;
pub use midfielders::*;
use nalgebra::Vector3;
use crate::r#match::{GameState, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent};

use crate::{PlayerFieldPositionGroup};

pub trait VelocityStrategy {
    fn detect_velocity(
        &self,
        current_time: u64,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &GameState,
    ) -> Vector3<f32>;
}

impl VelocityStrategy for PlayerFieldPositionGroup {
    fn detect_velocity(&self, current_time: u64, player: &MatchPlayer, result: &mut Vec<PlayerUpdateEvent>, objects_positions: &MatchObjectsPositions, state: &GameState) -> Vector3<f32> {
        match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::detect_velocity(
                current_time,
                player,
                result,
                objects_positions,
                state,
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::detect_velocity(
                current_time,
                player,
                result,
                objects_positions,
                state,
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::detect_velocity(
                current_time,
                player,
                result,
                objects_positions,
                state,
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::detect_velocity(
                current_time,
                player,
                result,
                objects_positions,
                state,
            ),
        }
    }
}