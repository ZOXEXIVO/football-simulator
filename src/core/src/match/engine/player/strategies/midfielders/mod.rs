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
use crate::r#match::strategies::StateHandler;

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
        let state_handler: StateHandler = match player.state {
            PlayerState::Standing => MidfielderStandingState::process,
            PlayerState::Walking => MidfielderWalkingState::process,
            PlayerState::Running => MidfielderRunningState::process,
            PlayerState::Tackling => MidfielderTacklingState::process,
            PlayerState::Shooting => MidfielderShootingState::process,
            PlayerState::Passing => MidfielderPassingState::process,
            PlayerState::Returning => MidfielderReturningState::process
        };

        state_handler(in_state_time,
                      player,
                      context,
                      tick_context,
                      player_context,
                      result)
    }
}
