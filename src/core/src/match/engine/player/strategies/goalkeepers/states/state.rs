use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum GoalkeeperState {
    Jumping,             // Jumping
    Diving,              // Diving to save the ball
    Catching,            // Catching the ball with hands
    Punching,            // Punching the ball away
    Kicking,             // Kicking the ball
    HoldingBall,         // Holding the ball in hands
    Throwing,            // Throwing the ball with hands
    PickingUpBall,       // Picking up the ball from the ground
    Distributing,        // Distributing the ball after catching it
    ComingOut,           // Coming out of the goal to intercept
    ReturningToGoal,     // Returning to the goal after coming out
    Sweeping,            // Acting as a "sweeper", clearing the ball in front of the defense line
    CommandingDefense,   // Commanding defenders, directing their positions
    AwaitingAction,      // Awaiting action, watching the play
    UnderPressure,       // Under pressure from opponents
    Injured,             // Injured
    Resting,             // Resting, not involved in active play
    PreparingForSave,    // Preparing to make a save
    PenaltySave,         // Saving a penalty
}

impl Display for GoalkeeperState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GoalkeeperState::Jumping => write!(f, "Jumping"),
            GoalkeeperState::Diving => write!(f, "Diving"),
            GoalkeeperState::Catching => write!(f, "Catching"),
            GoalkeeperState::Punching => write!(f, "Punching"),
            GoalkeeperState::Kicking => write!(f, "Kicking"),
            GoalkeeperState::HoldingBall => write!(f, "Holding Ball"),
            GoalkeeperState::Throwing => write!(f, "Throwing"),
            GoalkeeperState::PickingUpBall => write!(f, "Picking Up Ball"),
            GoalkeeperState::Distributing => write!(f, "Distributing"),
            GoalkeeperState::ComingOut => write!(f, "Coming Out"),
            GoalkeeperState::ReturningToGoal => write!(f, "Returning to Goal"),
            GoalkeeperState::Sweeping => write!(f, "Sweeping"),
            GoalkeeperState::CommandingDefense => write!(f, "Commanding Defense"),
            GoalkeeperState::AwaitingAction => write!(f, "Awaiting Action"),
            GoalkeeperState::UnderPressure => write!(f, "Under Pressure"),
            GoalkeeperState::Injured => write!(f, "Injured"),
            GoalkeeperState::Resting => write!(f, "Resting"),
            GoalkeeperState::PreparingForSave => write!(f, "Preparing for Save"),
            GoalkeeperState::PenaltySave => write!(f, "Penalty Save"),
        }
    }
}