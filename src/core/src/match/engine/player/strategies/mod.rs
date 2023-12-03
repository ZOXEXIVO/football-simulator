pub mod defenders;
pub mod fordwarders;
pub mod goalkeeper;
pub mod midfielders;

pub use defenders::*;
pub use fordwarders::*;
pub use goalkeeper::*;
pub use midfielders::*;
use nalgebra::Vector3;
use crate::r#match::{GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent};

use crate::{PlayerFieldPositionGroup};

pub trait VelocityStrategy {
    fn calculate_velocity(
        &self,
        context: &mut MatchContext,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Vector3<f32>;
}

impl VelocityStrategy for PlayerFieldPositionGroup {
    fn calculate_velocity(&self, context: &mut MatchContext, player: &MatchPlayer, result: &mut Vec<PlayerUpdateEvent>, objects_positions: &MatchObjectsPositions) -> Vector3<f32> {
        match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::detect_velocity(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::detect_velocity(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::detect_velocity(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::detect_velocity(
                context,
                player,
                result,
                objects_positions
            ),
        }
    }
}