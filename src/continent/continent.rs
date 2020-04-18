use crate::Country;

use crate::simulator::context::GlobalContext;
pub use rayon::prelude::*;
use crate::country::CountryResult;
use crate::continent::ContinentResult;

pub struct Continent {
    pub id: u32,
    pub name: String,
    pub countries: Vec<Country>
}

impl Continent {
    pub fn simulate(&mut self, ctx: GlobalContext) -> ContinentResult {
        let country_results: Vec<CountryResult> = self.countries.iter_mut().map(
            |country| country.simulate(ctx.with_country(country.id))
        ).collect();

        ContinentResult::new(country_results)
    }
}
