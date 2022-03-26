use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct PlayerTrainingHistoryItem {
    pub date: NaiveDateTime,
    pub rate: u8,
}

#[derive(Debug)]
pub struct PlayerTrainingHistory {
    data: Vec<PlayerTrainingHistoryItem>,
}

impl PlayerTrainingHistory {
    pub fn new() -> Self {
        PlayerTrainingHistory {
            data: Vec::with_capacity(100),
        }
    }

    pub fn add(&mut self, date: NaiveDateTime, rate: u8) {
        self.data.push(PlayerTrainingHistoryItem { date, rate })
    }
}
