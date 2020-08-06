use crate::club::Club;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use crate::league::LeagueSettings;
use super::DayMonthPeriod;

#[derive(Debug, Clone)]
pub struct ScheduleTour {
    pub items: Vec<ScheduleItem>,
    pub played: bool
}

impl ScheduleTour{ 
    pub fn new(games_count: usize) -> Self {
        ScheduleTour {
            items: Vec::with_capacity(games_count),
            played: false
        }
    }
}

#[derive(Debug)]
pub struct ScheduleManager {
    pub tours: Vec<ScheduleTour>
}

#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub date: NaiveDateTime,
    pub home_club_id: u32,
    pub away_club_id: u32,
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
    
    pub fn generate(&mut self, year: u16, clubs: &[Club], tours_count: usize, league_settings: &LeagueSettings) {
        self.tours = Vec::with_capacity( (clubs.len() / 2) * tours_count);
        
        for item in self.generate_for_period(&league_settings.season_starting_half, year, clubs, tours_count){
            self.tours.push(item);
        }

        for item in self.generate_for_period(&league_settings.season_ending_half, year, clubs, tours_count){
            self.tours.push(item);
        }
    }
    
    fn generate_for_period(&mut self, period: &DayMonthPeriod, year: u16, clubs: &[Club], tours_count: usize) -> Vec<ScheduleTour> {
        let current_date = ScheduleManager::get_nearest_saturday(
            NaiveDate::from_ymd(year as i32, period.from_month as u32, period.from_day as u32));

        let items_count = (clubs.len() / 2) * tours_count;
        
        let mut result = Vec::with_capacity(items_count);

        for _ in 0..tours_count {
            result.push(ScheduleTour::new(clubs.len() / 2))
        }
        
        let mut current_tour = 0;

        for (idx, club) in clubs.iter().enumerate() {

            for (inner_idx, inner_club) in clubs.iter().enumerate() {
    
                if club.id == inner_club.id {
                    continue;
                }
                
                let item = ScheduleItem {
                    home_club_id: club.id,
                    away_club_id: inner_club.id,
                    date: current_date
                };

                result[current_tour].items.push(item);
                
                current_tour += 1;
                current_tour %= tours_count;
            }
        }
        
        for tour_id in 1..result.len(){
            let current_tour = &mut result[tour_id].items;
        }
        
        result
    }
    
    pub fn start_new_tour(&mut self, date: NaiveDate){
        //let mut current_week_games = Vec::with_capacity(30);

        let start_date = date;
        let end_date = date + Duration::days(7);

        // for day_game in self.tours.iter().filter(|s| s.date.date() >= start_date && s.date.date() <= end_date) {
        //     current_week_games.push(day_game.clone())
        // }
    }

    fn get_nearest_saturday(date: NaiveDate) -> NaiveDateTime {
        let mut current_date = NaiveDateTime::new(NaiveDate::from_ymd(
            date.year(), date.month() as u32, date.day() as u32),
            NaiveTime::from_hms(0, 0, 0)
        );
        
        loop {
            if current_date.weekday() == Weekday::Sat {
                break;
            }

            current_date += Duration::days(1)
        }

        current_date
    }
    
    pub fn get_matches(&self, date: NaiveDateTime) -> Vec<&ScheduleItem> {
        let mut result = Vec::new();
        
        for tour in &self.tours {
            for item in &tour.items {
                if item.date == date {
                    result.push(item);
                }
            }
        }
        
        result
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
