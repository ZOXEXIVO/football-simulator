mod champion_league;
mod context;
mod league_europe;

pub use context::*;

pub trait Tournament {
    fn simulate(&mut self, context: &mut TournamentContext);
}
