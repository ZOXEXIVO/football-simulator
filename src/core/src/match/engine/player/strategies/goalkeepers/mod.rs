mod decision;
mod states;

use crate::r#match::position::VectorExtensions;
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
        match player.state {
            PlayerState::Standing => GoalkeeperStandingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Walking => GoalkeeperWalkingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Running => GoalkeeperRunningState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Tackling => GoalkeeperTacklingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Shooting => GoalkeeperShootingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Passing => GoalkeeperPassingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Returning => GoalkeeperReturningState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
        }
    }
}
