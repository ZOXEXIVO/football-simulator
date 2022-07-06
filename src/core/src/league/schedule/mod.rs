pub mod models;
pub mod round;

use crate::league::{LeagueSettings, Season};
pub use models::*;

pub trait ScheduleGenerator {
    fn generate(
        &self,
        league_id: u32,
        season: Season,
        teams: &[u32],
        league_settings: &LeagueSettings,
    ) -> Result<Vec<ScheduleTour>, ScheduleError>;
}
