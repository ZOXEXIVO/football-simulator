use crate::core::context::SimulationContext;
use crate::generators::Generator;

use crate::continent::Continent;
use crate::people::{Player, Staff};
pub use rayon::prelude::*;
use std::sync::Mutex;

pub struct SimulatorData {
    pub continents: Vec<Continent>,

    pub free_players_pool: Mutex<Vec<Player>>,
    pub free_staffs_pool: Mutex<Vec<Staff>>,
}

#[derive(Default)]
pub struct FootballSimulator {
    data: Option<SimulatorData>,
}

impl FootballSimulator {
    pub fn new() -> Self {
        Self { data: None }
    }

    pub fn generate(&mut self) {
        self.data = Some(SimulatorData::generate());
    }

    pub fn items_count(&self) -> usize {
        self.data
            .as_ref()
            .unwrap()
            .continents
            .iter()
            .map(|continent| continent.items_count())
            .sum()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let unwrapped_data = self.data.as_mut().unwrap();

        unwrapped_data
            .continents
            .par_iter_mut()
            .for_each(|continent| {
                let mut cloned_context = context.clone();
                continent.simulate(&mut cloned_context);
            });

        context.next_date();
    }
}
