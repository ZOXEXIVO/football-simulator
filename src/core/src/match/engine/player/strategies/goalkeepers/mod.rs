mod decision;
mod states;

use crate::r#match::strategies::goalkeepers::states::{
    GoalkeeperPassingState, GoalkeeperReturningState, GoalkeeperRunningState,
    GoalkeeperShootingState, GoalkeeperStandingState, GoalkeeperTacklingState,
    GoalkeeperWalkingState,
};
use crate::r#match::{
    BallContext, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState,
    PlayerTickContext, StateChangeResult,
};
use itertools::Itertools;
use std::ops::Deref;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::StateHandler;

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn calculate(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match player.state {
            PlayerState::Standing => GoalkeeperStandingState::process,
            PlayerState::Walking => GoalkeeperWalkingState::process,
            PlayerState::Running => GoalkeeperRunningState::process,
            PlayerState::Tackling => GoalkeeperTacklingState::process,
            PlayerState::Shooting => GoalkeeperShootingState::process,
            PlayerState::Passing => GoalkeeperPassingState::process,
            PlayerState::Returning => GoalkeeperReturningState::process
        };

        state_handler(in_state_time,
                      player,
                      context,
                      tick_context,
                      player_context,
                      result)
    }
}
