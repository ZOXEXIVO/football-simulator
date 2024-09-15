use crate::context::{GlobalContext, SimulationContext};
use crate::league::{
    LeagueMatch, LeagueMatchResultResult, LeagueResult, LeagueTable, MatchStorage, Schedule,
};
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

        scheduled_matches
            .iter_mut()
            .take(1)
            .for_each(|scheduled_match| {
                let score = scheduled_match.result.as_mut().unwrap();

                let home_team = self.get_team(clubs, score.home.team_id);
                let away_team = self.get_team(clubs, score.away.team_id);

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

                scheduled_match.result = Some(LeagueMatchResultResult::new(&match_result.score.home_team, &match_result.score.away_team));

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

#[derive(Debug)]
pub struct LeagueSettings {
    pub season_starting_half: DayMonthPeriod,
    pub season_ending_half: DayMonthPeriod,
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
    use chrono::NaiveDate;

    #[test]
    fn test_is_time_for_new_schedule_true() {
        let settings = LeagueSettings {
            season_starting_half: DayMonthPeriod {
                from_day: 1,
                from_month: 1,
                to_day: 0,
                to_month: 0,
            },
            season_ending_half: DayMonthPeriod {
                from_day: 1,
                from_month: 7,
                to_day: 0,
                to_month: 0,
            },
        };

        let context = SimulationContext {
            date: NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(), // Season starting date
            // Add other fields as needed
            day: 0,
            hour: 0,
        };

        assert!(settings.is_time_for_new_schedule(&context));
    }

    #[test]
    fn test_is_time_for_new_schedule_false() {
        let settings = LeagueSettings {
            season_starting_half: DayMonthPeriod {
                from_day: 1,
                from_month: 1,
                to_day: 0,
                to_month: 0,
            },
            season_ending_half: DayMonthPeriod {
                from_day: 1,
                from_month: 7,
                to_day: 0,
                to_month: 0,
            },
        };

        let context = SimulationContext {
            date: NaiveDate::from_ymd_opt(2024, 7, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(), // Not season starting date
            // Add other fields as needed
            day: 0,
            hour: 0,
        };

        assert!(!settings.is_time_for_new_schedule(&context));
    }
}
