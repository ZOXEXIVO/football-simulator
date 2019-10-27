use core::cell::RefCell;

use crate::utils::ParallelUtils;

extern crate crossbeam;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::models::country::Country;

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

type ProgressCallback = Arc<Box<dyn FnMut(i32) -> ()>>;

pub struct FootballSimulator {
    cpu_count: usize,
    data: SimulatorData,
    progress_callback: ProgressCallback
}

pub struct SimulatorData {
    countries: Vec<RefCell<Country>>
}

impl FootballSimulator {
    pub fn new(cpu_count: usize, progress_callback: ProgressCallback) -> Self {
        println!("sumulator init with {} cores", cpu_count);

        Self {
            cpu_count: cpu_count,
            data: SimulatorData {
                countries: (0..190)
                    .map(|_| RefCell::new(Generator::generate()))
                    .collect(),
            },
            progress_callback: progress_callback
        }
    }

    fn set_progress(&mut self, value: i32){
        
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let chunk_size =
            ParallelUtils::calculate_chunk_size(self.data.countries.len(), self.cpu_count);

        let mut local_progress = Arc::new(AtomicI32::new(0));

        crossbeam::scope(|scope| {
            for countries_chunk in self.data.countries.chunks_mut(chunk_size) {
                let mut cloned_context = context.clone();

                let cloned_progress = Arc::clone(&local_progress);
                let cloned_callback = Arc::clone(&self.progress_callback);

                scope.spawn(move |_| {
                    for country in countries_chunk.iter_mut() {
                        country.borrow_mut().simulate(&mut cloned_context);

                        cloned_progress.fetch_add(1, Ordering::SeqCst);

                        (*cloned_callback)(cloned_progress.into_inner());
                    }
                });
            }
        })
        .unwrap();

        context.next_date();
    }
}
