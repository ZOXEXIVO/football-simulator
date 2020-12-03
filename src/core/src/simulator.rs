use crate::continent::{ContinentResult, Continent};
use chrono::{NaiveDateTime, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::transfers::TransferPool;
use crate::{Player, Country, Team};
use crate::context::{GlobalContext, SimulationContext};
use crate::league::League;
use crate::utils::Logging;

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        let message = &format!("simulate date {}", data.date);
        
        Logging::estimate(|| {
            let ctx = GlobalContext::new(SimulationContext::new(data.date));

            let results: Vec<ContinentResult> = data.continents.iter_mut()
                .map(|continent| continent.simulate(ctx.with_continent(continent.id)))
                .collect();

            for result in results {
                result.process(data);
            }

            data.next_date();
        }, message);        
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

    pub fn continents(&self, id: u32) -> Option<&Continent>{
        self.continents.iter().find(|c| c.id == id)
    }
    
    pub fn continents_mut(&mut self, id: u32) -> Option<&mut Continent>{
        self.continents.iter_mut().find(|c| c.id == id)
    }

    pub fn counties(&self, id: u32) -> Option<&Country>{
        self.continents.iter()
            .flat_map(|c|&c.countries)
            .find(|c| c.id == id)
    }
    
    pub fn counties_mut(&mut self, id: u32) -> Option<&mut Country>{
        self.continents.iter_mut()
            .flat_map(|c|&mut c.countries)
            .find(|c| c.id == id)
    }

    pub fn leagues(&self, id: u32) -> Option<&League>{
        self.continents.iter()
            .flat_map(|c|&c.countries)
            .flat_map(|c|&c.leagues)
            .find(|c| c.id == id)
    }
    
    pub fn leagues_mut(&mut self, id: u32) -> Option<&mut League>{
        self.continents.iter_mut()
            .flat_map(|c|&mut c.countries)
            .flat_map(|c|&mut c.leagues)
            .find(|c| c.id == id)
    }

    pub fn teams(&self, id: u32) -> Option<&Team>{
        self.continents.iter()
            .flat_map(|c|&c.countries)
            .flat_map(|c|&c.clubs)
            .flat_map(|c|&c.teams)
            .find(|c| c.id == id)
    }

    pub fn teams_mut(&mut self, id: u32) -> Option<&mut Team>{
        self.continents.iter_mut()
            .flat_map(|c|&mut c.countries)
            .flat_map(|c|&mut c.clubs)
            .flat_map(|c|&mut c.teams)
            .find(|c| c.id == id)
    }
}
