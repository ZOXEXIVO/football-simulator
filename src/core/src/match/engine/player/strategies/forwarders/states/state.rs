use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum ForwardState {
    Standing,            // Standing still
    Passing,             // Passing the ball
    Dribbling,           // Dribbling the ball past opponents
    Shooting,            // Taking a shot on goal
    Heading,             // Heading the ball, often during crosses or set pieces
    HoldingUpPlay,       // Holding up the ball to allow teammates to join the attack
    RunningInBehind,     // Making a run behind the defense to receive a pass
    Pressing,            // Pressing defenders to force a mistake or regain possession
    Finishing,           // Attempting to score from a close range
    CreatingSpace,       // Creating space for teammates by pulling defenders away
    CrossReceiving,      // Positioning to receive a cross
    OffsideTrapBreaking, // Trying to beat the offside trap by timing runs
    Tackling,            // Tackling the ball
    Assisting,           // Providing an assist by passing or crossing to a teammate
}

impl Display for ForwardState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ForwardState::Standing => write!(f, "Standing"),
            ForwardState::Dribbling => write!(f, "Dribbling"),
            ForwardState::Shooting => write!(f, "Shooting"),
            ForwardState::Heading => write!(f, "Heading"),
            ForwardState::HoldingUpPlay => write!(f, "Holding Up Play"),
            ForwardState::RunningInBehind => write!(f, "Running In Behind"),
            ForwardState::Pressing => write!(f, "Pressing"),
            ForwardState::Finishing => write!(f, "Finishing"),
            ForwardState::CreatingSpace => write!(f, "Creating Space"),
            ForwardState::CrossReceiving => write!(f, "Cross Receiving"),
            ForwardState::OffsideTrapBreaking => write!(f, "Offside Trap Breaking"),
            ForwardState::Assisting => write!(f, "Assisting"),
            ForwardState::Passing => write!(f, "Passing"),
            ForwardState::Tackling => write!(f, "Tackling"),
        }
    }
}
