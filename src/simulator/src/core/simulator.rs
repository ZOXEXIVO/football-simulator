use core::cell::RefCell;
use std::slice;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::utils::ParallelUtils;

extern crate crossbeam;

use crate::core::context::SimulationContext;
use crate::generators::Generator;
use crate::models::country::Country;

pub struct FootballSimulator {
    cpu_count: usize,
    data: SimulatorData,
}

pub struct SimulatorData {
    countries: Vec<RefCell<Country>>,
}

impl FootballSimulator {
    pub fn new(cpu_count: usize) -> Self {
        println!("sumulator init with {} cores", cpu_count);

        Self {
            cpu_count: cpu_count,
            data: SimulatorData {
                countries: (0..190)
                    .map(|_| RefCell::new(Generator::generate()))
                    .collect(),
            },
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        let chunk_size =
            ParallelUtils::calculate_chunk_size(self.data.countries.len(), self.cpu_count);

        crossbeam::scope(|scope| {
            for countries_chunk in self.data.countries.chunks_mut(chunk_size) {
                let mut cloned_context = context.clone();

                scope.spawn(move |_| {
                    for country in countries_chunk.iter_mut() {
                        country.borrow_mut().simulate(&mut cloned_context);
                    }
                });
            }
        })
        .unwrap();

        context.next_date();
    }
}
