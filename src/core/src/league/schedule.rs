use crate::club::Club;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use crate::league::{LeagueSettings, Season};
use super::DayMonthPeriod;
use std::collections::{HashMap};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::utils::DateUtils;

#[derive(Debug, Clone)]
pub struct ScheduleTour {
    pub items: Vec<ScheduleItem>,
    pub played: bool,
}

impl ScheduleTour {
    pub fn new(games_count: usize) -> Self {
        ScheduleTour {
            items: Vec::with_capacity(games_count),
            played: false,
        }
    }
}


#[derive(Debug)]
pub struct ScheduleManager {
    pub tours: Vec<ScheduleTour>
}

#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub id: String,
    pub date: NaiveDateTime,

    pub home_club_id: u32,
    pub away_club_id: u32,

    pub home_goals: Option<u8>,
    pub away_goals: Option<u8>,
}

impl ScheduleItem {
    pub fn new(date: NaiveDateTime, home_club_id: u32, away_club_id: u32) -> Self {
        let id = format!("{}{}{}", date, home_club_id, away_club_id);

        ScheduleItem {
            id,
            date,
            home_club_id,
            away_club_id,

            home_goals: None,
            away_goals: None,
        }
    }
}

impl Default for ScheduleItem {
    fn default() -> Self {
        ScheduleItem{
            id: "".to_string(),
            date: NaiveDateTime::from_timestamp(0, 0),
            home_club_id: 0,
            away_club_id: 0,
            home_goals: None,
            away_goals: None
        }
    }
}

const DAY_PLAYING_TIMES: [(u8, u8); 4] = [(13, 0), (14, 0), (16, 0), (18, 0)];

impl ScheduleManager {
    pub fn new() -> Self {
        ScheduleManager {
            tours: Vec::new()
        }
    }

    pub fn exists(&self) -> bool {
        !self.tours.is_empty()
    }

    pub fn generate(&mut self, season: Season, clubs: &[Club], league_settings: &LeagueSettings) {
        let clubs_len = clubs.len();
        
        let tours_count = (clubs_len * clubs_len - clubs_len) / (clubs_len / 2);

        self.tours = Vec::with_capacity((clubs_len / 2) * tours_count);

        let (season_year_start, season_year_end) = match season {
            Season::OneYear(year) => (year, year),
            Season::TwoYear(start_year, end_year) => (start_year, end_year)
        };

        let mut club_ids: Vec<u32> = clubs.iter().map(|c| c.id).collect();

        for item in self.generate_for_period(&league_settings.season_starting_half, season_year_start, &club_ids, tours_count / 2) {
            self.tours.push(item);
        }

        club_ids.reverse();
        
        for item in self.generate_for_period(&league_settings.season_ending_half, season_year_end, &club_ids, tours_count / 2) {
            self.tours.push(item);
        }
    }

    fn generate_for_period(&mut self, period: &DayMonthPeriod, year: u16, club_ids: &[u32], tours_generate_count: usize) -> Vec<ScheduleTour> {
        let mut current_date = DateUtils::get_next_saturday(
            NaiveDate::from_ymd(year as i32, period.from_month as u32, period.from_day as u32));

        let club_len = club_ids.len();
        let club_half_len = club_len / 2;
        
        let items_count = (club_len / 2) * tours_generate_count;

        let mut result = Vec::with_capacity(items_count);

        for _ in 0..tours_generate_count {
            result.push(ScheduleTour::new(club_half_len))
        }

        for tour in 0..tours_generate_count {
            let mut rival_map = HashMap::with_capacity(club_half_len);

            let current_tour = &mut result[tour];

            println!("fill tour {}",  tour);
            
            for club_idx in 0..club_half_len {
                let rival_idx = rival_map.entry(club_idx).or_insert(club_half_len + club_idx);
             
                println!("club_idx = {}, rival_idx = {}",  club_idx, *rival_idx);

                if club_idx == *rival_idx {
                    continue;    
                }
                                
                let home_club_id = club_ids[club_idx];
                let away_club_id = club_ids[*rival_idx];

                current_tour.items.push(ScheduleItem::new(
                    current_date, home_club_id, away_club_id));

                *rival_idx += 1;
                *rival_idx %= club_half_len;
            }

            current_tour.items.shuffle(&mut thread_rng());
            
            current_date += Duration::days(7);
        }
        
        result
    }

    
    pub fn update_match_result(&mut self, id: &str, home_goals: u8, away_goals: u8) {
        for tour in &mut self.tours {
            if tour.played {
                continue;
            }
            
            if let Some(item) = tour.items.iter_mut().find(|i| i.id == id) {
                item.home_goals = Some(home_goals);
                item.away_goals = Some(away_goals);
                
                if tour.items.iter().all(|i| i.home_goals.is_some() && i.away_goals.is_some()) {
                    tour.played = true;
                }
            }
        }
    }

    pub fn get_matches(&self, date: NaiveDateTime) -> Vec<&ScheduleItem> {
        self.tours.iter()
            .flat_map(|t| &t.items)
            .filter(|s| s.date == date)
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::club::ClubMood;
    use crate::ClubBoard;

    #[test]
    fn generate_is_correct() {
        //let mut clubs = Vec::new();
        //
        // clubs.push(Club {
        //     id: 1,
        //     name: "1".to_string(),
        //     mood: ClubMood::default(),
        //     board: ClubBoard::new(),
        //     players: PlayerCollection::new(vec![]),
        //     staffs: StaffCollection::new(vec![]),
        //     tactics: None,
        //     transfer_list: vec![],
        //     match_history: vec![]
        // });
        //
        // clubs.push(Club {
        //     id: 2,
        //     name: "1".to_string(),
        //     mood: ClubMood::default(),
        //     board: ClubBoard::new(),
        //     players: PlayerCollection::new(vec![]),
        //     staffs: StaffCollection::new(vec![]),
        //     tactics: None,
        //     transfer_list: vec![],
        //     match_history: vec![]
        // });
        //
        // clubs.push(Club {
        //     id: 3,
        //     name: "1".to_string(),
        //     mood: ClubMood::default(),
        //     board: ClubBoard::new(),
        //     players: PlayerCollection::new(vec![]),
        //     staffs: StaffCollection::new(vec![]),
        //     tactics: None,
        //     transfer_list: vec![],
        //     match_history: vec![]
        // });
        //
        // let schedule = Schedule::generate(&clubs, NaiveDate::from_ymd(2020, 3, 1)).unwrap();

        //sassert_eq!(2, schedule.items.len());
    }
}
