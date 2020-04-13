use crate::continent::Continent;
use crate::simulator::context::GlobalContext;
use crate::simulator::SimulationContext;
use chrono::{Duration, NaiveDateTime};
pub use rayon::prelude::*;

#[derive()]
pub struct SimulatorData {
    pub continents: Vec<Continent>,

    pub date: NaiveDateTime,
}

impl SimulatorData {
    pub fn next_date(&mut self) {
        self.date += Duration::hours(1);
    }
}

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn new() -> Self {
        FootballSimulator {}
    }

    pub fn simulate(&mut self, data: &mut SimulatorData) {
        let mut global_ctx = GlobalContext::new(SimulationContext::new(data.date));

        let continent_ctx = global_ctx.with_continent();

        for continent in &mut data.continents {
            continent.simulate(continent_ctx);
        }

        data.next_date();
    }
}
