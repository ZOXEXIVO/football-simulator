use chrono::NaiveDate;
use chrono::prelude::*;

pub struct DateUtils;

impl DateUtils{
    pub fn is_birthday(birth_date: NaiveDate, current_date: NaiveDate) -> bool{
        birth_date.month() == current_date.month() && birth_date.day() == current_date.day()
    }
}