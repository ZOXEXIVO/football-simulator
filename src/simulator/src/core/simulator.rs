use core::cell::RefCell;

use crate::utils::ParallelUtils;

extern crate crossbeam;
extern crate crossbeam_channel;

use crossbeam_channel::Sender;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::models::country::Country;

pub struct FootballSimulator<'c>  {
    cpu_count: usize,
    data: SimulatorData<'c>
}

pub struct SimulatorData<'c>  {
    countries: Vec<RefCell<Country<'c>>>
}

impl<'c> FootballSimulator<'c>  {
    pub fn new(cpu_count: usize) -> Self {
        println!("sumulator init with {} cores", cpu_count);

        Self {
            cpu_count: cpu_count,
            data: SimulatorData {
                countries: (0..16)
                    .map(|i| RefCell::new(Generator::generate(i)))
                    .collect()
            }
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext, progress_sender: &Sender<i32>) {
        let chunk_size =
            ParallelUtils::get_chunk_size(self.data.countries.len(), self.cpu_count);

        progress_sender.send(0).unwrap();


        crossbeam::scope(|scope| {            
            for countries_chunk in self.data.countries.chunks_mut(chunk_size) {
                let mut cloned_context = context.clone();
  
                scope.spawn(move |_| {
                    for country in countries_chunk.iter_mut() {             
                        country.borrow_mut().simulate(&mut cloned_context);
                        
                        progress_sender.send(1).unwrap();
                    }
                });
            }
        })
        .unwrap();

        context.next_date();
    }
}
