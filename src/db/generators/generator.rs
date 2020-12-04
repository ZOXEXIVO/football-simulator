use crate::db::{DatabaseEntity, PlayerGenerator};
use core::club::academy::ClubAcademy;
use core::context::{NaiveDateTime, NaiveTime};
use core::continent::Continent;
use core::league::{DayMonthPeriod, League, LeagueSettings, LeagueTable, Schedule};
use core::shared::Location;
use core::transfers::TransferPool;
use core::utils::IntegerUtils;
use core::{Club, ClubBoard, ClubFinances, ClubMood, Country, NaiveDate, PlayerCollection, PlayerPositionType, SimulatorData, StaffCollection, Team, TeamReputation, TeamType, TrainingSchedule, SimulatorDataIndexes};
use std::str::FromStr;
use crate::db::loaders::ContinentEntity;

pub struct Generator;

impl Generator {
    pub fn generate(data: &DatabaseEntity) -> SimulatorData {
        let current_date = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 11, 15),
            NaiveTime::from_hms(0, 0, 0),
        );

        let continents = data.continents
            .iter()
            .map(|continent| {
                let continent = Continent {
                    id: continent.id,
                    name: continent.name.clone(),
                    countries: Generator::generate_countries(continent, data),
                };

                continent
            })
            .collect();

        SimulatorData::new(current_date, continents)
    }

    fn generate_countries(continent: &ContinentEntity, data: &DatabaseEntity) -> Vec<Country> {
        return data
            .countries
            .iter()
            .filter(|cn| cn.continent_id == continent.id)
            .map(|country| {
                let clubs = Generator::generate_clubs(country.id, data);

                let country = Country {
                    id: country.id,
                    code: country.code.clone(),
                    name: country.name.clone(),
                    continent_id: continent.id,
                    leagues: Generator::generate_leagues(country.id, data),
                    clubs,
                    reputation: country.reputation,
                };

                country
            })
            .collect();
    }

    fn generate_leagues(country_id: u32, data: &DatabaseEntity) -> Vec<League> {
        return data
            .leagues
            .iter()
            .filter(|l| l.country_id == country_id)
            .map(|league| {
                let league = League {
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
                    table: Option::None,
                    reputation: 0,
                };

                league
            })
            .collect();
    }

    fn generate_clubs(country_id: u32, data: &DatabaseEntity) -> Vec<Club> {
        return data
            .clubs
            .iter()
            .filter(|c| c.country_id == country_id)
            .map(|club| {
                let club = Club {
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
                            Team::new(
                                t.id,
                                t.league_id,
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
                                PlayerCollection::new((0..50).map(|_| PlayerGenerator::generate()).collect()),
                                StaffCollection::new(Vec::new()),
                            )
                        })
                        .collect(),
                };

                club
            })
            .collect();
    }
}

pub struct PlayerPositionGenerator;

impl PlayerPositionGenerator {
    pub fn generate() -> PlayerPositionType {
        match IntegerUtils::random(0, 3) {
            0 => PlayerPositionType::Goalkeeper,
            1 => PlayerPositionType::Defender,
            2 => PlayerPositionType::Midfielder,
            3 => PlayerPositionType::Forward,
            _ => panic!("Unknown player position type"),
        }
    }
}
