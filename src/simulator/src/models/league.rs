use crate::models::schedule::Schedule;
use crate::core::SimulationContext;
use crate::models::club::Club;

extern crate rand;
use rand::Rng;

use std::thread;

pub struct League {
    pub name: String,    
    pub clubs: Vec<Club>,
    pub schedule: Option<Schedule>
}

impl League {
   pub fn items_count(&self) -> usize {
            return self.clubs.iter().map(|club| club.items_count()).sum();
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for club in &mut self.clubs {
                  club.simulate(context);
            } 

            let mut rng = rand::thread_rng();

            thread::sleep_ms(rng.gen_range(100, 2000));
      }
}