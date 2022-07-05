use crate::continent::{Tournament, TournamentContext};
use crate::context::GlobalContext;

pub struct LeagueEurope {}

impl LeagueEurope {}

impl Tournament for LeagueEurope {
    fn simulate(&mut self, _: &mut TournamentContext, _: GlobalContext<'_>) {
    }
}
