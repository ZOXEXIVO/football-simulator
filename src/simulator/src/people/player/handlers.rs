use crate::{PlayerEvent, SimulationContext};

pub struct PlayerEventHandlers;

impl PlayerEventHandlers{
    pub fn handle(event: PlayerEvent, context: &mut SimulationContext){
        match event {
            PlayerEvent::Birthday(pid) => {
                
            }
            PlayerEvent::ContractExpired(days) => {
                
            }
        }
    }
}