use crate::staff::staff::Staff;
use crate::player::player::Player;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::country::Country;

pub use rayon::prelude::*;

pub struct SimulatorData {
    pub countries: Vec<Country>,

    pub free_players: Vec<Player>,
    pub free_staff: Vec<Staff>
}

pub struct FootballSimulator {
    data: Option<SimulatorData>,
}

impl FootballSimulator {
    pub fn new() -> Self {
        Self {
            data: None
        }
    }

    pub fn generate(&mut self){
        self.data = Some(SimulatorData::generate());
    }

    pub fn items_count(&self) -> usize {
        return self
            .data            
            .as_ref()
            .unwrap()
            .countries
            .par_iter()
            .map(|country| country.items_count())
            .sum();
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let unwraped_data = self.data.as_mut().unwrap();

        unwraped_data.countries.par_iter_mut().for_each(|country|{
             country.simulate(&mut context.clone());
        });

        context.next_date();
    }
}
