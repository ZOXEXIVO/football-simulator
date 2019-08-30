mod core;
pub use crate::core::FootballSimulator;
pub use crate::core::SimulationContext;

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
