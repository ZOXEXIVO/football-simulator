use crate::core::context::SimulationContext;
use crate::country::Country;
use crate::generators::Generator;

use crate::continent::Continent;
use crate::{Player, Staff};
pub use rayon::prelude::*;

pub struct SimulatorData {
    pub continents: Vec<Continent>,

    pub free_players: Vec<Player>,
    pub free_staff: Vec<Staff>,
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
