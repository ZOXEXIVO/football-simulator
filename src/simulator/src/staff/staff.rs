use crate::shared::fullname::FullName;
use std::fmt::{Formatter, Display, Result};
use crate::core::{SimulationContext};
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
            id,
            full_name,
            birth_date,
            // behaviour: Behavior::new()
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) -> Vec<StaffEvent> {
        let mut result = Vec::new();
        
        if DateUtils::is_birthday(self.birth_date, context.date.date()) {
           
        }

        result 
    }
}

#[derive(Debug, Clone)]
pub enum StaffEvent {
    Birthday(i32),
    ContractExpired(i32)
}

//DISPLAY
impl Display for Staff {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}