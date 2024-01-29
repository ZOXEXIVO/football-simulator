mod defenders;
mod forwarders;
mod goalkeepers;
mod midfielders;

pub use defenders::*;
pub use forwarders::*;
pub use goalkeepers::*;
pub use midfielders::*;

use crate::r#match::{
    GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};
use nalgebra::Vector3;

use crate::PlayerFieldPositionGroup;

pub trait StateStrategy {
    fn calculate(
        &self,
        in_state_time: u64,
        context: &mut MatchContext,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult;
}

pub struct StateChangeResult {
    pub state: Option<PlayerState>,
    pub velocity: Option<Vector3<f32>>,
}

impl StateChangeResult {
    pub fn with(state: PlayerState, velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: Some(velocity),
        }
    }

    pub fn none() -> Self {
        StateChangeResult {
            state: None,
            velocity: None,
        }
    }

    pub fn with_state(state: PlayerState) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: None,
        }
    }

    pub fn with_velocity(velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: None,
            velocity: Some(velocity),
        }
    }
}

impl StateStrategy for PlayerFieldPositionGroup {
    fn calculate(
        &self,
        in_state_time: u64,
        context: &mut MatchContext,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult {
        match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::calculate(
                in_state_time,
                context,
                player,
                result,
                objects_positions,
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::calculate(
                in_state_time,
                context,
                player,
                result,
                objects_positions,
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::calculate(
                in_state_time,
                context,
                player,
                result,
                objects_positions,
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::calculate(
                in_state_time,
                context,
                player,
                result,
                objects_positions,
            ),
        }
    }
}

pub struct BallMetadata {
    is_ball_heading_towards_goal: bool,
    ball_distance: f32,
}
