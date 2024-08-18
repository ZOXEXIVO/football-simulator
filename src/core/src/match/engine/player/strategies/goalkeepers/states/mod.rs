pub mod passing;
pub mod returning;
pub mod running;
pub mod shooting;
pub mod standing;
pub mod tackling;
pub mod walking;

pub use passing::*;
pub use returning::*;
pub use running::*;
pub use shooting::*;
pub use standing::*;
pub use tackling::*;
pub use walking::*;

pub enum GoalkeeperState {
    Standing,
    Walking,
    Running,
    Jumping,
    Diving,
    Catching,
    Punching,
    Kicking,
    Passing,
    HoldingBall,
    Throwing,
    PickingUpBall,
    Distributing,
    ComingOut,
    ReturningToGoal,
    Sweeping,
    CommandingDefense,
    AwaitingAction,
    UnderPressure,
    Injured,
    Resting,
    PreparingForSave,
    PenaltySave,
}