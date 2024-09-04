pub mod decision;
pub mod states;
use crate::r#match::defenders::states::{
    DefenderBlockingState, DefenderClearingState, DefenderHeadingState, DefenderHoldingLineState,
    DefenderInterceptingState, DefenderMarkingState, DefenderOffsideTrapState,
    DefenderPassingState, DefenderPressingState, DefenderRestingState, DefenderSlidingTackleState,
    DefenderStandingState, DefenderState, DefenderTrackingBackState,
};
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult,
    StateProcessor,
};

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn process(
        state: DefenderState,
        state_processor: &mut StateProcessor,
    ) -> StateChangeResult {
        match state {
            DefenderState::Standing => state_processor.process(DefenderStandingState::default()),
            DefenderState::Resting => state_processor.process(DefenderRestingState::default()),
            DefenderState::Passing => state_processor.process(DefenderPassingState::default()),
            DefenderState::Blocking => state_processor.process(DefenderBlockingState::default()),
            DefenderState::Intercepting => {
                state_processor.process(DefenderInterceptingState::default())
            }
            DefenderState::Marking => state_processor.process(DefenderMarkingState::default()),
            DefenderState::Clearing => state_processor.process(DefenderClearingState::default()),
            DefenderState::Heading => state_processor.process(DefenderHeadingState::default()),
            DefenderState::SlidingTackle => {
                state_processor.process(DefenderSlidingTackleState::default())
            }
            DefenderState::Pressing => state_processor.process(DefenderPressingState::default()),
            DefenderState::TrackingBack => {
                state_processor.process(DefenderTrackingBackState::default())
            }
            DefenderState::HoldingLine => {
                state_processor.process(DefenderHoldingLineState::default())
            }
            DefenderState::OffsideTrap => {
                state_processor.process(DefenderOffsideTrapState::default())
            }
        }
    }
}
