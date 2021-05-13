use crate::context::{GlobalContext, SimulationContext};
use crate::continent::{Continent, ContinentResult};
use crate::league::League;
use crate::transfers::TransferPool;
use crate::utils::Logging;
use crate::{Club, Country, Player, Team};
use chrono::{Duration, NaiveDateTime, Timelike};
use std::collections::HashMap;

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

    indexes: SimulatorDataIndexes,
}

impl SimulatorData {
    pub fn new(date: NaiveDateTime, continents: Vec<Continent>) -> Self {
        let mut data = SimulatorData {
            continents,
            date,
            transfer_pool: TransferPool::new(),
            indexes: SimulatorDataIndexes::new(),
        };

        data.refresh_indexes();

        data
    }

    pub fn next_date(&mut self) {
        self.date += Duration::days(1);
    }

    fn refresh_indexes(&mut self) {
        for continent in &self.continents {
            for country in &continent.countries {
                //fill leagues
                for league in &country.leagues {
                    self.indexes
                        .add_league_location(league.id, continent.id, country.id);
                }

                //fill teams
                for club in &country.clubs {
                    self.indexes
                        .add_club_location(club.id, continent.id, country.id);

                    for team in &club.teams {
                        self.indexes.add_team_name(team.id, team.name.clone());
                        self.indexes
                            .add_team_location(team.id, continent.id, country.id, club.id);

                        for player in &team.players.players {
                            self.indexes.add_player_location(
                                player.id,
                                continent.id,
                                country.id,
                                club.id,
                                team.id,
                            );
                        }
                    }
                }
            }
        }
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

    pub fn countie_mut(&mut self, id: u32) -> Option<&mut Country> {
        self.continents
            .iter_mut()
            .flat_map(|c| &mut c.countries)
            .find(|c| c.id == id)
    }

    pub fn league(&self, id: u32) -> Option<&League> {
        let (league_continent_id, league_country_id) =
            self.indexes.get_league_location(id).unwrap();

        self.continents
            .iter()
            .find(|continent| continent.id == league_continent_id)
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
        let (league_continent_id, league_country_id) =
            self.indexes.get_league_location(id).unwrap();

        self.continents
            .iter_mut()
            .find(|continent| continent.id == league_continent_id)
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
        self.indexes.get_team_name(id)
    }

    pub fn club(&self, id: u32) -> Option<&Club> {
        let (club_continent_id, club_country_id) = self.indexes.get_club_location(id).unwrap();

        self.continents
            .iter()
            .find(|continent| continent.id == club_continent_id)
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
        let (club_continent_id, club_country_id) = self.indexes.get_club_location(id).unwrap();

        self.continents
            .iter_mut()
            .find(|continent| continent.id == club_continent_id)
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
        let (team_continent_id, team_country_id, team_club_id) =
            self.indexes.get_team_location(id).unwrap();

        self.continents
            .iter()
            .find(|continent| continent.id == team_continent_id)
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
        let (team_continent_id, team_country_id, team_club_id) =
            self.indexes.get_team_location(id).unwrap();

        self.continents
            .iter_mut()
            .find(|continent| continent.id == team_continent_id)
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
        let (player_continent_id, player_country_id, player_club_id, player_team_id) =
            self.indexes.get_player_location(id).unwrap();

        self.continents
            .iter()
            .find(|continent| continent.id == player_continent_id)
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
        let (player_continent_id, player_country_id, player_club_id, player_team_id) =
            self.indexes.get_player_location(id).unwrap();

        self.continents
            .iter_mut()
            .find(|continent| continent.id == player_continent_id)
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

pub struct SimulatorDataIndexes {
    league_indexes: HashMap<u32, (u32, u32)>,
    club_indexes: HashMap<u32, (u32, u32)>,
    team_indexes: HashMap<u32, (u32, u32, u32)>,
    player_indexes: HashMap<u32, (u32, u32, u32, u32)>,
    team_name_index: HashMap<u32, String>,
}

impl SimulatorDataIndexes {
    pub fn new() -> Self {
        SimulatorDataIndexes {
            league_indexes: HashMap::new(),
            club_indexes: HashMap::new(),
            team_indexes: HashMap::new(),
            player_indexes: HashMap::new(),
            team_name_index: HashMap::new(),
        }
    }

    //league indexes
    pub fn add_league_location(&mut self, league_id: u32, continent_id: u32, country_id: u32) {
        self.league_indexes
            .insert(league_id, (continent_id, country_id));
    }

    pub fn get_league_location(&self, league_id: u32) -> Option<(u32, u32)> {
        match self.league_indexes.get(&league_id) {
            Some((league_continent_id, league_country_id)) => {
                Some((*league_continent_id, *league_country_id))
            }
            None => None,
        }
    }

    //club indexes

    pub fn add_club_location(&mut self, club_id: u32, continent_id: u32, country_id: u32) {
        self.club_indexes
            .insert(club_id, (continent_id, country_id));
    }

    pub fn get_club_location(&self, club_id: u32) -> Option<(u32, u32)> {
        match self.club_indexes.get(&club_id) {
            Some((club_continent_id, club_country_id)) => {
                Some((*club_continent_id, *club_country_id))
            }
            None => None,
        }
    }

    //team indexes
    pub fn add_team_name(&mut self, team_id: u32, name: String) {
        self.team_name_index.insert(team_id, name);
    }
    pub fn get_team_name(&self, team_id: u32) -> Option<&str> {
        match self.team_name_index.get(&team_id) {
            Some(team_name) => Some(team_name),
            None => None,
        }
    }

    pub fn add_team_location(
        &mut self,
        team_id: u32,
        continent_id: u32,
        country_id: u32,
        club_id: u32,
    ) {
        self.team_indexes
            .insert(team_id, (continent_id, country_id, club_id));
    }

    pub fn get_team_location(&self, team_id: u32) -> Option<(u32, u32, u32)> {
        match self.team_indexes.get(&team_id) {
            Some((team_continent_id, team_country_id, team_club_id)) => {
                Some((*team_continent_id, *team_country_id, *team_club_id))
            }
            None => None,
        }
    }

    //player indexes

    pub fn add_player_location(
        &mut self,
        player_id: u32,
        continent_id: u32,
        country_id: u32,
        club_id: u32,
        team_id: u32,
    ) {
        self.player_indexes
            .insert(player_id, (continent_id, country_id, club_id, team_id));
    }

    pub fn get_player_location(&self, player_id: u32) -> Option<(u32, u32, u32, u32)> {
        match self.player_indexes.get(&player_id) {
            Some((player_continent_id, player_country_id, player_club_id, player_team_id)) => {
                Some((
                    *player_continent_id,
                    *player_country_id,
                    *player_club_id,
                    *player_team_id,
                ))
            }
            None => None,
        }
    }
}
