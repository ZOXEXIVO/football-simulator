use crate::people::{Staff, StaffResult};
use crate::simulator::context::GlobalContext;
use crate::simulator::SimulationContext;
pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};

#[derive(Debug, PartialEq)]
pub enum StaffPosition {
    SportDirector,
    MainCoach,
    Coach,
    Physio,
}

#[derive(Debug, PartialEq)]
pub enum StaffStatus {
    Active,
    ExpiredContract,
}

#[derive(Debug)]
pub struct StaffClubContract {
    expired: NaiveDate,
    pub position: StaffPosition,
    pub status: StaffStatus,
}

impl StaffClubContract {
    pub fn new(expired: NaiveDate, position: StaffPosition, status: StaffStatus) -> Self {
        StaffClubContract {
            expired,
            position,
            status,
        }
    }

    pub fn is_expired(&self, context: &SimulationContext) -> bool {
        self.expired >= context.date.date()
    }

    pub fn simulate(&mut self, context: &SimulationContext) {
        if context.check_contract_expiration() && self.is_expired(context) {
            self.status = StaffStatus::ExpiredContract;
        }
    }
}