use crate::club::{PlayerCollectionResult};
use crate::simulator::SimulatorData;
use crate::StaffCollectionResult;

pub struct TeamResult {
    pub player: PlayerCollectionResult,
    pub staff: StaffCollectionResult
}

impl TeamResult {
    pub fn new(player: PlayerCollectionResult, 
               staff: StaffCollectionResult) -> Self {
        TeamResult {
            player,
            staff
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        self.player.process(data);
        self.staff.process(data);
    }
}
