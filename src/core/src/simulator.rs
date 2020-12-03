use crate::continent::{ContinentResult, Continent};
use chrono::{NaiveDateTime, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::transfers::TransferPool;
use crate::{Player, Country, Team};
use crate::context::{GlobalContext, SimulationContext};
use crate::league::League;
use crate::utils::Logging;
use std::collections::HashMap;

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

    pub transfer_pool: TransferPool<Player>,
    
    indexes: SimulatorDataIndexes
}

impl SimulatorData {
    pub fn new(date: NaiveDateTime, continents: Vec<Continent>) -> Self{
        let id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect::<String>();
        
        SimulatorData{
            id,
            continents,
            date,
            transfer_pool: TransferPool::new(),
            indexes: SimulatorDataIndexes::new()
        }
    }

    pub fn next_date(&mut self) {
        self.date += Duration::days(1);
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
    
    pub fn refresh_indexes(&mut self) {
        for continent in &self.continents {
            for country in &continent.countries {
                //fill leagues
                for league in &country.leagues {
                    self.indexes.add_league_location(league.id, continent.id, country.id);
                }
                
                //fill teams
                for club in &country.clubs {
                    for team in &club.teams {
                        self.indexes.add_team_location(team.id, continent.id, country.id, club.id);
                    }
                }
            }
        }
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
        let (league_continent_id, league_country_id) =
            self.indexes.get_league_location(id).unwrap();
        
        self.continents
            .iter().find(|continent| continent.id == league_continent_id).unwrap().countries
            .iter().find(|country| country.id == league_country_id).unwrap().leagues         
            .iter().find(|c| c.id == id)
    }
    
    pub fn leagues_mut(&mut self, id: u32) -> Option<&mut League>{
        let (league_continent_id, league_country_id) =
            self.indexes.get_league_location(id).unwrap();

        self.continents
            .iter_mut().find(|continent| continent.id == league_continent_id).unwrap().countries
            .iter_mut().find(|country| country.id == league_country_id).unwrap().leagues
            .iter_mut().find(|c| c.id == id)
    }

    pub fn teams(&self, id: u32) -> Option<&Team>{
        let (team_continent_id, team_country_id, team_club_id) = 
            self.indexes.get_team_location(id).unwrap();
        
        self.continents
            .iter().find(|continent| continent.id == team_continent_id).unwrap().countries
            .iter().find(|country| country.id == team_country_id).unwrap().clubs
            .iter().find(|club| club.id == team_club_id).unwrap().teams
            .iter().find(|c| c.id == id)
    }

    pub fn teams_mut(&mut self, id: u32) -> Option<&mut Team>{
        let (team_continent_id, team_country_id, team_club_id) =
            self.indexes.get_team_location(id).unwrap();

        self.continents
            .iter_mut().find(|continent| continent.id == team_continent_id).unwrap().countries
            .iter_mut().find(|country| country.id == team_country_id).unwrap().clubs
            .iter_mut().find(|club| club.id == team_club_id).unwrap().teams
            .iter_mut().find(|c| c.id == id)
    }
}

pub struct SimulatorDataIndexes {
    league_indexes: HashMap<u32, (u32, u32)>,
    team_indexes: HashMap<u32, (u32, u32, u32)>
}

impl SimulatorDataIndexes {
    pub fn new() -> Self {
        SimulatorDataIndexes {
            league_indexes: HashMap::new(),
            team_indexes: HashMap::new(),
        }
    }
    
    //league indexes
    pub fn add_league_location(&mut self, league_id: u32, continent_id: u32, country_id: u32){
        self.league_indexes.insert(league_id, (continent_id, country_id));
    }

    pub fn get_league_location(&self, league_id: u32) -> Option<(u32, u32)> {
        match self.league_indexes.get(&league_id) {
            Some((league_continent_id, league_country_id)) => {
                Some((*league_continent_id, *league_country_id))
            }
            None => {
                None
            }
        }
    }
    
    //team indexes
    pub fn add_team_location(&mut self, team_id: u32, continent_id: u32, country_id: u32, club_id: u32){
        self.team_indexes.insert(team_id, (continent_id, country_id, club_id));
    }
    
    pub fn get_team_location(&self, team_id: u32) -> Option<(u32, u32, u32)> {
        match self.team_indexes.get(&team_id) {
            Some((team_continent_id, team_country_id, team_club_id)) => {
                Some((*team_continent_id, *team_country_id, *team_club_id))
            }
            None => {
                None   
            }            
        }
    }
}