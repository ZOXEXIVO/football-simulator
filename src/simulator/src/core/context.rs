pub use chrono::prelude::*;

use chrono::{ Duration };

#[derive(Clone)]
pub struct SimulationContext {       
    pub date: NaiveDateTime,
    pub hour: u32
}

impl SimulationContext {
      pub fn new(date: NaiveDateTime) -> Self {
            SimulationContext { 
                  date,
                  hour: 0
            }
      }

      pub fn next_date(&mut self){
          self.date += Duration::hours(1);
          self.hour = self.date.time().hour();
      }

      pub fn check_contract_expiration(&self) -> bool {
          self.hour == 0
      }  
}
