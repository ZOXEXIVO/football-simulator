use crate::context::{GlobalContext, SimulationContext};
use crate::continent::{Continent, ContinentResult};
use crate::league::League;
use crate::shared::SimulatorDataIndexes;
use crate::transfers::TransferPool;
use crate::utils::Logging;
use crate::{Club, Country, Player, Team};
use chrono::{Duration, NaiveDateTime};

pub struct FootballSimulator;

impl FootballSimulator {
    pub fn simulate(data: &mut SimulatorData) {
        let message = &format!("simulate date {}", data.date);

        Logging::estimate(
            || {
                let ctx = GlobalContext::new(SimulationContext::new(data.date));

                let results: Vec<ContinentResult> = data
                    .continents
                    .iter_mut()
                    .map(|continent| continent.simulate(ctx.with_continent(continent.id)))
                    .collect();

                for result in results {
                    result.process(data);
                }

                data.next_date();
            },
            message,
        );
    }
}

pub struct SimulatorData {
    pub continents: Vec<Continent>,

    pub date: NaiveDateTime,

    pub transfer_pool: TransferPool<Player>,

    indexes: Option<SimulatorDataIndexes>,
}

impl SimulatorData {
    pub fn new(date: NaiveDateTime, continents: Vec<Continent>) -> Self {
        let mut data = SimulatorData {
            continents,
            date,
            transfer_pool: TransferPool::new(),
            indexes: None,
        };

        let mut indexes = SimulatorDataIndexes::new();

        indexes.refresh(&data);

        data.indexes = Some(indexes);

        data
    }

    pub fn next_date(&mut self) {
        self.date += Duration::days(1);
    }

    pub fn continent(&self, id: u32) -> Option<&Continent> {
        self.continents.iter().find(|c| c.id == id)
    }

    pub fn continent_mut(&mut self, id: u32) -> Option<&mut Continent> {
        self.continents.iter_mut().find(|c| c.id == id)
    }

    pub fn country(&self, id: u32) -> Option<&Country> {
        self.continents
            .iter()
            .flat_map(|c| &c.countries)
            .find(|c| c.id == id)
    }

    pub fn country_mut(&mut self, id: u32) -> Option<&mut Country> {
        self.continents
            .iter_mut()
            .flat_map(|c| &mut c.countries)
            .find(|c| c.id == id)
    }

    pub fn league(&self, id: u32) -> Option<&League> {
        let (league_continent_id, league_country_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_league_location(id)
            .unwrap();

        self.continent(league_continent_id)
            .unwrap()
            .countries
            .iter()
            .find(|country| country.id == league_country_id)
            .unwrap()
            .leagues
            .iter()
            .find(|c| c.id == id)
    }

    pub fn league_mut(&mut self, id: u32) -> Option<&mut League> {
        let (league_continent_id, league_country_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_league_location(id)
            .unwrap();

        self.continent_mut(league_continent_id)
            .unwrap()
            .countries
            .iter_mut()
            .find(|country| country.id == league_country_id)
            .unwrap()
            .leagues
            .iter_mut()
            .find(|c| c.id == id)
    }

    pub fn team_name(&self, id: u32) -> Option<&str> {
        self.indexes.as_ref().unwrap().get_team_name(id)
    }

    pub fn club(&self, id: u32) -> Option<&Club> {
        let (club_continent_id, club_country_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_club_location(id)
            .unwrap();

        self.continent(club_continent_id)
            .unwrap()
            .countries
            .iter()
            .find(|country| country.id == club_country_id)
            .unwrap()
            .clubs
            .iter()
            .find(|c| c.id == id)
    }

    pub fn club_mut(&mut self, id: u32) -> Option<&mut Club> {
        let (club_continent_id, club_country_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_club_location(id)
            .unwrap();

        self.continent_mut(club_continent_id)
            .unwrap()
            .countries
            .iter_mut()
            .find(|country| country.id == club_country_id)
            .unwrap()
            .clubs
            .iter_mut()
            .find(|c| c.id == id)
    }

    pub fn team(&self, id: u32) -> Option<&Team> {
        let (team_continent_id, team_country_id, team_club_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_team_location(id)
            .unwrap();

        self.continent(team_continent_id)
            .unwrap()
            .countries
            .iter()
            .find(|country| country.id == team_country_id)
            .unwrap()
            .clubs
            .iter()
            .find(|club| club.id == team_club_id)
            .unwrap()
            .teams
            .iter()
            .find(|c| c.id == id)
    }

    pub fn team_mut(&mut self, id: u32) -> Option<&mut Team> {
        let (team_continent_id, team_country_id, team_club_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_team_location(id)
            .unwrap();

        self.continent_mut(team_continent_id)
            .unwrap()
            .countries
            .iter_mut()
            .find(|country| country.id == team_country_id)
            .unwrap()
            .clubs
            .iter_mut()
            .find(|club| club.id == team_club_id)
            .unwrap()
            .teams
            .iter_mut()
            .find(|c| c.id == id)
    }

    pub fn player(&self, id: u32) -> Option<&Player> {
        let (player_continent_id, player_country_id, player_club_id, player_team_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_player_location(id)
            .unwrap();

        self.continent(player_continent_id)
            .unwrap()
            .countries            
            .iter()
            .find(|country| country.id == player_country_id)
            .unwrap()
            .clubs
            .iter()
            .find(|club| club.id == player_club_id)
            .unwrap()
            .teams
            .iter()
            .find(|team| team.id == player_team_id)
            .unwrap()
            .players
            .players
            .iter()
            .find(|c| c.id == id)
    }

    pub fn player_mut(&mut self, id: u32) -> Option<&mut Player> {
        let (player_continent_id, player_country_id, player_club_id, player_team_id) = self
            .indexes
            .as_ref()
            .unwrap()
            .get_player_location(id)
            .unwrap();

        self.continent_mut(player_continent_id)
            .unwrap()
            .countries
            .iter_mut()
            .find(|country| country.id == player_country_id)
            .unwrap()
            .clubs
            .iter_mut()
            .find(|club| club.id == player_club_id)
            .unwrap()
            .teams
            .iter_mut()
            .find(|team| team.id == player_team_id)
            .unwrap()
            .players
            .players
            .iter_mut()
            .find(|c| c.id == id)
    }
}
