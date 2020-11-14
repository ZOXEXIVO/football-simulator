use crate::continent::{ContinentResult, Continent};
use chrono::{NaiveDateTime, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::transfers::TransferPool;
use crate::{Player, Country};
use crate::context::{GlobalContext, SimulationContext};
use crate::league::League;
use log::{debug};

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        debug!("start simulating for {}", data.date);
        
        let ctx = GlobalContext::new(SimulationContext::new(data.date));

        let results: Vec<ContinentResult> = data.continents.iter_mut()
            .map(|continent| continent.simulate(ctx.with_continent(continent.id)))
            .collect();

        debug!("produced {} continent results", results.len());
        
        for result in results {
            result.process(data);
        }
        
        data.next_date();

        debug!("end simulating for {}", data.date);
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
    
    pub fn continent_mut(&mut self, id: u32) -> Option<&mut Continent>{
        self.continents.iter_mut().find(|c| c.id == id)
    }

    pub fn counties_mut(&mut self, id: u32) -> Option<&mut Country>{
        self.continents.iter_mut()
            .flat_map(|c|&mut c.countries)
            .find(|c| c.id == id)
    }

    pub fn leagues_mut(&mut self, id: u32) -> Option<&mut League>{
        self.continents.iter_mut()
            .flat_map(|c|&mut c.countries)
            .flat_map(|c|&mut c.leagues)
            .find(|c| c.id == id)
    }
}
