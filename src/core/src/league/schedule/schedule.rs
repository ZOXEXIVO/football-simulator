use crate::context::GlobalContext;
use crate::league::round::RoundSchedule;
use crate::league::{LeagueMatch, LeagueSettings, ScheduleGenerator, ScheduleResult, Season};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use log::{debug, error};

#[derive(Debug)]
pub struct Schedule {
    pub tours: Vec<ScheduleTour>,
}

impl Schedule {
    pub fn new() -> Self {
        Schedule { tours: Vec::new() }
    }

    pub fn simulate(
        &mut self,
        league_settings: &LeagueSettings,
        ctx: GlobalContext<'_>,
    ) -> ScheduleResult {
        let mut result = ScheduleResult::new();

        if self.tours.is_empty() || league_settings.is_time_for_new_schedule(&ctx.simulation) {
            let league_ctx = ctx.league.as_ref().unwrap();

            let generator = RoundSchedule::new();

            match generator.generate(
                league_ctx.id,
                Season::OneYear(ctx.simulation.date.year() as u16),
                league_ctx.team_ids,
                league_settings,
            ) {
                Ok(generated_schedule) => {
                    self.tours = generated_schedule;
                    result.generated = true;
                }
                Err(error) => {
                    error!("Generating schedule error: {}", error.message);
                }
            }
        }

        result.scheduled_matches = self
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

        result
    }

    pub fn get_matches(&self, date: NaiveDateTime) -> Vec<ScheduleItem> {
        self.tours
            .iter()
            .flat_map(|t| &t.items)
            .filter(|s| s.date == date)
            .map(|s| ScheduleItem::new(s.league_id, s.home_team_id, s.away_team_id, s.date, None))
            .collect()
    }

    pub fn get_matches_for_team(&self, team_id: u32) -> Vec<ScheduleItem> {
        self.tours
            .iter()
            .flat_map(|t| &t.items)
            .filter(|s| s.home_team_id == team_id || s.away_team_id == team_id)
            .map(|s| {
                let res = match &s.result {
                    Some(result) => Some(ScheduleItemResult {
                        home_goals: result.home_goals,
                        away_goals: result.away_goals,
                    }),
                    None => None,
                };

                ScheduleItem::new(s.league_id, s.home_team_id, s.away_team_id, s.date, res)
            })
            .collect()
    }

    pub fn update_match_result(&mut self, id: &str, home_goals: u8, away_goals: u8) {
        let mut updated = false;

        for tour in &mut self.tours.iter_mut().filter(|t| !t.played()) {
            if let Some(item) = tour.items.iter_mut().find(|i| i.id == id) {
                item.result = Some(ScheduleItemResult {
                    home_goals,
                    away_goals,
                });

                updated = true;
            }
        }

        match updated {
            true => {
                debug!(
                    "update match result, schedule_id={}, {}:{}",
                    id, home_goals, away_goals
                );
            }
            _ => {
                debug!(
                    "match result not updated, schedule_id={}, {}:{}",
                    id, home_goals, away_goals
                );
            }
        }
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule { tours: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleError {
    pub message: String,
}

impl ScheduleError {
    pub fn from_str(str: &'static str) -> Self {
        ScheduleError {
            message: str.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub id: String,
    pub league_id: u32,

    pub date: NaiveDateTime,

    pub home_team_id: u32,
    pub away_team_id: u32,

    pub result: Option<ScheduleItemResult>,
}

impl ScheduleItem {
    pub fn new(
        league_id: u32,
        home_team_id: u32,
        away_team_id: u32,
        date: NaiveDateTime,
        result: Option<ScheduleItemResult>,
    ) -> Self {
        let id = format!("{}_{}_{}", date.date(), home_team_id, away_team_id);

        ScheduleItem {
            id,
            league_id,
            date,
            home_team_id,
            away_team_id,

            result,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleItemResult {
    pub home_goals: u8,
    pub away_goals: u8,
}

#[derive(Debug, Clone)]
pub struct ScheduleTour {
    pub num: u8,
    pub items: Vec<ScheduleItem>,
}

impl ScheduleTour {
    pub fn new(num: u8, games_count: usize) -> Self {
        ScheduleTour {
            num,
            items: Vec::with_capacity(games_count),
        }
    }

    pub fn played(&self) -> bool {
        self.items.iter().all(|i| i.result.is_some())
    }

    pub fn start_date(&self) -> NaiveDate {
        self.items
            .iter()
            .min_by_key(|t| t.date)
            .unwrap()
            .date
            .date()
    }

    pub fn end_date(&self) -> NaiveDate {
        self.items
            .iter()
            .max_by_key(|t| t.date)
            .unwrap()
            .date
            .date()
    }
}
