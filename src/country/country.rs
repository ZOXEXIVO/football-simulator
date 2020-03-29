use crate::country::CountryContext;
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

    pub fn simulate(&mut self, context: &mut CountryContext) {
        for league in &mut self.leagues {
            let mut context = LeagueContext::new(context);
            league.simulate(&mut context);
        }
    }
}
