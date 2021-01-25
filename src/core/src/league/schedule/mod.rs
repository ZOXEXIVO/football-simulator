pub mod models;
pub mod types;

pub use models::*;
pub use types::*;
use crate::league::{Season, LeagueSettings};

pub trait ScheduleGenerator {
    fn generate(&self, league_id: u32, season: Season, teams: &[u32], league_settings: &LeagueSettings) -> Result<Schedule, ScheduleError>;
}