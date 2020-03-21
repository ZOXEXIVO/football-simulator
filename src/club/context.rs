use crate::core::SimulationContext;
use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct ClubSimulationContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,

    pub transfer_requests: Vec<u32>,
}

impl ClubSimulationContext {
    pub fn new(context: &SimulationContext) -> Self {
        ClubSimulationContext {
            date: context.date,
            day: context.day,
            hour: context.hour,
            transfer_requests: Vec::new(),
        }
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}
