use crate::r#match::defenders::states::{DefenderBlockingState, DefenderClearingState, DefenderHeadingState, DefenderHoldingLineState, DefenderInterceptingState, DefenderMarkingState, DefenderOffsideTrapState, DefenderPassingState, DefenderPressingState, DefenderRestingState, DefenderReturningState, DefenderSlidingTackleState, DefenderStandingState, DefenderTacklingState, DefenderTrackingBackState, DefenderWalkingState};
use crate::r#match::{StateProcessingResult, StateProcessor};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum DefenderState {
    Standing,      // Standing
    Resting,       // Resting after an attack
    Passing,       // Passing the ball
    Running,       // Running in the direction of the ball
    Blocking,      // Blocking a shot or pass
    Intercepting,  // Intercepting a pass
    Marking,       // Marking an attacker
    Clearing,      // Clearing the ball from the danger zone
    Heading,       // Heading the ball, often during corners or crosses
    SlidingTackle, // Sliding tackle
    Tackling,      // Tackling the ball
    Pressing,      // Pressing the opponent
    TrackingBack,  // Tracking back to defense after an attack
    HoldingLine,   // Holding the defensive line
    OffsideTrap,   // Setting up an offside trap,
    Returning,     // Returning the ball,
    Walking,       // Walking around
}

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn process(state: DefenderState, state_processor: StateProcessor) -> StateProcessingResult {
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
            DefenderState::Running => state_processor.process(DefenderOffsideTrapState::default()),
            DefenderState::Returning => state_processor.process(DefenderReturningState::default()),
            DefenderState::Walking => state_processor.process(DefenderWalkingState::default()),
            DefenderState::Tackling => state_processor.process(DefenderTacklingState::default()),
        }
    }
}

impl Display for DefenderState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            DefenderState::Standing => write!(f, "Standing"),
            DefenderState::Resting => write!(f, "Resting"),
            DefenderState::Passing => write!(f, "Passing"),
            DefenderState::Blocking => write!(f, "Blocking"),
            DefenderState::Intercepting => write!(f, "Intercepting"),
            DefenderState::Marking => write!(f, "Marking"),
            DefenderState::Clearing => write!(f, "Clearing"),
            DefenderState::Heading => write!(f, "Heading"),
            DefenderState::SlidingTackle => write!(f, "Sliding Tackle"),
            DefenderState::Pressing => write!(f, "Pressing"),
            DefenderState::TrackingBack => write!(f, "Tracking Back"),
            DefenderState::HoldingLine => write!(f, "Holding Line"),
            DefenderState::OffsideTrap => write!(f, "Offside Trap"),
            DefenderState::Running => write!(f, "Running"),
            DefenderState::Returning => write!(f, "Returning"),
            DefenderState::Walking => write!(f, "Walking"),
            DefenderState::Tackling => write!(f, "Tackling"),
        }
    }
}
