use crate::league::LeagueResult;
use crate::simulator::SimulatorData;
use crate::{ClubResult, SimulationResult};

pub struct CountryResult {
    pub leagues: Vec<LeagueResult>,
    pub clubs: Vec<ClubResult>,
}

impl CountryResult {
    pub fn new(leagues: Vec<LeagueResult>, clubs: Vec<ClubResult>) -> Self {
        CountryResult { leagues, clubs }
    }

    pub fn process(self, data: &mut SimulatorData, result: &mut SimulationResult) {
        for league_result in self.leagues {
            league_result.process(data, result);
        }

        for club_result in self.clubs {
            club_result.process(data, result);
        }
    }
}
