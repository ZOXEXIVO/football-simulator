pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};
use crate::context::SimulationContext;

#[derive(Debug)]
pub struct PlayerClubContract {
    pub salary: f64,
    pub expired: NaiveDate,
    pub additional_options: AdditionalOptions,
}

#[derive(Debug)]
pub struct AdditionalOptions {
    pub yearly_increase_wage: u16,
}

impl PlayerClubContract {
    pub fn new(salary: f64, expired: NaiveDate) -> Self {
        PlayerClubContract {
            salary,
            expired,
            additional_options: AdditionalOptions {
                yearly_increase_wage: 15,
            },
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();

        let naive_now = NaiveDate::from_ymd(now.year(), now.month(), now.day());

        self.expired >= naive_now
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if context.check_contract_expiration() && self.is_expired() {}
    }
}