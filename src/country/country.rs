use crate::core::context::GlobalContext;
use crate::league::{League, LeagueContext};

pub struct Country {
    pub name: String,
    pub leagues: Vec<League>,
    pub reputation: u16,
}

impl Country {
    pub fn items_count(&self) -> usize {
        self.leagues.iter().map(|league| league.items_count()).sum()
    }

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        let ctx = ctx.with_league(LeagueContext::new());

        for league in &mut self.leagues {
            league.simulate(ctx);
        }
    }
}
