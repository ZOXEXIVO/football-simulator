use crate::league::LeagueResult;
use crate::simulator::SimulatorData;

pub struct CountryResult{
    pub leagues: Vec<LeagueResult>,
}

impl CountryResult {
    pub fn new(leagues: Vec<LeagueResult>) -> Self {
        CountryResult {           
            leagues           
        }
    }

    pub fn process(self, data: &mut SimulatorData){
        for result in self.leagues {
            result.process(data);
        }
    }
}