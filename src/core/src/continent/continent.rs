use crate::context::GlobalContext;
use crate::continent::ContinentResult;
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
        let country_results: Vec<CountryResult> = self
            .countries
            .par_iter_mut()
            .map(|country| {
                let message = &format!("simulate country: {}", &country.name);
                Logging::estimate_result(|| country.simulate(ctx.with_country(country.id)), message)
            })
            .collect();

        ContinentResult::new(country_results)
    }
}
