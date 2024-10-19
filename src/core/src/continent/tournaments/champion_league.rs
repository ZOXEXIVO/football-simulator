use crate::context::GlobalContext;
use crate::continent::{Tournament, TournamentContext};

pub struct ChampionLeague {}

impl ChampionLeague {}

impl Tournament for ChampionLeague {
    fn simulate(&mut self, _: &mut TournamentContext, _: GlobalContext<'_>) {}
}
