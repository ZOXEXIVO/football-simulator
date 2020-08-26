use crate::club::Club;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use crate::league::{LeagueSettings, Season};
use super::DayMonthPeriod;

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

    pub fn generate(&mut self, season: Season, clubs: &[Club], tours_count: usize, league_settings: &LeagueSettings) {
        self.tours = Vec::with_capacity((clubs.len() / 2) * tours_count);

        let (season_year_start, season_year_end) = match season {
            Season::OneYear(year) => (year, year),
            Season::TwoYear(start_year, end_year) => (start_year, end_year)
        };

        for item in self.generate_for_period(&league_settings.season_starting_half, season_year_start, clubs, tours_count) {
            self.tours.push(item);
        }

        for item in self.generate_for_period(&league_settings.season_ending_half, season_year_end, clubs, tours_count) {
            self.tours.push(item);
        }
    }

    fn generate_for_period(&mut self, period: &DayMonthPeriod, year: u16, clubs: &[Club], tours_count: usize) -> Vec<ScheduleTour> {
        let mut current_date = ScheduleManager::get_nearest_saturday(
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

                result[current_tour].items.push(ScheduleItem::new(
                    current_date, club.id, inner_club.id)
                );

                current_tour += 1;
                current_tour %= tours_count;
            }

            current_date += Duration::days(7);
        }

        for tour_id in 1..result.len() {
            let current_tour = &mut result[tour_id].items;
        }

        result
    }

    pub fn start_new_tour(&mut self, date: NaiveDate) {
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
                                                  NaiveTime::from_hms(0, 0, 0),
        );

        loop {
            if current_date.weekday() == Weekday::Sat {
                break;
            }

            current_date += Duration::days(1)
        }

        current_date
    }

    pub fn set_goals(&mut self, id: &str, home_goals: u8, away_goals: u8) {
        let schedule_item = self.tours.iter_mut()
            .flat_map(|t| &mut t.items)
            .find(|s| s.id == id)
            .unwrap();

        schedule_item.home_goals = Some(home_goals);
        schedule_item.away_goals = Some(away_goals);
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
