use crate::models::club::Club;
use chrono::NaiveDate;

pub struct Schedule<'a> {
    pub items: Vec<ScheduleItem<'a>>,
}

impl<'a> Schedule<'a> {
    pub fn generate(clubs: &'a Vec<Club>) -> Result<Schedule, ()> {
        let mut schedule_items = Vec::with_capacity(clubs.len());

        let club_len = clubs.len();

        for idx in 0..club_len {
            let first_index = idx;
            let last_index = club_len - idx - 1;

            if first_index == last_index{
                continue;
            }
            
            let item = ScheduleItem {
                home_club: &clubs[first_index],
                guest_club: &clubs[last_index],
                date: NaiveDate::from_ymd(2019, 10, 1),
            };

            schedule_items.push(item);
        }

        let result = Schedule {
            items: schedule_items,
        };
        
        return Ok(result);
    }
}

pub struct ScheduleItem<'a> {
    pub date: NaiveDate,
    pub home_club: &'a Club,
    pub guest_club: &'a Club,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_is_correct() {
        let mut clubs = Vec::new();

        clubs.push(Club{
            name: "1".to_string(), 
            players: vec![]
        });

        clubs.push(Club{
            name: "2".to_string(), 
            players: vec![]
        });

        clubs.push(Club{
            name: "3".to_string(), 
            players: vec![]
        });

        let schedule = Schedule::generate(&clubs).unwrap();
        
        assert_eq!(6, schedule.items.len());
    }
}
