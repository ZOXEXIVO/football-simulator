use crate::club::{ClubResult};
use crate::simulator::SimulatorData;

pub struct LeagueResult{
    pub clubs: Vec<ClubResult>,
}

impl LeagueResult {
    pub fn new(clubs: Vec<ClubResult>) -> Self {
        LeagueResult {
            clubs
        }
    }

    pub fn process(self, data: &mut SimulatorData){
        for result in self.clubs {
            result.process(data);
        }
    }
}