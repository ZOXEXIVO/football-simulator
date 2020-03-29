pub use chrono::prelude::*;

use chrono::Duration;

pub trait Context {
    fn date(&self) -> NaiveDateTime;
    fn hour(&self) -> u8;
}

#[derive(Clone)]
pub struct SimulationContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl SimulationContext {
    pub fn new(date: NaiveDateTime) -> Self {
        SimulationContext {
            date,
            day: 0,
            hour: 0,
        }
    }

    pub fn next_date(&mut self) {
        self.date += Duration::hours(1);

        self.day = self.date.day() as u8;
        self.hour = self.date.time().hour() as u8;
    }
}

impl Context for SimulationContext {
    fn date(&self) -> NaiveDateTime {
        self.date
    }

    fn hour(&self) -> u8 {
        self.hour
    }
}
