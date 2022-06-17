use chrono::prelude::*;
use chrono::{Duration, NaiveDate};

pub struct DateUtils;

impl DateUtils {
    pub fn is_birthday(birth_date: NaiveDate, current_date: NaiveDate) -> bool {
        birth_date.month() == current_date.month() && birth_date.day() == current_date.day()
    }

    pub fn next_saturday(date: NaiveDate) -> NaiveDateTime {
        let mut current_date = NaiveDateTime::new(date, NaiveTime::from_hms(0, 0, 0));

        while current_date.weekday() != Weekday::Sat {
            current_date += Duration::days(1)
        }

        current_date
    }
}
