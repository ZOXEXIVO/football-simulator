use crate::context::{GlobalContext, SimulationContext};
use crate::league::{LeagueMatch, LeagueMatchResultResult, LeagueResult, LeagueTable, Schedule};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::{Club, Team};
use chrono::{Datelike, NaiveDate};

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub country_id: u32,
    pub schedule: Schedule,
    pub table: LeagueTable,
    pub settings: LeagueSettings,
    pub reputation: u16,
}

impl League {
    pub fn new(
        id: u32,
        name: String,
        country_id: u32,
        reputation: u16,
        settings: LeagueSettings,
    ) -> Self {
        League {
            id,
            name,
            country_id,
            schedule: Schedule::default(),
            table: LeagueTable::default(),
            settings,
            reputation,
        }
    }

    pub fn simulate(&mut self, clubs: &[Club], ctx: GlobalContext<'_>) -> LeagueResult {
        let mut scheduled_matches = self
            .schedule
            .simulate(&self.settings, ctx.with_league(self.id, &[]));

        let played_matches = self.process_matches(&mut scheduled_matches, clubs);

        let table_result = self.table.simulate(ctx);

        let matches = scheduled_matches
            .iter()
            .map(|lm| MatchResult::from(lm))
            .collect();

        league.table.as_mut().unwrap().update(&matches);

        for match_result in &self.match_results {
            Self::process_match_results(match_result, data);
        }

        LeagueResult::new(self.id, table_result, played_matches)
    }

    fn process_matches(
        &mut self,
        scheduled_matches: &mut Vec<LeagueMatch>,
        clubs: &[Club],
    ) -> Vec<MatchResult> {
        let mut result = Vec::new(); //TODO capacity

        for scheduled_match in scheduled_matches {
            let match_to_play = Match::make(
                scheduled_match.league_id,
                &scheduled_match.id,
                self.get_team(clubs, scheduled_match.home_team_id),
                self.get_team(clubs, scheduled_match.away_team_id),
            );

            let message = &format!(
                "play match: {} - {}",
                &match_to_play.home_team.name, &match_to_play.away_team.name
            );

            let match_result = Logging::estimate_result(|| match_to_play.play(), message);

            scheduled_match.result = Some(LeagueMatchResultResult {
                home_goals: match_result.home_goals,
                away_goals: match_result.away_goals,
            });

            result.push(match_result);
        }

        result
    }

    fn get_team<'c>(&self, clubs: &'c [Club], id: u32) -> &'c Team {
        clubs
            .iter()
            .flat_map(|c| &c.teams.teams)
            .find(|team| team.id == id)
            .unwrap()
    }
}

#[derive(Debug)]
pub struct LeagueSettings {
    pub season_starting_half: DayMonthPeriod,
    pub season_ending_half: DayMonthPeriod,
}

#[derive(Debug)]
pub struct DayMonthPeriod {
    pub from_day: u8,
    pub from_month: u8,

    pub to_day: u8,
    pub to_month: u8,
}

impl DayMonthPeriod {
    pub fn new(from_day: u8, from_month: u8, to_day: u8, to_month: u8) -> Self {
        DayMonthPeriod {
            from_day,
            from_month,
            to_day,
            to_month,
        }
    }
}

impl LeagueSettings {
    pub fn is_time_for_new_schedule(&self, context: &SimulationContext) -> bool {
        let season_starting_date = &self.season_starting_half;

        let date = context.date.date();

        (NaiveDate::day(&date) as u8) == season_starting_date.from_day
            && (date.month() as u8) == season_starting_date.from_month
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::prelude::{NaiveDateTime, NaiveTime};

    #[test]
    fn is_time_for_new_schedule_is_correct() {
        //        let mut settings = LeagueSettings {
        //            season_starting: (1, 3),
        //            season_ending: (4, 5),
        //        };
        //
        //        let mut context = SimulationContext::new(
        //            date: NaiveDate::from_ymd(2020, 3, 1)
        //        );
        //
        //        let result = settings.is_time_for_new_schedule(&mut context);
        //
        //        assert_eq!(true, result);
    }
}
