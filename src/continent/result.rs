use crate::country::CountryResult;
use crate::simulator::SimulatorData;

pub struct ContinentResult {
    pub countries: Vec<CountryResult>,
}

impl ContinentResult {
    pub fn new(countries: Vec<CountryResult>) -> Self {
        ContinentResult {
            countries
        }
    }
    
    pub fn process(self, data: &mut SimulatorData){
        for result in self.countries {
            result.process(data);
        }
    }
}