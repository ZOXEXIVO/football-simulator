use crate::club::academy::result::ClubAcademyResult;
use crate::club::{BoardResult, ClubFinanceResult};
use crate::simulator::SimulatorData;
use crate::TeamResult;

pub struct ClubResult {
    pub finance: ClubFinanceResult,
    pub teams: Vec<TeamResult>,
    pub board: BoardResult,
    pub academy: ClubAcademyResult,
}

impl ClubResult {
    pub fn new(finance: ClubFinanceResult, teams: Vec<TeamResult>, board: BoardResult, academy: ClubAcademyResult) -> Self {
        ClubResult {
            finance,
            teams,
            board,
            academy,
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        self.finance.process(data);
        
        for team_result in self.teams {
            team_result.process(data);
        }
        
        self.board.process(data);       
        self.academy.process(data);
    }
}
