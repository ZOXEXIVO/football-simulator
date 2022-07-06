pub mod schedule;
pub mod round;
pub mod result;

use crate::league::{LeagueSettings, Season};
pub use schedule::*;
pub use result::*;

pub trait ScheduleGenerator {
    fn generate(
        &self,
        league_id: u32,
        season: Season,
        teams: &[u32],
        league_settings: &LeagueSettings,
    ) -> Result<Vec<ScheduleTour>, ScheduleError>;
}
