use crate::league::LeagueContext;
use chrono::NaiveDateTime;

pub struct ClubContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl ClubContext {
    pub fn new(context: &LeagueContext) -> Self {
        ClubContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
        }
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}
