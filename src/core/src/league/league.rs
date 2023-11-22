use crate::context::{GlobalContext, SimulationContext};
use crate::league::{LeagueMatch, LeagueMatchResultResult, LeagueResult, LeagueTable, MatchStorage, Schedule};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::{Club, Team};
use chrono::{Datelike, NaiveDate};

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub country_id: u32,
    pub schedule: Schedule,
    pub table: LeagueTable,
    pub settings: LeagueSettings,
    pub matches: MatchStorage,
    pub reputation: u16,
}

impl League {
    pub fn new(
        id: u32,
        name: String,
        slug: String,
        country_id: u32,
        reputation: u16,
        settings: LeagueSettings,
    ) -> Self {
        League {
            id,
            name,
            slug,
            country_id,
            schedule: Schedule::default(),
            table: LeagueTable::default(),
            matches: MatchStorage::new(),
            settings,
            reputation,
        }
    }

    pub fn simulate(&mut self, clubs: &[Club], ctx: GlobalContext<'_>) -> LeagueResult {
        let table_result = self.table.simulate(&ctx);

        let league_teams: Vec<u32> = clubs
            .iter()
            .flat_map(|c| c.teams.with_league(self.id))
            .collect();

        let mut schedule_result = self
            .schedule
            .simulate(&self.settings, ctx.with_league(self.id, &league_teams));

        if schedule_result.is_match_scheduled() {
            let match_results = self.play_matches(&mut schedule_result.scheduled_matches, clubs);
            self.table.update_from_results(&match_results);

            match_results.clone().into_iter().for_each(|m| {
                self.matches.push(m);
            });

            return LeagueResult::with_match_result(self.id, table_result, match_results);
        }

        LeagueResult::new(self.id, table_result)
    }

    fn play_matches(
        &mut self,
        scheduled_matches: &mut Vec<LeagueMatch>,
        clubs: &[Club],
    ) -> Vec<MatchResult> {
        let mut result = Vec::with_capacity(scheduled_matches.len());

        scheduled_matches.iter_mut().take(1).for_each(|scheduled_match| {
            let home_team = self.get_team(clubs, scheduled_match.home_team_id);
            let away_team = self.get_team(clubs, scheduled_match.away_team_id);

            let match_to_play = Match::make(
                scheduled_match.id.clone(),
                scheduled_match.league_id,
                home_team.get_match_squad(),
                away_team.get_match_squad(),
            );

            let message = &format!(
                "play match: {} - {}",
                &match_to_play.home_squad.team_name, &match_to_play.away_squad.team_name
            );

            let match_result = Logging::estimate_result(|| match_to_play.play(), message);

            scheduled_match.result = Some(LeagueMatchResultResult {
                home_goals: match_result.score.home,
                away_goals: match_result.score.away,
            });

            result.push(match_result);
        });

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

    #[test]
    fn is_time_for_new_schedule_is_correct() {
        //        let mut settings = LeagueSettings {
        //            season_starting: (1, 3),
        //            season_ending: (4, 5),
        //        };
        //
        //        let mut context = SimulationContext::new(
        //            date: NaiveDate::from_ymd_opt(2020, 3, 1)
        //        );
        //
        //        let result = settings.is_time_for_new_schedule(&mut context);
        //
        //        assert_eq!(true, result);
    }
}
