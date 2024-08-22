use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum GoalkeeperState {
    Standing,            // Standing
    Resting,             // Resting
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
    Tackling,            // Tackling the ball
    Sweeping,            // Acting as a "sweeper", clearing the ball in front of the defense line
    UnderPressure,       // Under pressure from opponents
    Shooting,            // Shoot to goal
    PreparingForSave,    // Preparing to make a save
    PenaltySave,         // Saving a penalty,
    Walking              // Walking
}

impl Display for GoalkeeperState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            GoalkeeperState::Standing => write!(f, "Standing"),
            GoalkeeperState::Resting => write!(f, "Resting"),
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
            GoalkeeperState::UnderPressure => write!(f, "Under Pressure"),
            GoalkeeperState::Shooting => write!(f, "Try shoot to goal"),
            GoalkeeperState::PreparingForSave => write!(f, "Preparing for Save"),
            GoalkeeperState::PenaltySave => write!(f, "Penalty Save"),
            GoalkeeperState::Tackling => write!(f, "Tackling"),
            GoalkeeperState::Walking => write!(f, "Walking"),
        }
    }
}