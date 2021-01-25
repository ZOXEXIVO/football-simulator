use crate::league::{Schedule, LeagueResult, LeagueTable, Season, LeagueMatch, RoundSchedule, ScheduleGenerator, LeagueContext};
use crate::context::{GlobalContext, SimulationContext};
use chrono::Datelike;
use log::{error};

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub country_id: u32,
    pub schedule: Option<Schedule>,
    pub table: Option<LeagueTable>,
    pub settings: LeagueSettings,
    pub reputation: u16,
}

impl League {
    pub fn new(id: u32, name: String, country_id: u32, reputation: u16, settings: LeagueSettings) -> Self {
        League {
            id,
            name,
            country_id,
            schedule: None,
            table: Option::None,
            settings,
            reputation,
        }
    }
    
    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> LeagueResult {
        let league_ctx = ctx.league.as_ref().unwrap();
        
        if self.table.is_none() {
            self.table = Some(LeagueTable::with_clubs(&league_ctx.team_ids));
        }
        
        let scheduled_matches = self.simulate_schedule(&ctx);

        LeagueResult::new(self.id, scheduled_matches)
    }

    fn simulate_schedule(&mut self, ctx: &GlobalContext<'_>) -> Vec<LeagueMatch> {
        if self.schedule.is_none() || self.settings.is_time_for_new_schedule(&ctx.simulation) {
            let schedule_generator = self.get_schedule_generator();

            let league_ctx = ctx.league.as_ref().unwrap();

            match schedule_generator.generate(self.id,Season::OneYear(2021), league_ctx.team_ids, &self.settings) {
                Ok(generated_schedule) => {
                    self.schedule = Some(generated_schedule);
                },
                Err(error) => {
                    error!("Generating schedule error: {}", error.message);
                }
            }
        }

        let scheduled_matches  =
            self.schedule.as_ref().unwrap().get_matches(ctx.simulation.date)
                .iter()
                .map(|sm|
                    LeagueMatch {
                        id: sm.id.clone(),
                        league_id: sm.league_id,
                        date: sm.date,
                        home_team_id: sm.home_team_id,
                        away_team_id: sm.away_team_id,
                        result: None
                    }
                ).collect();

        scheduled_matches
    }
    
    fn get_schedule_generator(&self) -> impl ScheduleGenerator {
        RoundSchedule::new()
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
