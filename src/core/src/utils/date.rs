use chrono::prelude::*;
use chrono::{Duration, NaiveDate};

pub struct DateUtils;

impl DateUtils {
    #[inline]
    pub fn is_birthday(birth_date: NaiveDate, current_date: NaiveDate) -> bool {
        birth_date.year() == current_date.year() && birth_date.ordinal() == current_date.ordinal()
    }

    #[inline]
    pub fn age(birthdate: NaiveDate, now: NaiveDate) -> u8 {
        let age_duration = now.signed_duration_since(birthdate);
        (age_duration.num_days() / 365) as u8
    }

    pub fn next_saturday(date: NaiveDate) -> NaiveDate {
        let mut current_date = date;

        while current_date.weekday() != Weekday::Sat {
            current_date = current_date.succ();
        }

        current_date
    }
}
