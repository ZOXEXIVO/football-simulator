use std::fmt::{Formatter, Display, Result};
use crate::core::{EventType, SimulationContext};

use chrono::prelude::*;

pub struct Player {
      id: u32,
      full_name: FullName,
      birth_date: NaiveDate,
      //behaviour: Behavior
}

impl Player {
      pub fn new(id: u32, full_name: FullName, birth_date: NaiveDate) -> Player {
            Player {
                  id: id,
                  full_name: full_name,
                  birth_date: birth_date,
                  // behaviour: Behavior::new()
            }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            let current_date = context.date;

            if self.birth_date.month() == current_date.month()
                  && self.birth_date.day() == current_date.day()
            {
                  context.send(EventType::Birthday(self.id))
            }
      }
}

pub struct FullName {
      pub first_name: String,
      pub last_name: String,
      pub middle_name: String,
}

//DISPLAY
impl Display for Player {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}, {}", self.full_name, self.birth_date)
      }
}

impl Display for FullName {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {} {}", self.last_name, self.first_name, self.middle_name)
      }
}