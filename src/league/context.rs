use crate::core::context::Context;
use crate::country::CountryContext;
use chrono::NaiveDateTime;

pub struct LeagueContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl LeagueContext {
    pub fn new(context: &CountryContext) -> Self {
        LeagueContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
        }
    }
}

impl Context for LeagueContext {
    fn date(&self) -> NaiveDateTime {
        self.date
    }

    fn hour(&self) -> u8 {
        self.hour
    }
}
