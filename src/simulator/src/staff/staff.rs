use crate::shared::fullname::FullName;
use std::fmt::{Formatter, Display, Result};
use crate::core::{EventType, SimulationContext };
use crate::utils::DateUtils;

use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Staff {
      pub id: u32,
      full_name: FullName,
      birth_date: NaiveDate,
      //behaviour: Behavior
}

impl Staff {
      pub fn new(id: u32, full_name: FullName, birth_date: NaiveDate) -> Staff {
            Staff {
                  id: id,
                  full_name: full_name,
                  birth_date: birth_date,
                  // behaviour: Behavior::new()
            }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            if DateUtils::is_birthday(self.birth_date, context.date){
                  context.send(EventType::Birthday(self.id));
            }
      }
}

//DISPLAY
impl Display for Staff {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}, {}", self.full_name, self.birth_date)
      }
}