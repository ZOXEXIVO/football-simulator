pub mod decision;
pub mod states;
use crate::r#match::defenders::states::{
    DefenderBlockingState, DefenderClearingState, DefenderHeadingState, DefenderHoldingLineState,
    DefenderInterceptingState, DefenderMarkingState, DefenderOffsideTrapState,
    DefenderPressingState, DefenderRestingState, DefenderSlidingTackleState, DefenderStandingState,
    DefenderState, DefenderTrackingBackState,
};
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::strategies::defenders::states::DefenderPassingState;
use crate::r#match::strategies::StateHandler;
use crate::r#match::{
    BallState, GameState, GameTickContext, MatchContext, MatchPlayer, PlayerTickContext,
    StateChangeResult,
};

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match player.state {
            PlayerState::Defender(DefenderState::Standing) => DefenderStandingState::process,
            PlayerState::Defender(DefenderState::Resting) => DefenderRestingState::process,
            PlayerState::Defender(DefenderState::Passing) => DefenderPassingState::process,
            PlayerState::Defender(DefenderState::Blocking) => DefenderBlockingState::process,
            PlayerState::Defender(DefenderState::Intercepting) => {
                DefenderInterceptingState::process
            }
            PlayerState::Defender(DefenderState::Marking) => DefenderMarkingState::process,
            PlayerState::Defender(DefenderState::Clearing) => DefenderClearingState::process,
            PlayerState::Defender(DefenderState::Heading) => DefenderHeadingState::process,
            PlayerState::Defender(DefenderState::SlidingTackle) => {
                DefenderSlidingTackleState::process
            }
            PlayerState::Defender(DefenderState::Pressing) => DefenderPressingState::process,
            PlayerState::Defender(DefenderState::TrackingBack) => {
                DefenderTrackingBackState::process
            }
            PlayerState::Defender(DefenderState::HoldingLine) => DefenderHoldingLineState::process,
            PlayerState::Defender(DefenderState::OffsideTrap) => DefenderOffsideTrapState::process,
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

    fn is_on_defending_half(player: &MatchPlayer, state: &GameState) -> bool {
        match state.ball_state {
            Some(ball_state) => ball_state == BallState::HomeSide && player.is_home,
            None => false,
        }
    }
}

enum DefenderBehavior {
    Defend,
    Support,
    Idle,
}
