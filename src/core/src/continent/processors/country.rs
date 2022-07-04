use crate::context::GlobalContext;
use crate::continent::Continent;
use crate::league::round::RoundSchedule;
use crate::league::{
    League, LeagueMatch, LeagueMatchResultResult, LeagueResult, LeagueTable, ScheduleGenerator,
    Season,
};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::{Country, CountryResult};
use chrono::Datelike;
use log::error;

pub struct ContinentCountryProcessor;

impl ContinentCountryProcessor {
    pub fn process(continent: &mut Continent, ctx: &GlobalContext<'_>) -> Vec<CountryResult> {
        continent
            .countries
            .iter_mut()
            .map(|country| {
                let message = &format!("simulate country: {}", &country.name);
                Logging::estimate_result(|| country.simulate(ctx.with_country(country.id)), message)
            })
            .collect()
    }
}
