use crate::core::SimulationContext;
use crate::shared::fullname::FullName;
use crate::utils::DateUtils;
use std::fmt::{Display, Formatter, Result};

use crate::Behaviour;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Staff {
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub behaviour: Behaviour,
}

impl Staff {
    pub fn new(id: u32, full_name: FullName, birth_date: NaiveDate) -> Self {
        Staff {
            id,
            full_name,
            birth_date,
            behaviour: Behaviour::default(),
        }
    }

    pub fn stub() -> Self {
        Staff {
            id: 0,
            full_name: FullName {
                first_name: "stub".to_string(),
                last_name: "stub".to_string(),
                middle_name: "stub".to_string(),
            },
            behaviour: Behaviour::default(),
            birth_date: NaiveDate::from_ymd(2019, 1, 1),
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) -> Vec<StaffEvent> {
        let mut result_events = Vec::new();

        if DateUtils::is_birthday(self.birth_date, context.date.date()) {
            result_events.push(StaffEvent::Birthday(self.id));
        }

        result_events
    }
}

#[derive(Debug, Clone)]
pub enum StaffEvent {
    Birthday(u32),
    ContractExpired(i32),
}

//DISPLAY
impl Display for Staff {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}
