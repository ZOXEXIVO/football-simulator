use crate::continent::{Tournament, TournamentContext};

pub struct LeagueEurope {}

impl LeagueEurope {}

impl Tournament for LeagueEurope {
    fn simulate(&mut self, context: &mut TournamentContext) {}
}
