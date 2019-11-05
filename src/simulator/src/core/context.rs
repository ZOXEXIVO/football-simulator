use crate::core::events::EventType;
pub use chrono::prelude::*;

use chrono::{ Duration };

#[derive(Clone)]
pub struct SimulationContext {       
    pub events: Vec<EventType>,
    pub date: NaiveDate
}

impl SimulationContext {
      pub fn new(date: NaiveDate) -> SimulationContext {
            SimulationContext { 
                  events: vec![],
                  date: date
            }
      }

      pub fn next_date(&mut self){
           self.date = self.date + Duration::days(1);
      }

      pub fn send(&mut self, event: EventType){
            self.events.push(event);
      }
}
