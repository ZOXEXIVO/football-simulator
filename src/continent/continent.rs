use crate::Country;

use crate::continent::{ContinentContext, Tournament, TournamentContext};
use crate::country::CountryContext;
pub use rayon::prelude::*;

pub struct Continent {
    pub name: String,
    pub countries: Vec<Country>,

    pub tournaments: Vec<Box<dyn Tournament>>,
}

impl Continent {
    pub fn items_count(&self) -> usize {
        self.countries
            .iter()
            .map(|country| country.items_count())
            .sum()
    }

    pub fn simulate(&mut self, context: &mut ContinentContext) {
        self.countries.par_iter_mut().for_each(|country| {
            let mut context = CountryContext::new(context);

            country.simulate(&mut context);
        });

        for tournament in &mut self.tournaments {
            let mut context = TournamentContext::new(context);

            tournament.simulate(&mut context)
        }
    }
}
