use crate::continent::{Tournament, TournamentContext};
use crate::context::GlobalContext;

pub struct ChampionLeague {}

impl ChampionLeague {}

impl Tournament for ChampionLeague {
    fn simulate(&mut self, tournament_ctx: &mut TournamentContext, ctx: GlobalContext) {}
}
