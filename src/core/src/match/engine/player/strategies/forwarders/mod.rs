pub mod states;

use crate::r#match::strategies::forwarders::states::{
    ForwardPassingState, ForwardReturningState, ForwardRunningState, ForwardShootingState,
    ForwardStandingState, ForwardTacklingState, ForwardWalkingState,
};
use crate::r#match::{
    GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState,
    PlayerTickContext, PlayerUpdateEvent, StateChangeResult, SteeringBehavior,
};

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn calculate(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        match player.state {
            PlayerState::Standing => ForwardStandingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Walking => ForwardWalkingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Running => ForwardRunningState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Tackling => ForwardTacklingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Shooting => ForwardShootingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Passing => ForwardPassingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Returning => ForwardReturningState::process(
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
