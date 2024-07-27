pub mod states;

use crate::r#match::strategies::midfielders::states::{
    MidfielderPassingState, MidfielderReturningState, MidfielderRunningState,
    MidfielderShootingState, MidfielderStandingState, MidfielderTacklingState,
    MidfielderWalkingState,
};
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerState,
    PlayerTickContext, StateChangeResult,
};
use crate::r#match::player::events::PlayerUpdateEvent;

pub struct MidfielderStrategies {}

impl MidfielderStrategies {
    pub fn calculate(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        match player.state {
            PlayerState::Standing => MidfielderStandingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Walking => MidfielderWalkingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Running => MidfielderRunningState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Tackling => MidfielderTacklingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Shooting => MidfielderShootingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Passing => MidfielderPassingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Returning => MidfielderReturningState::process(
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
