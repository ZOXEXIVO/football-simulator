use crate::continent::Continent;
use chrono::{NaiveDateTime, Duration};

use rand::Rng;
use rand::distributions::Alphanumeric;

pub struct SimulatorData {
    pub id: String,
    
    pub continents: Vec<Continent>,

    pub date: NaiveDateTime,
}

impl SimulatorData {
    pub fn next_date(&mut self) {
        self.date += Duration::hours(1);
    }
    
    pub fn id(&self) -> String {
        self.id.clone()
    }
     
    pub fn generate_id() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect::<String>()
    }
}