mod champion_league;
mod context;
mod league_europe;

use crate::context::GlobalContext;
pub use context::*;

pub trait Tournament {
    fn simulate(&mut self, tournament_ctx: &mut TournamentContext, ctx: GlobalContext<'_>);
}
