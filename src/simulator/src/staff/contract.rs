
use crate::staff::staff::Staff;
use crate::core::{EventType, SimulationContext };

pub use chrono::prelude::{NaiveDate, DateTime, Utc, Datelike};

pub enum StaffPosition{
      SportDirector,
      MainCoach,
      Coach
}

#[derive(Debug, Clone)]
pub struct StaffClubContract {
      staff: Staff,
      expired: NaiveDate,
      position: StaffPosition
}

impl StaffClubContract {
      pub fn new(staff: Staff, expired: NaiveDate) -> Self {
            StaffClubContract {
                  staff,
                  expired,
            }
      }

      pub fn is_expired(&self, context: &mut SimulationContext) -> bool {
            self.expired >= context.date.date()
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            if self.is_expired(context) {
                  context.send(EventType::StaffContractExpired(self.staff.id))
            }

            self.staff.simulate(context);
      }
}

pub struct StaffCollection{
      staffs: Vec<StaffClubContract>
}

impl StaffCollection{
      pub fn get_main_coach(&self) -> Option<&Staff>{
            let main_coach_contract = self.staffs.iter()
            .find(|c| c.position == StaffPosition::MainCoach);

            if main_coach_contract.is_none(){
                  return None;
            }
            
            Some(main_coach_contract.staff)
      }
}

impl IntoIter for StaffCollection{
      type Item = StaffClubContract;
      type IntoIter = std::Vec::IntoIter<StaffClubContract>;

      fn into_iter(self) -> Self::IntoIter{
            self.staffs.into_iter()
      }
}