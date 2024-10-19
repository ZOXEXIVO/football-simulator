use crate::country::CountryResult;
use crate::simulator::SimulatorData;
use crate::SimulationResult;

pub struct ContinentResult {
    pub countries: Vec<CountryResult>,
}

impl ContinentResult {
    pub fn new(countries: Vec<CountryResult>) -> Self {
        ContinentResult { countries }
    }

    pub fn process(self, data: &mut SimulatorData, result: &mut SimulationResult) {
        for country_result in self.countries {
            country_result.process(data, result);
        }
    }
}
