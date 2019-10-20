use std::sync::{ Arc, Mutex };
use std::thread;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::models::country::Country;

pub struct FootballSimulator {
    cpu_count: usize,
    data: Option<Arc<SimulatorData>>
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

        self.data = Some(Arc::new(simulator_data));
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let thread_handles = Vec::with_capacity(self.cpu_count);

        let batch_size = self.data.unwrap().countries.len() / self.cpu_count;

        for i in 0..thread_handles.len() {
            let local_data = self.data.unwrap().clone();

            let start_idx = i * batch_size;
            let end_idx = start_idx + batch_size;

            let thread_handle = thread::spawn(move || {
                let countries_slice = local_data.countries.iter().skip(start_idx).take(batch_size).collect();

                for country in countries_slice{
                    country.simulate();
                }
            });

            thread_handles.push(thread_handle);
        }

        for thread_handle in thread_handles{
            thread_handle.join();
        }

        context.next_date();
    }
}
