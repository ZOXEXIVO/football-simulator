mod common;
mod defenders;
mod forwarders;
mod goalkeepers;
mod midfielders;

use std::hash::Hash;
pub use common::*;
pub use defenders::*;
pub use forwarders::*;
pub use goalkeepers::*;
pub use midfielders::*;

use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerState,
    PlayerTickContext,
};
use nalgebra::Vector3;

use crate::PlayerFieldPositionGroup;
use crate::r#match::player::events::PlayerUpdateEvent;

pub trait StataHandler {
    fn fast_path() -> Option<StateChangeResult>;
    fn slow_path() -> Option<StateChangeResult>;
}

pub trait StateStrategy {
    fn calculate(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
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

type StateHandler = fn(
    in_state_time: u64,
    player: &mut MatchPlayer,
    context: &mut MatchContext,
    tick_context: &GameTickContext,
    player_context: PlayerTickContext,
    result: &mut Vec<PlayerUpdateEvent>,
) -> StateChangeResult;

impl StateStrategy for PlayerFieldPositionGroup {
    fn calculate(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::calculate,
            PlayerFieldPositionGroup::Defender => DefenderStrategies::calculate,
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::calculate,
            PlayerFieldPositionGroup::Forward => ForwardStrategies::calculate,
        };

        state_handler(in_state_time,
                      player,
                      context,
                      tick_context,
                      player_context,
                      result)
    }
}
