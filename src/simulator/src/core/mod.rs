mod simulator;
pub use simulator::FootballSimulator;
pub use simulator::SimulatorData;

pub mod context;
pub use context::SimulationContext;

mod events;
pub use events::EventType;

pub mod visitor;
pub use visitor::Visitor;

pub use crate::utils::*;