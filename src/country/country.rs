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
        let mut league_ctx = LeagueContext::new();

        for league in &mut self.leagues {
            league.simulate(&mut ctx.with_league(&mut league_ctx));
        }
    }
}
