use crate::club::Club;
use chrono::NaiveDate;

pub struct Schedule {
    pub items: Vec<ScheduleItem>
}

impl Schedule {
    pub fn generate(clubs: &Vec<Club>) -> Result<Schedule, ()> {
        let mut schedule_items = Vec::with_capacity(clubs.len());

        let club_len = clubs.len();

        for idx in 0..club_len {
            let first_index = idx;
            let last_index = club_len - idx - 1;

            if first_index == last_index{
                continue;
            }
            
            let item = ScheduleItem {
                home_club_id: clubs[first_index].id,
                guest_club_id: clubs[last_index].id,
                date: NaiveDate::from_ymd(2019, 10, 1),
            };

            schedule_items.push(item);
        }

        let result = Schedule {
            items: schedule_items,
        };
        
        return Ok(result);
    }

    pub fn get_matches(&self, date: NaiveDate) -> Vec<&ScheduleItem>{
        self.items.iter().filter(|x| x.date == date).collect()
    }
}

#[derive(Clone, Copy)]
pub struct ScheduleItem {
    pub date: NaiveDate,
    pub home_club_id: u32,
    pub guest_club_id: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_is_correct() {
        let mut clubs = Vec::new();

        clubs.push(Club{
            id: 1,
            name: "1".to_string(), 
            players: vec![],
            staffs: vec![]
        });

        clubs.push(Club{
            id: 2,
            name: "2".to_string(), 
            players: vec![],
            staffs: vec![]
        });

        clubs.push(Club{
            id: 3,
            name: "3".to_string(), 
            players: vec![],
            staffs: vec![]
        });

        let schedule = Schedule::generate(&clubs).unwrap();

        assert_eq!(2, schedule.items.len());
    }
}
