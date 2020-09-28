use crate::continent::{ContinentResult, Continent};
use chrono::{NaiveDateTime, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::transfers::TransferPool;
use crate::Player;
use crate::context::{GlobalContext, SimulationContext};

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        let ctx = GlobalContext::new(SimulationContext::new(data.date));

        let results: Vec<ContinentResult> = data.continents.iter_mut()
            .map(|continent| continent.simulate(ctx.with_continent(continent.id)))
            .collect();

        for result in results {
            result.process(data);
        }
        
        data.next_date();
    }
}


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
