mod core;
pub use crate::core::FootballSimulator;
pub use crate::core::SimulationContext;
pub use crate::core::EventType;

mod models;
mod generators;

pub use generators::*;

mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
