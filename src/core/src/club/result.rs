use crate::club::{BoardResult, ClubFinanceResult, StaffResult, PlayerCollectionResult};
use crate::simulator::SimulatorData;
use crate::club::academy::result::ClubAcademyResult;

pub struct ClubResult {
    pub board: BoardResult,
    pub player: PlayerCollectionResult,
    pub staff: StaffResult,
    pub finance: ClubFinanceResult,
    pub academy: ClubAcademyResult
}

impl ClubResult {
    pub fn new(board: BoardResult, player: PlayerCollectionResult, 
               staff: StaffResult, finance: ClubFinanceResult,
               academy: ClubAcademyResult) -> Self {
        ClubResult {
            board,
            player,
            staff,
            finance,
            academy
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        self.board.process(data);
        self.player.process(data);
        self.staff.process(data);
        self.finance.process(data);
        self.academy.process(data);
    }
}
