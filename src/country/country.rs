use crate::simulator::context::GlobalContext;
use crate::league::{League, LeagueResult};
use crate::country::CountryResult;

pub struct Country {
    pub id: u32,
    pub name: String,
    pub leagues: Vec<League>,
    pub reputation: u16
}

impl Country {
    pub fn simulate(&mut self, ctx: GlobalContext) -> CountryResult {
        let league_results: Vec<LeagueResult> = self.leagues.iter_mut()
            .map(|league| league.simulate(ctx.with_league(league.id)))
            .collect();
        
        CountryResult::new(league_results)
    }
}
