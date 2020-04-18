use crate::Country;

use crate::continent::{Tournament, TournamentContext};
use crate::simulator::context::GlobalContext;
pub use rayon::prelude::*;

pub struct Continent {
    pub name: String,
    pub countries: Vec<Country>,

    //pub tournaments: Vec<Box<dyn Tournament>>,
}

impl Continent {
    pub fn simulate(&mut self, ctx: GlobalContext) {
        self.countries.par_iter_mut().for_each(
            |country| country.simulate(ctx.with_country())
        );
        
        // let mut tournament_ctx = TournamentContext::new();
        //
        // for tournament in &mut self.tournaments {
        //     tournament.simulate(&mut tournament_ctx, ctx)
        // }
    }
}
