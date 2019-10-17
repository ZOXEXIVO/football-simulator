use std::sync::Arc;
use std::thread;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::models::country::Country;

pub struct FootballSimulator {
    cpu_count: usize,
    data: Option<SimulatorData>
}

pub struct SimulatorData {
     countries: Vec<Country>
}

impl FootballSimulator {
    pub fn new(cpu_count: usize) -> Self {
        Self {
            cpu_count: cpu_count,
            data: None
        }
    }

    pub fn generate(&mut self){
        let simulator_data = SimulatorData{
            countries: (0..10).map(|_| Generator::generate()).collect()
        };

        self.data = Some(simulator_data);
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let thread_handles = Vec::with_capacity(self.cpu_count);

        let batch_size = self.data.unwrap().countries.len()  / self.cpu_count;

        for i in 0..thread_handles.len() {
            let local_country = self.data.unwrap().countries;

          

            let mut local_simulation_context = context.clone();

            let thread_handle = thread::spawn(move || {
                local_country.simulate(&mut local_simulation_context);
            });

            thread_handles.push(thread_handle);
        }

        for thread_handle in thread_handles{
            thread_handle.join();
        }

        context.next_date();
    }
}
