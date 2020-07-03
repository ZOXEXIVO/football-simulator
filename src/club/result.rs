use crate::club::{BoardResult, ClubFinanceResult, StaffResult, PlayerCollectionResult};
use crate::simulator::SimulatorData;

pub struct ClubResult {
    pub board: BoardResult,
    pub player: PlayerCollectionResult,
    pub staff: StaffResult,
    pub finance: ClubFinanceResult
}

impl ClubResult {
    pub fn new(board: BoardResult, player: PlayerCollectionResult, 
               staff: StaffResult, finance: ClubFinanceResult) -> Self {
        ClubResult {
            board,
            player,
            staff,
            finance
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        self.board.process(data);
        self.player.process(data);
        self.staff.process(data);
    }
}
