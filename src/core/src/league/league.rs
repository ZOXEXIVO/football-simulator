use crate::league::{ScheduleManager, LeagueResult, LeagueTable, Season, LeagueMatchResult};
use crate::context::{GlobalContext, SimulationContext};
use chrono::Datelike;

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub country_id: u32,
    pub schedule_manager: ScheduleManager,
    pub table: LeagueTable,
    pub settings: LeagueSettings,
    pub reputation: u16,
}

impl League {
    pub fn new(id: u32, name: String, country_id: u32, reputation: u16, settings: LeagueSettings) -> Self {
        League {
            id,
            name,
            country_id,
            schedule_manager: ScheduleManager::new(),
            table: LeagueTable::empty(),
            settings,
            reputation,
        }
    }
    
    pub fn simulate(&mut self, ctx: GlobalContext) -> LeagueResult {
        if !self.schedule_manager.exists() || self.settings.is_time_for_new_schedule(&ctx.simulation) {
            self.schedule_manager.generate(Season::TwoYear(2020, 2021), &Vec::new(),  &self.settings);
        }

        let scheduled_matches  = 
            self.schedule_manager.get_matches(ctx.simulation.date)
                .iter()
                .map(|sm| 
                    LeagueMatchResult {
                        schedule: &sm,
                        home_goals: sm.home_goals,
                        away_goals: sm.away_goals
                    }
                ).collect();

        LeagueResult::new(self.id, scheduled_matches)
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
