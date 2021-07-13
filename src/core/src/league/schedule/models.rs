use chrono::{NaiveDateTime, NaiveDate};
use log::{debug};

#[derive(Debug)]
pub struct Schedule {
    pub tours: Vec<ScheduleTour>
}

impl Schedule {
    pub fn with_tours_capacity(capacity: usize) -> Self {
        Schedule {
            tours: Vec::with_capacity(capacity)
        }
    }
    
    pub fn stub() -> Self {
        Schedule {
            tours: Vec::new()
        }
    }

    pub fn get_matches(&self, date: NaiveDateTime) -> Vec<ScheduleItem> {
        self.tours.iter()
            .flat_map(|t| &t.items)
            .filter(|s| s.date == date)
            .map(|s| {
                ScheduleItem::new(
                    s.league_id,
                    s.home_team_id,
                    s.away_team_id,
                    s.date,
                    None
                )
            })
            .collect()
    }

    pub fn get_matches_for_team(&self, team_id: u32) -> Vec<ScheduleItem> {
        self.tours.iter()
            .flat_map(|t| &t.items)
            .filter(|s| s.home_team_id == team_id || s.away_team_id == team_id)
            .map(|s| {
                let res = match &s.result {
                    Some(result) => Some(ScheduleItemResult {
                        home_goals: result.home_goals,
                        away_goals: result.away_goals
                    }),
                    None => None
                };
                
                ScheduleItem::new(
                    s.league_id,
                    s.home_team_id,
                    s.away_team_id,
                    s.date,
                    res
                )
            })
            .collect()
    }

    pub fn update_match_result(&mut self, id: &str, home_goals: i32, away_goals: i32) {        
        let mut updated = false;

        for tour in &mut self.tours.iter_mut().filter(|t| !t.played()) {            
            if let Some(item) = tour.items.iter_mut().find(|i| i.id == id) {                
                item.result = Some(ScheduleItemResult {
                    home_goals,
                    away_goals
                });

                updated = true;
            }
        }

        match updated {
            true => {
                debug!("update match result, schedule_id={}, {}:{}", id, home_goals, away_goals);
            }
            _ => {
                debug!("match result not updated, schedule_id={}, {}:{}", id, home_goals, away_goals);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleError {
    pub message: String
}

impl ScheduleError {
    pub fn from_str(str: &'static str) -> Self {
        ScheduleError {
            message: str.to_owned()
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

    pub result: Option<ScheduleItemResult>
}

impl ScheduleItem {
    pub fn new(league_id: u32, home_team_id: u32, away_team_id: u32, date: NaiveDateTime, result: Option<ScheduleItemResult>) -> Self {
        let id = format!("{}_{}_{}", date.date(), home_team_id, away_team_id);

        ScheduleItem {
            id,
            league_id,
            date,
            home_team_id,
            away_team_id,

            result
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleItemResult{
    pub home_goals: i32,
    pub away_goals: i32,
}

#[derive(Debug, Clone)]
pub struct ScheduleTour {
    pub num: u8,
    pub items: Vec<ScheduleItem>
}

impl ScheduleTour {
    pub fn new(num: u8, games_count: usize) -> Self {
        ScheduleTour {
            num,
            items: Vec::with_capacity(games_count)
        }
    }

    pub fn played(&self) -> bool {
        self.items.iter().all(|i| i.result.is_some())
    }

    pub fn start_date(&self) -> NaiveDate {
        self.items.iter().min_by_key(|t| t.date).unwrap().date.date()
    }

    pub fn end_date(&self) -> NaiveDate {
        self.items.iter().max_by_key(|t| t.date).unwrap().date.date()
    }
}