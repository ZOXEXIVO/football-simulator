mod goalkeepers;
mod defenders;
mod midfielders;
mod forwarders;

pub use goalkeepers::*;
pub use defenders::*;
pub use midfielders::*;
pub use forwarders::*;

use nalgebra::Vector3;
use crate::r#match::{GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

use crate::{PlayerFieldPositionGroup};

pub trait StateStrategy {
    fn calculate(
        &self,
        context: &mut MatchContext,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult;
}

pub struct StateChangeResult {
    pub state: Option<PlayerState>,
    pub velocity: Option<Vector3<f32>>
}

impl StateChangeResult {
    pub fn with(state: PlayerState, velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: Some(velocity)
        }
    }

    pub fn none() -> Self {
        StateChangeResult {
            state: None,
            velocity: None
        }
    }

    pub fn with_state(state: PlayerState) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: None
        }
    }

    pub fn with_velocity(velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: None,
            velocity: Some(velocity)
        }
    }
}

impl StateStrategy for PlayerFieldPositionGroup {
    fn calculate(&self, context: &mut MatchContext, player: &MatchPlayer, result: &mut Vec<PlayerUpdateEvent>, objects_positions: &MatchObjectsPositions) -> StateChangeResult {
        match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::calculate(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::calculate(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::calculate(
                context,
                player,
                result,
                objects_positions
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::calculate(
                context,
                player,
                result,
                objects_positions
            )
        }
    }
}