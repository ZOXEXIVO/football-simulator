use crate::league::LeagueResult;
use crate::simulator::SimulatorData;
use crate::ClubResult;
use crate::r#match::MatchResult;

pub struct CountryResult{
    pub leagues: Vec<LeagueResult>,
    pub clubs: Vec<ClubResult>,
    pub match_results: Vec<MatchResult>
}

impl CountryResult {
    pub fn new(leagues: Vec<LeagueResult>, clubs: Vec<ClubResult>, match_results: Vec<MatchResult>) -> Self {
        CountryResult {           
            leagues, 
            clubs,
            match_results
        }
    }

    pub fn process(self, data: &mut SimulatorData){
        for league_result in self.leagues {
            league_result.process(data);
        }

        for club_result in self.clubs {
            club_result.process(data);
        }
    }
}