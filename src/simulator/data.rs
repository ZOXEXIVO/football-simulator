use crate::continent::Continent;
use chrono::{NaiveDateTime, Duration};

use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::transfers::TransferPool;
use crate::club::Player;

pub struct SimulatorData {
    pub id: String,
    
    pub continents: Vec<Continent>,

    pub date: NaiveDateTime,
    
    pub transfer_pool: TransferPool<Player>
}

impl SimulatorData {
    pub fn next_date(&mut self) {
        self.date += Duration::days(1);
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