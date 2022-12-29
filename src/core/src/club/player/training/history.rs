use crate::PlayerSkills;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct PlayerTrainingHistory {
    records: Vec<TrainingRecord>,
}

impl PlayerTrainingHistory {
    pub fn new() -> Self {
        PlayerTrainingHistory {
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, record: TrainingRecord) {
        self.records.push(record);
    }

    pub fn get_latest_record(&self) -> Option<&TrainingRecord> {
        self.records.last()
    }
}

#[derive(Debug)]
pub struct TrainingRecord {
    date: NaiveDateTime,
    skills: PlayerSkills,
}

impl TrainingRecord {
    fn new(date: NaiveDateTime, skills: PlayerSkills) -> Self {
        TrainingRecord { date, skills }
    }
}
