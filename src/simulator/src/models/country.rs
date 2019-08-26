use crate::core::SimulationContext;
use crate::models::league::League;

pub struct Country {
      pub name: String,    
      pub leagues: Vec<League>
}

impl Country{      
      pub fn new(name: String, leagues: Vec<League>) -> Country {
            Country { name: name, leagues: leagues }
      }

      pub fn items_count(&self) -> usize {
            let mut count: usize = 0;
    
            for league in &self.leagues {
                count = count + league.items_count(); 
            }

            count
      } 

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for league in &mut self.leagues {
                  league.simulate(context);
            }
      }
}