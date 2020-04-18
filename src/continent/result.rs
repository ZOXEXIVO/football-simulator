use crate::country::CountryResult;

pub struct ContinentResult {
    pub clubs: Vec<CountryResult>,
}

impl ContinentResult {
    pub fn new(clubs: Vec<CountryResult>) -> Self {
        ContinentResult {
            clubs
        }
    }
}