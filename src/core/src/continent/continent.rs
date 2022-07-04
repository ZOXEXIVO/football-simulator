use crate::context::GlobalContext;
use crate::continent::{ContinentCountryProcessor, ContinentResult};
use crate::country::CountryResult;
use crate::utils::Logging;
use crate::Country;
pub use rayon::prelude::*;

pub struct Continent {
    pub id: u32,
    pub name: String,
    pub countries: Vec<Country>,
}

impl Continent {
    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ContinentResult {
        let country_result = ContinentCountryProcessor::process(self, &ctx);
        ContinentResult::new(country_result)
    }
}
