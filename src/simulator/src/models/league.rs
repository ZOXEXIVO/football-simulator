use crate::models::schedule::Schedule;
use crate::core::SimulationContext;
use crate::models::club::Club;

pub struct League {
    pub name: String,    
    pub clubs: Vec<Club>,
    pub schedule: Option<Schedule>
}

impl League {
   pub fn items_count(&self) -> usize {
            let mut count: usize = 0;
    
            for club in &self.clubs {
                count = count + club.items_count(); 
            }

            count
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for club in &mut self.clubs {
                  club.simulate(context);
            } 
      }
}