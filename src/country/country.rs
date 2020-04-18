use crate::simulator::context::GlobalContext;
use crate::league::League;

pub struct Country {
    pub name: String,
    pub leagues: Vec<League>,
    pub reputation: u16,
}

impl Country {
    pub fn simulate(&mut self, ctx: GlobalContext) {
        for league in &mut self.leagues {
            league.simulate(ctx.with_league());
        }
    }
}
