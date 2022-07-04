use crate::context::GlobalContext;
use crate::league::round::RoundSchedule;
use crate::league::{
    League, LeagueMatch, LeagueMatchResultResult, LeagueResult, LeagueTable, ScheduleGenerator,
    Season,
};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::Country;
use chrono::Datelike;
use log::error;

pub struct LeagueScheduleProcessor;

impl LeagueScheduleProcessor {
    pub fn process(league: &mut League, ctx: &GlobalContext<'_>) -> Vec<LeagueMatch> {
        if league.settings.is_time_for_new_schedule(&ctx.simulation) || league.schedule.is_none() {
            let league_ctx = ctx.league.as_ref().unwrap();

            league.table = Some(LeagueTable::with_clubs(&league_ctx.team_ids));

            let schedule_generator = RoundSchedule::new();

            let league_ctx = ctx.league.as_ref().unwrap();

            match schedule_generator.generate(
                league.id,
                Season::OneYear(ctx.simulation.date.year() as u16),
                league_ctx.team_ids,
                &league.settings,
            ) {
                Ok(generated_schedule) => {
                    league.schedule = Some(generated_schedule);
                }
                Err(error) => {
                    error!("Generating schedule error: {}", error.message);
                }
            }
        }

        let scheduled_matches = league
            .schedule
            .as_ref()
            .unwrap()
            .get_matches(ctx.simulation.date)
            .iter()
            .map(|sm| LeagueMatch {
                id: sm.id.clone(),
                league_id: sm.league_id,
                date: sm.date,
                home_team_id: sm.home_team_id,
                away_team_id: sm.away_team_id,
                result: None,
            })
            .collect();

        scheduled_matches
    }
}
