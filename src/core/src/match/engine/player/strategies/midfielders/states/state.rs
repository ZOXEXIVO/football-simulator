use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum MidfielderState {
    Standing,             // Standing still
    Distributing,         // Distributing the ball to teammates
    SupportingAttack,     // Supporting the attack, moving forward
    HoldingPossession,    // Holding possession of the ball, maintaining control
    SwitchingPlay,        // Switching the play to the other side of the field
    Crossing,             // Delivering a cross into the box
    LongPassing,          // Executing a long pass
    ShortPassing,         // Executing a short pass
    DistanceShooting,     // Taking a shot from a long distance
    Pressing,             // Pressing the opponent to regain possession
    TrackingRunner,       // Tracking a runner to prevent a break
    Tackling,             // Tackling to win the ball
}

impl Display for MidfielderState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            MidfielderState::Standing => write!(f, "Standing"),
            MidfielderState::Distributing => write!(f, "Distributing"),
            MidfielderState::SupportingAttack => write!(f, "Supporting Attack"),
            MidfielderState::HoldingPossession => write!(f, "Holding Possession"),
            MidfielderState::SwitchingPlay => write!(f, "Switching Play"),
            MidfielderState::Crossing => write!(f, "Crossing"),
            MidfielderState::LongPassing => write!(f, "Long Passing"),
            MidfielderState::ShortPassing => write!(f, "Short Passing"),
            MidfielderState::Distributing => write!(f, "Shooting from Distance"),
            MidfielderState::Pressing => write!(f, "Pressing"),
            MidfielderState::TrackingRunner => write!(f, "Tracking Runner"),
            MidfielderState::Tackling => write!(f, "Tackling"),
            MidfielderState::DistanceShooting => write!(f, "DistanceShooting"),
        }
    }
}