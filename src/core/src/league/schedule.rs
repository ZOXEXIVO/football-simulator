use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use crate::league::{LeagueSettings, Season};
use crate::utils::DateUtils;
use log::{debug};

#[derive(Debug, Clone)]
pub struct ScheduleTour {
    pub num: u8,
    pub items: Vec<ScheduleItem>,
    pub played: bool,
}

impl ScheduleTour {
    pub fn new(num: u8, games_count: usize) -> Self {
        ScheduleTour {
            num,
            items: Vec::with_capacity(games_count),
            played: false,
        }
    }
}


#[derive(Debug)]
pub struct Schedule {
    pub tours: Vec<ScheduleTour>
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

#[derive(Debug, Clone)]
pub struct ScheduleItemResult{
    pub home_goals: u8,
    pub away_goals: u8,
}

impl ScheduleItem {
    pub fn new(league_id: u32, home_team_id: u32, away_team_id: u32, date: NaiveDateTime) -> Self {
        let id = format!("{}{}{}", date, home_team_id, away_team_id);

        ScheduleItem {
            id,
            league_id,
            date,
            home_team_id,
            away_team_id,

            result: None
        }
    }
}

const DAY_PLAYING_TIMES: [(u8, u8); 4] = [(13, 0), (14, 0), (16, 0), (18, 0)];

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            tours: Vec::new()
        }
    }

    pub fn exists(&self) -> bool {
        !self.tours.is_empty()
    }

    pub fn generate(&mut self, league_id: u32, season: Season, teams: &[u32], league_settings: &LeagueSettings) {
        debug!("schedule: generation for {:?}", season);
        
        let teams_len = teams.len();

        debug!("schedule: team_len = {}", teams_len);
        
        let tours_count = (teams_len * teams_len - teams_len) / (teams_len / 2);

        debug!("schedule: tours_count = {}", tours_count);
        
        self.tours = Vec::with_capacity((teams_len / 2) * tours_count);

        let (season_year_start, season_year_end) = match season {
            Season::OneYear(year) => (year, year),
            Season::TwoYear(start_year, end_year) => (start_year, end_year)
        };

        let current_date = DateUtils::get_next_saturday(
            NaiveDate::from_ymd(season_year_start as i32, league_settings.season_starting_half.from_month as u32, league_settings.season_starting_half.from_day as u32));

        for item in Self::generate_tours(league_id, teams, current_date) {
            self.tours.push(item);
        }
    }

    fn generate_tours(league_id: u32, teams: &[u32], mut current_date: NaiveDateTime) -> Vec<ScheduleTour> {
        let team_len= teams.len() as u32;
        let games_count = (team_len / 2) as usize;
        
        let tours_count = ((team_len * team_len - team_len) / (team_len / 2)) as usize;

        let mut result = Vec::with_capacity(tours_count);
        
        let mut games_offset = 0;
        
        let games = Self::generate_game_pairs(teams, tours_count);
        
        for tour in 1..tours_count {           
            let mut tour = ScheduleTour::new(tour as u8, games_count);
            
            for game_idx in 0..games_count {
                let (home_team_id, away_team_id) = games[games_offset + game_idx as usize];
                
                tour.items.push(ScheduleItem::new(league_id, home_team_id, away_team_id, current_date))
            }
            
            games_offset += games_count;
            current_date += Duration::days(7);
            
            result.push(tour);
        }

        result
    }
    
    fn generate_game_pairs(clubs: &[u32], tours_count: usize) -> Vec<(u32, u32)> {
        let mut result = Vec::new();

        let mut temp_vec = Vec::new();

        let team_len= clubs.len() as u32;
        let team_len_half = team_len / 2 as u32;

        for club in 1..team_len_half + 1 {
            temp_vec.push((club, team_len - club + 1))
        }

        for club in &temp_vec {
            result.push((club.0, club.1));
        }

        for _ in 0..tours_count {
            Self::rotate(&mut temp_vec);

            for club in &temp_vec {
                result.push((club.0, club.1));
            }
        }

        result
    }

    fn rotate(clubs: &mut Vec<(u32, u32)>){
        let teams_len = clubs.len();

        let right_top = clubs[0].1;
        let left_bottom = clubs[teams_len - 1].0;

        for i in 0..teams_len - 1{
            clubs[i].1 =  clubs[i + 1].1;
            clubs[teams_len-i-1].0 =  clubs[teams_len-i - 2].0;
        }

        clubs[0].0 = right_top;
        clubs[teams_len - 1].1 = left_bottom;
    }
    
    pub fn update_match_result(&mut self, id: &str, home_goals: u8, away_goals: u8) {
        for tour in &mut self.tours {
            if tour.played {
                continue;
            }
            
            if let Some(item) = tour.items.iter_mut().find(|i| i.id == id) {
                item.result = Some(ScheduleItemResult {
                    home_goals,
                    away_goals
                });
                
                if tour.items.iter().all(|i| i.result.is_some()) {
                    tour.played = true;
                }
            }
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
                    s.date
                )
            })
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
