use crate::SimulatorData;
use std::collections::HashMap;

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

    pub fn refresh(&mut self, data: &SimulatorData) {
        for continent in &data.continents {
            for country in &continent.countries {
                //fill leagues
                for league in &country.leagues {
                    self.add_league_location(league.id, continent.id, country.id);
                }

                //fill teams
                for club in &country.clubs {
                    self.add_club_location(club.id, continent.id, country.id);

                    for team in &club.teams.teams {
                        self.add_team_name(team.id, team.name.clone());
                        self.add_team_location(team.id, continent.id, country.id, club.id);

                        for player in &team.players.players {
                            self.add_player_location(
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
