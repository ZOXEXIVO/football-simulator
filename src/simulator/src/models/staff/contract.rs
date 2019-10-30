
use crate::models::staff::staff::Staff;
use crate::core::context::SimulationContext;

pub use chrono::prelude::{NaiveDate, DateTime, Utc, Datelike};

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

      pub fn is_expired(&self) -> bool {
            let now = Utc::now();

            let naive_now = NaiveDate::from_ymd(
                  now.year(), now.month(), now.day()
            );

            self.expired >= naive_now
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            self.staff.simulate(context);
      }
}
