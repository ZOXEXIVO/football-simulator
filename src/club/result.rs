use crate::people::{StaffResult, PlayerCollectionResult};
use crate::club::BoardResult;
use crate::simulator::SimulatorData;

pub struct ClubResult {
    pub board: BoardResult,
    pub player: PlayerCollectionResult,
    pub staff: StaffResult
}

impl ClubResult {
    pub fn new(board: BoardResult, player: PlayerCollectionResult, staff: StaffResult) -> Self {
        ClubResult {
            board,
            player,
            staff
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        self.board.process(data);
        self.player.process(data);
        self.staff.process(data);
    }
}