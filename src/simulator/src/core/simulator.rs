use crate::staff::staff::Staff;
use crate::player::player::Player;
use crate::utils::ParallelUtils;

extern crate crossbeam;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::country::Country;

pub struct SimulatorData {
    pub countries: Vec<Country>,

    pub free_players: Vec<Player>,
    pub free_staff: Vec<Staff>
}

pub struct FootballSimulator {
    cpu_count: usize,
    data: Option<SimulatorData>,
}

impl FootballSimulator {
    pub fn new(cpu_count: usize) -> Self {
        println!("sumulator init with {} cores", cpu_count);

        Self {
            cpu_count: cpu_count,
            data: None
        }
    }

    pub fn generate(&mut self){
        self.data = Some(SimulatorData::generate(0));
    }

    pub fn items_count(&self) -> usize {
        return self
            .data            
            .as_ref()
            .unwrap()
            .countries
            .iter()
            .map(|country| country.items_count())
            .sum();
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let unwraped_data = self.data.as_mut().unwrap();

        let chunk_size = ParallelUtils::get_chunk_size(unwraped_data.countries.len(), self.cpu_count);

        crossbeam::scope(|scope| {
            for countries_chunk in unwraped_data.countries.chunks_mut(chunk_size) {
                let mut cloned_context = context.clone();
                scope.spawn(move |_| {
                    for country in countries_chunk.iter_mut() {
                        country.simulate(&mut cloned_context);
                    }
                });
            }
        }).unwrap();

        context.next_date();
    }
}
