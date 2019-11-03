mod simulator;
pub use self::simulator::{FootballSimulator, SimulatorData};

pub mod context;
pub use context::SimulationContext;

mod events;
pub use events::EventType;

pub use crate::utils::*;