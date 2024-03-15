use chrono::prelude::*;
use chrono::NaiveDate;

pub struct DateUtils;

impl DateUtils {
    #[inline]
    pub fn is_birthday(birth_date: NaiveDate, current_date: NaiveDate) -> bool {
        birth_date.month() == current_date.month() && birth_date.day() == current_date.day()
    }

    #[inline]
    pub fn age(birthdate: NaiveDate, now: NaiveDate) -> u8 {
        let age_duration = now.signed_duration_since(birthdate);
        (age_duration.num_days() / 365) as u8
    }

    pub fn next_saturday(date: NaiveDate) -> NaiveDate {
        let mut current_date = date;

        while current_date.weekday() != Weekday::Sat {
            current_date = current_date.succ_opt().unwrap();
        }

        current_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_birthday() {
        let birth_date = NaiveDate::from_ymd(1990, 3, 16);
        let current_date = NaiveDate::from_ymd(2024, 3, 16);
        assert!(DateUtils::is_birthday(birth_date, current_date));

        let birth_date = NaiveDate::from_ymd(1990, 3, 16);
        let current_date = NaiveDate::from_ymd(2024, 3, 17);
        assert!(!DateUtils::is_birthday(birth_date, current_date));
    }

    #[test]
    fn test_age() {
        let birth_date = NaiveDate::from_ymd(1990, 3, 16);
        let current_date = NaiveDate::from_ymd(2024, 3, 16);

        assert_eq!(DateUtils::age(birth_date, current_date), 34);

        let birth_date = NaiveDate::from_ymd(1990, 3, 16);
        let current_date = NaiveDate::from_ymd(2024, 3, 15);

        assert_eq!(DateUtils::age(birth_date, current_date), 34);
    }

    #[test]
    fn test_next_saturday() {
        let date = NaiveDate::from_ymd(2024, 3, 12); // A Tuesday
        let next_saturday = DateUtils::next_saturday(date);

        assert_eq!(next_saturday, NaiveDate::from_ymd(2024, 3, 16));

        let date = NaiveDate::from_ymd(2024, 3, 17); // A Saturday
        let next_saturday = DateUtils::next_saturday(date);

        assert_eq!(next_saturday, NaiveDate::from_ymd(2024, 3, 23));

        let date = NaiveDate::from_ymd(2024, 3, 18); // A Sunday
        let next_saturday = DateUtils::next_saturday(date);

        assert_eq!(next_saturday, NaiveDate::from_ymd(2024, 3, 23));
    }
}
