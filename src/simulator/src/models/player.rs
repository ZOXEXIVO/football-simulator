use crate::core::{ SimulationContext, SimulationEvent, EventType };

use chrono::prelude::*;

pub struct Player{
      full_name: FullName,
      birth_date: NaiveDate,

      //behaviour: Behavior
}

impl Player{      
      pub fn new(full_name: FullName, birth_date: NaiveDate) -> Player{
            Player{
                  full_name: full_name, 
                  birth_date: birth_date,
                 // behaviour: Behavior::new()
             }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext){
            let current_date = context.date;

            if self.birth_date.month() == current_date.month() && self.birth_date.day() == current_date.day() {
                  context.send( SimulationEvent {
                         event_type: EventType::Birthday 
                  })
            }            

            
      }
}

pub struct FullName{
      pub first_name: String,
      pub last_name: String,
      pub middle_name: String
}