extern crate chrono;
pub use chrono::prelude::*;

pub struct SimulationContext {       
    pub events: Vec<SimulationEvent>,
    pub date: NaiveDate
}

impl SimulationContext {
      pub fn new(date: NaiveDate) -> SimulationContext {
            SimulationContext { 
                  events: vec![],
                  date: date
            }
      }

      pub fn send(&mut self, event: SimulationEvent){
            self.events.push(event);
      }
}

pub struct SimulationEvent {       
     pub event_type: EventType
}

pub enum EventType{
     Birthday
}