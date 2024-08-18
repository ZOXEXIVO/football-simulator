pub mod states;

use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::forwarders::states::{
    ForwardPassingState, ForwardReturningState, ForwardRunningState, ForwardShootingState,
    ForwardStandingState, ForwardTacklingState, ForwardWalkingState,
};
use crate::r#match::strategies::StateHandler;
use crate::r#match::{
    GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer,
    PlayerTickContext, StateChangeResult,
};
use crate::r#match::player::state::PlayerState;

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match player.state {
            PlayerState::Standing => ForwardStandingState::process,
            PlayerState::Walking => ForwardWalkingState::process,
            PlayerState::Running => ForwardRunningState::process,
            PlayerState::Tackling => ForwardTacklingState::process,
            PlayerState::Shooting => ForwardShootingState::process,
            PlayerState::Passing => ForwardPassingState::process,
            PlayerState::Returning => ForwardReturningState::process,
            _ => {
                unimplemented!()
            }
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
