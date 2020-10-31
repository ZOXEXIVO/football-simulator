use crate::club::{Club, ClubBoard, ClubMood, TrainingSchedule, ClubFinances, ClubSponsorshipContract,
                  PlayerCollection, ClubReputation};
use crate::country::Country;
use crate::league::{League, LeagueSettings, LeagueTable, ScheduleManager, DayMonthPeriod};
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::utils::{IntegerUtils, StringUtils};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::continent::Continent;

use crate::club::{
    Mental, Physical, PlayerAttributes, PlayerClubContract,
    PlayerPosition, PlayerPositionType, PlayerSkills, Staff, StaffClubContract, StaffCollection,
    StaffPosition, StaffStatus, Technical,
};

use crate::transfers::TransferPool;
use crate::shared::Location;
use crate::generators::PlayerGenerator;

impl SimulatorData {
    pub fn generate() -> SimulatorData {
        let date = NaiveDate::from_ymd(2020, 11, 15);
        let time = NaiveTime::from_hms(0, 0, 0);

        let clubs = vec![
            Club::new(1, String::from("Zenit St. Petersburg"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(2, String::from("Krasnodar"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(3, String::from("CSKA Moscow"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(4, String::from("Lokomotiv Moscow "),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(5, String::from("Spartak Moscow"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(6, String::from("Dinamo Moscow"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(7, String::from("Rubin Kazan"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(8, String::from("Rostov"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(9, String::from("Akhmat"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(10, String::from("Arsenal Tula"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(11, String::from("Sochi"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(12, String::from("Ufa"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(13, String::from("Ural"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(14, String::from("Orenburg"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(15, String::from("Krylya Sovetov"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),
            Club::new(16, String::from("Tambov"),
                      Location::new(1),
                      ClubFinances::new(25_000_000, Vec::new()),
                      ClubReputation::new(3000, 2000, 1000),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect()))
        ];

        SimulatorData {
            id: SimulatorData::generate_id(),
            continents: vec![
                Continent {
                    id: 0,
                    name: "Africa".to_string(),
                    countries: vec![],
                },
                Continent {
                    id: 1,
                    name: "Europe".to_string(),
                    countries: vec![
                        Country {
                            id: 87,
                            code: String::from("ru"),
                            name: String::from("Russia"),
                            leagues: vec![
                                League::new(1, String::from("Premier league"), 5000, LeagueSettings {
                                    season_starting_half: DayMonthPeriod::new(1, 7, 5, 12),
                                    season_ending_half: DayMonthPeriod::new(1, 3, 31, 5),
                                }, clubs)
                            ],
                            reputation: 5000,
                        }
                    ],
                },
                Continent {
                    id: 2,
                    name: "North America".to_string(),
                    countries: vec![],
                },
                Continent {
                    id: 3,
                    name: "South America".to_string(),
                    countries: vec![],
                },
                Continent {
                    id: 4,
                    name: "Australia".to_string(),
                    countries: vec![],
                },
            ],
            date: NaiveDateTime::new(date, time),
            transfer_pool: TransferPool::new(),
        }
    }
}

impl League {
    fn generate() -> League {
        let clubs_count = 10;

        let clubs: Vec<Club> = (0..clubs_count).map(|_| Club::generate()).collect();
        let club_headers = clubs.iter().map(|c| c.id).collect();

        League {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(30),
            clubs,
            schedule_manager: ScheduleManager::new(),
            settings: LeagueSettings {
                season_starting_half: DayMonthPeriod::new(1, 7, 5, 12),
                season_ending_half: DayMonthPeriod::new(1, 3, 31, 5),
            },
            league_table: LeagueTable::new(club_headers),
            reputation: 5000,
        }
    }
}

impl Club {
    fn generate() -> Club {
        let training_schedule = TrainingSchedule::new(
            NaiveTime::from_hms(10, 0, 0),
            NaiveTime::from_hms(17, 0, 0),
        );

        let sponsorship_contracts = vec![
            ClubSponsorshipContract::new(String::from("Sponsor 1"),
                                         IntegerUtils::random(1, 10_000_000),
                                         NaiveDate::from_ymd(2023, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 2"),
                                         IntegerUtils::random(1, 10_000_000),
                                         NaiveDate::from_ymd(2025, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 3"),
                                         IntegerUtils::random(1, 10_000_000),
                                         NaiveDate::from_ymd(2020, 1, 1))
        ];

        Club::new(
            IntegerUtils::random(1, 10_000_000) as u32,
            StringUtils::random_string(15),
            Location::new(2),
            ClubFinances::new(IntegerUtils::random(-10000, 10000000) as i32, sponsorship_contracts),
            ClubReputation::new(3000, 2000, 1000),
            training_schedule,
            PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
            StaffCollection::new((0..10).map(|_| Staff::generate()).collect()),
        )
    }
}

impl Staff {
    fn generate() -> Staff {
        let year = IntegerUtils::random(1980, 2010) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Staff::new(
            IntegerUtils::random(1, 10_000_000) as u32,
            FullName {
                first_name: StringUtils::random_string(5),
                last_name: StringUtils::random_string(10),
                middle_name: StringUtils::random_string(15),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            Some(StaffClubContract::new(
                NaiveDate::from_ymd(2020, 3, 14),
                StaffPosition::MainCoach,
                StaffStatus::Active,
            )),
        )
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
            _ => PlayerPositionType::Goalkeeper,
        }
    }
}
