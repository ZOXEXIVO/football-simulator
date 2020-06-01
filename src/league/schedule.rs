use crate::club::Club;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Schedule {
    pub items: Vec<ScheduleItem>,
    pub current_tour: Option<Tour>
}

#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub date: NaiveDate,
    pub home_club_id: u32,
    pub guest_club_id: u32,
}

#[derive(Debug)]
pub struct Tour {
    pub games: Vec<ScheduleItem>
}

impl Schedule {
    pub fn start_new_tour(&mut self, date: NaiveDate){
        let mut current_week_games = Vec::with_capacity(30);
        
        for day in 0..7 {
            let filter_date = date + Duration::days(day);

            for day_game in self.items.iter().filter(|s| s.date == filter_date) {
                current_week_games.push(day_game.clone())
            }           
        }       
        
        self.current_tour = Some(Tour::new(current_week_games));
    }
    
    pub fn generate(clubs: &[Club], date: NaiveDate) -> Result<Schedule, ()> {
        let club_len = clubs.len();

        let mut schedule_items = Vec::with_capacity(club_len * 2);

        let mut starting_date = date;

        for _odx in 0..club_len {
            for idx in 0..club_len / 2 {
                let first_index = idx;
                let last_index = club_len - idx - 1;

                if first_index == last_index {
                    continue;
                }

                if starting_date.weekday() == Weekday::Sat {}

                let item = ScheduleItem {
                    home_club_id: clubs[first_index].id,
                    guest_club_id: clubs[last_index].id,
                    date: starting_date,
                };

                schedule_items.push(item);
            }

            starting_date = starting_date.checked_add_signed(Duration::days(7)).unwrap();
        }

        Ok(Schedule {
            items: schedule_items,
            current_tour: None
        })
    }

    pub fn get_matches(&self, date: NaiveDate) -> Vec<&ScheduleItem> {
        self.items.iter().filter(|x| x.date == date).collect()
    }
}

impl Tour {
    pub fn new(games: Vec<ScheduleItem>) -> Self {
        Tour {
            games
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::club::ClubMood;
    use crate::people::{PlayerCollection, StaffCollection};
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
