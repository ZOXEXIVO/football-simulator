mod simulator;
pub use simulator::FootballSimulator;

mod context;
pub use context::SimulationContext;
pub use context::SimulationEvent;
pub use context::EventType;

pub mod visitor;
pub use visitor::Visitor;
