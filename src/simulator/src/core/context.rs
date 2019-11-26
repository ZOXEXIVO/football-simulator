pub use chrono::prelude::*;

use chrono::{ Duration };

#[derive(Clone)]
pub struct SimulationContext {       
    pub date: NaiveDateTime
}

impl SimulationContext {
      pub fn new(date: NaiveDateTime) -> Self {
            SimulationContext { 
                  date
            }
      }

      pub fn next_date(&mut self){
          self.date += Duration::hours(1);
      }

      pub fn check_contract_expiration(&self) -> bool {
          self.date.time().hour() == 0
      }  
}
