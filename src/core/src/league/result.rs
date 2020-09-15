use crate::club::{ClubResult};
use crate::simulator::SimulatorData;
use crate::r#match::MatchResult;

pub struct LeagueResult{
    pub clubs: Vec<ClubResult>,
    pub match_results: Vec<MatchResult>
}

impl LeagueResult {
    pub fn new(clubs: Vec<ClubResult>, match_results: Vec<MatchResult>) -> Self {
        LeagueResult {
            clubs,
            match_results
        }
    }

    pub fn process(self, data: &mut SimulatorData){
        for result in self.clubs {
            result.process(data);
        }
    }
}