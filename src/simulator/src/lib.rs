mod core;
pub use crate::core::FootballSimulator;
pub use crate::core::SimulationContext;

mod models;
pub use crate::models::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
