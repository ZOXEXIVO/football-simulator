mod states;

use std::ops::Deref;
use crate::r#match::position::VectorExtensions;
use crate::r#match::strategies::goalkeepers::states::{
    GoalkeeperPassingState, GoalkeeperReturningState, GoalkeeperRunningState,
    GoalkeeperShootingState, GoalkeeperStandingState, GoalkeeperTacklingState,
    GoalkeeperWalkingState,
};
use crate::r#match::{BallContext, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerTickContext, PlayerUpdateEvent, StateChangeResult};

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn calculate(
        in_state_time: u64,
        player: &MatchPlayer,
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

pub struct GameSituationInput<'p> {
    objects_positions: &'p MatchObjectsPositions
}

impl<'p> GameSituationInput<'p> {
    pub fn from_contexts(context: &MatchContext,
                         player: &MatchPlayer,
                         tick_context: &'p GameTickContext) -> Self {
        GameSituationInput {
            objects_positions: &tick_context.objects_positions
        }
    }
}

impl Deref for GameSituationInput<'_> {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &[]
    }
}

