use chrono::{NaiveDate, Duration};
use chrono::prelude::*;

pub struct DateUtils;

impl DateUtils{
    pub fn is_birthday(birth_date: NaiveDate, current_date: NaiveDate) -> bool{
        birth_date.month() == current_date.month() && birth_date.day() == current_date.day()
    }

    pub fn get_next_saturday(date: NaiveDate) -> NaiveDateTime {
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


}