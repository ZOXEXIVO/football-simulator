
use crate::staff::staff::Staff;
use crate::core::{EventType, SimulationContext };

pub use chrono::prelude::{NaiveDate, DateTime, Utc, Datelike};

#[derive(Clone)]
pub struct StaffClubContract {
      staff: Staff,
      expired: NaiveDate,
}

impl StaffClubContract {
      pub fn new(staff: Staff, expired: NaiveDate) -> Self {
            StaffClubContract {
                  staff: staff,
                  expired: expired,
            }
      }

      pub fn is_expired(&self, context: &mut SimulationContext) -> bool {
            self.expired >= context.date
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            if self.is_expired(context) {
                  context.send(EventType::StaffContractExpired(self.staff.id))
            }

            self.staff.simulate(context);
      }
}
