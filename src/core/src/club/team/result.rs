use crate::club::{StaffResult, PlayerCollectionResult};
use crate::simulator::SimulatorData;

pub struct TeamResult {
    pub player: PlayerCollectionResult,
    pub staff: StaffResult
}

impl TeamResult {
    pub fn new(player: PlayerCollectionResult, 
               staff: StaffResult) -> Self {
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
