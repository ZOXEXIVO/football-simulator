use crate::db::loaders::ContinentEntity;
use crate::db::{DatabaseEntity, PlayerGenerator, PositionType};
use core::club::academy::ClubAcademy;
use core::context::{NaiveTime, Timelike};
use core::continent::Continent;
use core::league::{DayMonthPeriod, League, LeagueSettings, LeagueTable, Schedule};
use core::shared::Location;
use core::utils::IntegerUtils;
use core::{Club, ClubBoard, ClubFinances, ClubMood, Country, Player, PlayerCollection, PlayerPosition, PlayerPositionType, SimulatorData, StaffCollection, Team, TeamReputation, TeamType, TrainingSchedule, Utc, CountryGeneratorData};
use std::str::FromStr;

pub struct Generator;

impl Generator {
    pub fn generate(data: &DatabaseEntity) -> SimulatorData {
        let current_date = Utc::now()
            .naive_utc()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        let continents = data
            .continents
            .iter()
            .map(|continent| Continent {
                id: continent.id,
                name: continent.name.clone(),
                countries: Generator::generate_countries(continent, data),
            })
            .collect();

        SimulatorData::new(current_date, continents)
    }

    fn generate_countries(
        continent: &ContinentEntity,
        data: &DatabaseEntity
    ) -> Vec<Country> {
        return data
            .countries
            .iter()
            .filter(|cn| cn.continent_id == continent.id)
            .map(|country| {
          
                let generator_data = match data.names_by_country.iter().find(|c| c.country_id == country.id) {
                    Some(names) => {
                        CountryGeneratorData::new(names.first_names.clone(), names.last_names.clone())
                    },
                    None => {
                        CountryGeneratorData::empty()
                    }
                };
                
                let mut player_generator = PlayerGenerator::with_people_names(&generator_data.people_names);

                let clubs = Generator::generate_clubs(country.id, data, &mut player_generator);

                let country = Country {
                    id: country.id,
                    code: country.code.clone(),
                    name: country.name.clone(),
                    continent_id: continent.id,
                    leagues: Generator::generate_leagues(country.id, data),
                    clubs,
                    reputation: country.reputation,
                    generator_data
                };

                country
            })
            .collect();
    }

    fn generate_leagues(
        country_id: u32,
        data: &DatabaseEntity
    ) -> Vec<League> {
        return data
            .leagues
            .iter()
            .filter(|l| l.country_id == country_id)
            .map(|league| {
                let league_clubs: Vec<u32> = data
                    .clubs
                    .iter()
                    .flat_map(|c| &c.teams)
                    .filter(|team| team.league_id == league.id)
                    .map(|t| t.id)
                    .collect();

                League {
                    id: league.id,
                    name: league.name.clone(),
                    country_id: league.country_id,
                    schedule: Schedule::new(),
                    settings: LeagueSettings {
                        season_starting_half: DayMonthPeriod {
                            from_day: league.settings.season_starting_half.from_day,
                            from_month: league.settings.season_starting_half.from_month,
                            to_day: league.settings.season_starting_half.to_day,
                            to_month: league.settings.season_starting_half.to_month,
                        },
                        season_ending_half: DayMonthPeriod {
                            from_day: league.settings.season_ending_half.from_day,
                            from_month: league.settings.season_ending_half.from_month,
                            to_day: league.settings.season_ending_half.to_day,
                            to_month: league.settings.season_ending_half.to_month,
                        },
                    },
                    table: Some(LeagueTable::with_clubs(&league_clubs)),
                    reputation: 0,
                }
            })
            .collect();
    }

    fn generate_clubs(
        country_id: u32,
        data: &DatabaseEntity,
        player_generator: &mut PlayerGenerator,
    ) -> Vec<Club> {
        return data
            .clubs
            .iter()
            .filter(|c| c.country_id == country_id)
            .map(|club| Club {
                id: club.id,
                name: club.name.clone(),
                location: Location {
                    city_id: club.location.city_id,
                },
                mood: ClubMood::default(),
                board: ClubBoard::new(),
                finance: ClubFinances::new(club.finance.balance, Vec::new()),
                academy: ClubAcademy::new(100),
                teams: club
                    .teams
                    .iter()
                    .map(|t| {
                        let mut players = Vec::with_capacity(100);
                        
                        let mut goalkeepers: Vec<Player> = (0..IntegerUtils::random(1, 5)).map(|_| {
                            player_generator.generate(country_id, PositionType::Goalkeeper)
                        }).collect();

                        let mut defenders: Vec<Player> = (0..IntegerUtils::random(7, 10)).map(|_| {
                            player_generator.generate(country_id, PositionType::Defender)
                        }).collect();

                        let mut midfielders: Vec<Player> = (0..IntegerUtils::random(9, 12)).map(|_| {
                            player_generator.generate(country_id, PositionType::Midfielder)
                        }).collect();

                        let mut strikers: Vec<Player> = (0..IntegerUtils::random(2, 4)).map(|_| {
                            player_generator.generate(country_id, PositionType::Striker)
                        }).collect();
                   
                        players.append(&mut goalkeepers);
                        players.append(&mut defenders);
                        players.append(&mut midfielders);
                        players.append(&mut strikers);
                                                
                        Team::new(
                            t.id,
                            t.league_id,
                            club.id,
                            t.name.clone(),
                            TeamType::from_str(&t.team_type).unwrap(),
                            TrainingSchedule::new(
                                NaiveTime::from_hms(10, 0, 0),
                                NaiveTime::from_hms(17, 0, 0),
                            ),
                            TeamReputation::new(
                                t.reputation.home,
                                t.reputation.national,
                                t.reputation.world,
                            ),
                            PlayerCollection::new(players),
                            StaffCollection::new(Vec::new()),
                        )
                    })
                    .collect(),
            })
            .collect();
    }
}
