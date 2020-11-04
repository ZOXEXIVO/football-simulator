use crate::club::{MatchHistory};
use crate::league::{ScheduleManager, LeagueResult, LeagueTable, Season};
use crate::r#match::{Match, MatchResult};
use crate::context::{GlobalContext, SimulationContext};
use chrono::Datelike;
use crate::{Club, ClubResult};

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub clubs: Vec<Club>,
    pub schedule_manager: ScheduleManager,
    pub settings: LeagueSettings,
    pub league_table: LeagueTable,
    pub reputation: u16,
}

impl League {
    pub fn new(id: u32, name: String, reputation: u16, settings: LeagueSettings, clubs: Vec<Club>) -> Self {
        let club_headers = clubs.iter().map(|c| c.id).collect();
        
        League {
            id,
            name,
            clubs,
            schedule_manager: ScheduleManager::new(),
            settings,
            league_table: LeagueTable::new(club_headers),
            reputation,
        }
    }
    
    pub fn simulate(&mut self, ctx: GlobalContext) -> LeagueResult {
        if !self.schedule_manager.exists() || self.settings.is_time_for_new_schedule(&ctx.simulation) {
            self.schedule_manager.generate(Season::TwoYear(2020, 2021), &self.teams,  &self.settings);
        }

        let match_results = self.play_matches(&ctx);

        self.league_table.update(&match_results);

        let clubs_results: Vec<ClubResult> = self.clubs.iter_mut()
            .map(|club| club.simulate(ctx.with_club(club.id)))
            .collect();

        LeagueResult::new(clubs_results, match_results)
    }

    fn play_matches(&mut self, context: &GlobalContext) -> Vec<MatchResult> {
        let current_date = context.simulation.date;
    
        let matches: Vec<Match> = {
            self.schedule_manager.get_matches(current_date)
                .iter()
                .map(|m| {
                    Match::make(&m.id,
                                self.get_team(&m.home_team_id),
                                self.get_team(&m.away_team_id),
                    )
                }).collect()
        };
    
        let match_results: Vec<MatchResult> = matches.into_iter().map(|game| game.play()).collect();
    
        for match_result in &match_results {
            self.schedule_manager.update_match_result(&match_result.schedule_id, match_result.home_goals, match_result.away_goals);
    
            self.add_match_to_team_history(match_result.home_team_id,
                MatchHistory::new(
                    current_date, match_result.away_team_id, 
                    (match_result.home_goals, match_result.away_goals)),
            );
    
            self.add_match_to_team_history(match_result.away_team_id,
                MatchHistory::new(
                    current_date, match_result.home_team_id, 
                    (match_result.away_goals, match_result.home_goals)),
            );
        }
    
        match_results
    }

    fn add_match_to_team_history(&mut self, team_id: u32, history: MatchHistory) {
              //club.match_history.push(history);
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
    pub to_month: u8
}

impl DayMonthPeriod {
    pub fn new(from_day: u8, from_month: u8, to_day: u8, to_month: u8) -> Self {
        DayMonthPeriod {
            from_day,
            from_month,
            to_day,
            to_month
        }
    }
}

impl LeagueSettings {
    pub fn is_time_for_new_schedule(&self, context: &SimulationContext) -> bool {
        let season_starting_date = &self.season_starting_half;
        
        let date = context.date.date();

        (date.day() as u8) == season_starting_date.from_day && (date.month() as u8) == season_starting_date.from_month
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
