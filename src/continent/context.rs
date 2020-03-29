use crate::core::SimulationContext;
use chrono::NaiveDateTime;

pub struct ContinentContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl ContinentContext {
    pub fn new(context: &SimulationContext) -> Self {
        ContinentContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
        }
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}
