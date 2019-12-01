use crate::{StaffEvent, SimulationContext};

pub struct StaffEventHandlers;

impl StaffEventHandlers{
    pub fn handle(event: StaffEvent, context: &mut SimulationContext){
        match event {
            StaffEvent::Birthday(age) => {}
            StaffEvent::ContractExpired(days) => {}
        }
    }
}