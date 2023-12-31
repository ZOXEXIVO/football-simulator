mod goalkeepers;
mod defenders;
mod midfielders;
mod forwarders;

pub use goalkeepers::*;
pub use defenders::*;
pub use midfielders::*;
pub use forwarders::*;

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
    ) -> Option<Vector3<f32>>;
}

impl VelocityStrategy for PlayerFieldPositionGroup {
    fn calculate_velocity(&self, context: &mut MatchContext, player: &MatchPlayer, result: &mut Vec<PlayerUpdateEvent>, objects_positions: &MatchObjectsPositions) -> Option<Vector3<f32>> {
        match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::calculate_velocity(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::calculate_velocity(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::calculate_velocity(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::calculate_velocity(
                context,
                player,
                result,
                objects_positions
            ),
        }
    }
}