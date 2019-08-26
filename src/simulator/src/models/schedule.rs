use crate::models::club::Club;
use chrono::NaiveDate;

pub struct Schedule {
    items: Vec<ScheduleItem>
}

impl Schedule{
    pub fn new(club_count: usize) -> Schedule{
        let max_schedule_items = club_count * 3;

        Schedule { items: Vec::with_capacity(max_schedule_items)}
    }
}

pub struct ScheduleItem{
    date: NaiveDate,
    home: Club,
    guest: Club
}