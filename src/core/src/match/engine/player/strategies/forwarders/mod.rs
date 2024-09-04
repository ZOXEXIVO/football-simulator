pub mod states;

use crate::r#match::forwarders::states::{
    ForwardAssistingState, ForwardCreatingSpaceState, ForwardCrossReceivingState,
    ForwardDribblingState, ForwardFinishingState, ForwardHeadingState, ForwardHeadingUpPlayState,
    ForwardOffsideTrapBreakingState, ForwardPressingState, ForwardRunningInBehindState,
    ForwardState,
};
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::strategies::forwarders::states::{
    ForwardPassingState, ForwardShootingState, ForwardStandingState, ForwardTacklingState,
};
use crate::r#match::strategies::StateHandler;
use crate::r#match::{
    GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerTickContext,
    StateChangeResult,
};

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
            PlayerState::Forward(ForwardState::Standing) => ForwardStandingState::process,
            PlayerState::Forward(ForwardState::Passing) => ForwardPassingState::process,
            PlayerState::Forward(ForwardState::Dribbling) => ForwardDribblingState::process,
            PlayerState::Forward(ForwardState::Shooting) => ForwardShootingState::process,
            PlayerState::Forward(ForwardState::Heading) => ForwardHeadingState::process,
            PlayerState::Forward(ForwardState::HoldingUpPlay) => ForwardHeadingUpPlayState::process,
            PlayerState::Forward(ForwardState::RunningInBehind) => {
                ForwardRunningInBehindState::process
            }
            PlayerState::Forward(ForwardState::Pressing) => ForwardPressingState::process,
            PlayerState::Forward(ForwardState::Finishing) => ForwardFinishingState::process,
            PlayerState::Forward(ForwardState::CreatingSpace) => ForwardCreatingSpaceState::process,
            PlayerState::Forward(ForwardState::CrossReceiving) => {
                ForwardCrossReceivingState::process
            }
            PlayerState::Forward(ForwardState::OffsideTrapBreaking) => {
                ForwardOffsideTrapBreakingState::process
            }
            PlayerState::Forward(ForwardState::Tackling) => ForwardTacklingState::process,
            PlayerState::Forward(ForwardState::Assisting) => ForwardAssistingState::process,
            _ => {
                return StateChangeResult::none();
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
