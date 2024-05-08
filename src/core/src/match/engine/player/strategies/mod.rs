mod common;
mod defenders;
mod forwarders;
mod goalkeepers;
mod loader;
mod midfielders;

pub use common::*;
pub use defenders::*;
pub use forwarders::*;
pub use goalkeepers::*;
pub use midfielders::*;

use crate::r#match::{
    GameState, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState,
    PlayerTickContext, PlayerUpdateEvent,
};
use nalgebra::Vector3;

use crate::PlayerFieldPositionGroup;

pub trait StateStrategy {
    fn calculate(
        &self,
        in_state_time: u64,
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
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
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::calculate(
                in_state_time,
                player,
                context,
                tick_context,
                player_context,
                result,
            ),
            PlayerFieldPositionGroup::Defender => DefenderStrategies::calculate(
                in_state_time,
                player,
                context,
                tick_context,
                player_context,
                result,
            ),
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::calculate(
                in_state_time,
                player,
                context,
                tick_context,
                player_context,
                result,
            ),
            PlayerFieldPositionGroup::Forward => ForwardStrategies::calculate(
                in_state_time,
                player,
                context,
                tick_context,
                player_context,
                result,
            ),
        }
    }
}
