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
     countries: Vec<Mutex<Country>>
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
            countries: (0..10).map(|_| Mutex::new(Generator::generate())).collect()
        };

        self.data = Some(Arc::new(simulator_data));
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let thread_handles = Vec::with_capacity(self.cpu_count);

        let batch_size = self.data.unwrap().countries.len() / self.cpu_count;

        for i in 0..thread_handles.len() {
            let local_countries = self.data.unwrap().clone();

            let current_batch = local_countries.countries
            .iter()
            .skip(i - 1)
            .take(batch_size)
            .collect::<&Mutex<Country>>();

            let mut local_simulation_context = context.clone();

            let thread_handle = thread::spawn(move || {
                for ti in current_batch{
                    ti.simulate();
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
