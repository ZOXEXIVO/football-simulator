mod common;
mod defenders;
mod forwarders;
mod goalkeepers;
mod midfielders;

pub use common::*;
pub use defenders::*;
pub use forwarders::*;
pub use goalkeepers::*;
pub use midfielders::*;
use std::hash::Hash;

use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext};
use nalgebra::Vector3;

use crate::r#match::player::events::PlayerUpdateEvent;
use crate::PlayerFieldPositionGroup;
use crate::r#match::player::state::PlayerState;

type StateHandler = fn(
    in_state_time: u64,
    player: &mut MatchPlayer,
    context: &mut MatchContext,
    tick_context: &GameTickContext,
    player_context: PlayerTickContext,
    result: &mut Vec<PlayerUpdateEvent>,
) -> StateChangeResult;

pub trait StateStrategy {
    fn process(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult;
}

impl StateStrategy for PlayerFieldPositionGroup {
    fn process(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match self {
            PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::process,
            PlayerFieldPositionGroup::Defender => DefenderStrategies::process,
            PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::process,
            PlayerFieldPositionGroup::Forward => ForwardStrategies::process,
        };

        state_handler(
            in_state_time,
            player,
            context,
            tick_context,
            player_context,
            result,
        )
    }
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
