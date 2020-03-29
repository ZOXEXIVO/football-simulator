use crate::club::ClubContext;
use crate::core::context::Context;
use chrono::NaiveDateTime;

pub struct BoardContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl BoardContext {
    pub fn new(context: &ClubContext) -> Self {
        BoardContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
        }
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}

impl Context for BoardContext {
    fn date(&self) -> NaiveDateTime {
        self.date
    }

    fn hour(&self) -> u8 {
        self.hour
    }
}
