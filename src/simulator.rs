use crate::continent::{Continent, ContinentContext};
use crate::core::context::{GlobalContext, SimulationContext};

use chrono::NaiveDateTime;
pub use rayon::prelude::*;

#[derive()]
pub struct SimulatorData {
    pub continents: Vec<Continent>,

    pub date: NaiveDateTime,
}

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn new() -> Self {
        FootballSimulator {}
    }

    pub fn simulate(&mut self, data: &mut SimulatorData) {
        let mut simulation_ctx = SimulationContext::new(data.date);

        let mut ctx = GlobalContext::new(&mut simulation_ctx);

        let mut continent_ctx = ContinentContext::new();

        let ctx = &mut ctx.with_continent(&mut continent_ctx);

        for continent in &mut data.continents {
            continent.simulate(ctx);
        }

        //ctx.next_date();
    }
}
