use chrono::{NaiveDateTime, NaiveTime};

#[derive(Debug)]
pub struct TrainingSchedule {
    pub morning_time: NaiveTime,
    pub evening_time: NaiveTime,
}

impl TrainingSchedule {
    pub fn new(morning_time: NaiveTime, evening_time: NaiveTime) -> Self {
        TrainingSchedule {
            morning_time,
            evening_time,
        }
    }

    pub fn is_time(&self, date: NaiveDateTime) -> bool {
        self.morning_time == date.time() || self.evening_time == date.time()
    }
}
