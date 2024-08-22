pub mod states;

use crate::r#match::midfielders::states::{
    MidfielderAttackSupportingState, MidfielderCrossingState, MidfielderDistanceShootingState,
    MidfielderDistributingState, MidfielderHoldingPossessionState, MidfielderLongPassingState,
    MidfielderPressingState, MidfielderShortPassingState, MidfielderStandingState, MidfielderState,
    MidfielderSwitchingPlayState, MidfielderTrackingRunnerState,
};
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::strategies::midfielders::states::MidfielderTacklingState;
use crate::r#match::strategies::StateHandler;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult,
};

pub struct MidfielderStrategies {}

impl MidfielderStrategies {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match player.state {
            PlayerState::Midfielder(MidfielderState::Standing) => MidfielderStandingState::process,
            PlayerState::Midfielder(MidfielderState::Distributing) => {
                MidfielderDistributingState::process
            }
            PlayerState::Midfielder(MidfielderState::SupportingAttack) => {
                MidfielderAttackSupportingState::process
            }
            PlayerState::Midfielder(MidfielderState::HoldingPossession) => {
                MidfielderHoldingPossessionState::process
            }
            PlayerState::Midfielder(MidfielderState::SwitchingPlay) => {
                MidfielderSwitchingPlayState::process
            }
            PlayerState::Midfielder(MidfielderState::Crossing) => MidfielderCrossingState::process,
            PlayerState::Midfielder(MidfielderState::LongPassing) => {
                MidfielderLongPassingState::process
            }
            PlayerState::Midfielder(MidfielderState::ShortPassing) => {
                MidfielderShortPassingState::process
            }
            PlayerState::Midfielder(MidfielderState::DistanceShooting) => {
                MidfielderDistanceShootingState::process
            }
            PlayerState::Midfielder(MidfielderState::Pressing) => MidfielderPressingState::process,
            PlayerState::Midfielder(MidfielderState::TrackingRunner) => {
                MidfielderTrackingRunnerState::process
            }
            PlayerState::Midfielder(MidfielderState::Tackling) => MidfielderTacklingState::process,
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
