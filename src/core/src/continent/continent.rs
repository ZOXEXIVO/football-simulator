use crate::Country;

use crate::context::GlobalContext;
pub use rayon::prelude::*;
use crate::country::CountryResult;
use crate::continent::ContinentResult;
use log::{debug};

pub struct Continent {
    pub id: u32,
    pub name: String,
    pub countries: Vec<Country>
}

impl Continent {
    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ContinentResult {
        debug!("start simulating continent: {}", &self.name);
        
        let country_results: Vec<CountryResult> = self.countries.iter_mut().map(
            |country| country.simulate(ctx.with_country(country.id))
        ).collect();

        debug!("end simulating continent: {}", &self.name);
        
        ContinentResult::new(country_results)
    }
}
