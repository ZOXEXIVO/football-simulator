pub mod models;
pub mod round;

pub use models::*;
use crate::league::{Season, LeagueSettings};

pub trait ScheduleGenerator {
    fn generate(&self, league_id: u32, season: Season, teams: &[u32], league_settings: &LeagueSettings) -> Result<Schedule, ScheduleError>;
}