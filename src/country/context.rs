use crate::continent::ContinentContext;
use crate::core::context::Context;
use chrono::NaiveDateTime;

pub struct CountryContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl CountryContext {
    pub fn new(context: &ContinentContext) -> Self {
        CountryContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
        }
    }
}

impl Context for CountryContext {
    fn date(&self) -> NaiveDateTime {
        self.date
    }

    fn hour(&self) -> u8 {
        self.hour
    }
}
