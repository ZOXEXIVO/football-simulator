use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum DefenderState {
    Standing,      // Standing
    Resting,       // Resting after an attack
    Passing,       // Passing the ball
    Blocking,      // Blocking a shot or pass
    Intercepting,  // Intercepting a pass
    Marking,       // Marking an attacker
    Clearing,      // Clearing the ball from the danger zone
    Heading,       // Heading the ball, often during corners or crosses
    SlidingTackle, // Sliding tackle
    Pressing,      // Pressing the opponent
    TrackingBack,  // Tracking back to defense after an attack
    HoldingLine,   // Holding the defensive line
    OffsideTrap,   // Setting up an offside trap,
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
        }
    }
}
