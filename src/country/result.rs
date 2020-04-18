use crate::league::LeagueResult;

pub struct CountryResult{
    pub leagues: Vec<LeagueResult>,
}

impl CountryResult {
    pub fn new(leagues: Vec<LeagueResult>) -> Self {
        CountryResult {           
            leagues           
        }
    }
}