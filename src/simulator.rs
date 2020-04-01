use crate::continent::{Continent, ContinentContext};
use crate::core::context::SimulationContext;
use crate::people::{Player, Staff};
pub use rayon::prelude::*;
use std::sync::Mutex;

pub struct SimulatorData {
    pub continents: Vec<Continent>,

    pub free_players_pool: Mutex<Vec<Player>>,
    pub free_staffs_pool: Mutex<Vec<Staff>>,
}

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn new() -> Self {
        FootballSimulator {}
    }

    pub fn simulate(&mut self, data: &mut SimulatorData, context: &mut SimulationContext) {
        data.continents.iter_mut().for_each(|continent| {
            let mut context = ContinentContext::new(context);
            continent.simulate(&mut context);
        });

        context.next_date();
    }
}
