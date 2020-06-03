use crate::club::{Club, ClubBoard, ClubMood, TrainingSchedule, ClubFinances, ClubSponsorshipContract};
use crate::country::Country;
use crate::league::{League, LeagueSettings, LeagueTable, ScheduleManager};
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::utils::{IntegerUtils, StringUtils};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::continent::Continent;
use crate::people::{
    Mental, Physical, Player, PlayerAttributes, PlayerClubContract, PlayerCollection,
    PlayerPosition, PlayerPositionType, PlayerSkills, Staff, StaffClubContract, StaffCollection,
    StaffPosition, StaffStatus, Technical,
};
use crate::transfers::TransferPool;

impl SimulatorData {
    pub fn generate() -> SimulatorData {
        let date = NaiveDate::from_ymd(2020, 11, 15);
        let time = NaiveTime::from_hms(0, 0, 0);

        let clubs = vec![
            Club::new(1, String::from("Zenit St. Petersburg"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(2, String::from("Krasnodar"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(3, String::from("CSKA Moscow"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(4, String::from("Lokomotiv Moscow "),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(5, String::from("Spartak Moscow"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(6, String::from("Dinamo Moscow"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),


            Club::new(7, String::from("Rubin Kazan"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(8, String::from("Rostov"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(9, String::from("Akhmat"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(10, String::from("Arsenal Tula"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(11, String::from("Sochi"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(12, String::from("Ufa"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(13, String::from("Ural"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(14, String::from("Orenburg"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(15, String::from("Krylya Sovetov"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
                      StaffCollection::new((0..10).map(|_| Staff::generate()).collect())),

            Club::new(16, String::from("Tambov"),
                      ClubFinances::new(25_000_000, Vec::new()),
                      TrainingSchedule::new(
                          NaiveTime::from_hms(10, 0, 0),
                          NaiveTime::from_hms(17, 0, 0),
                      ),
                      PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
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
                                    season_starting: (1, 1),
                                    season_ending: (1, 12),
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
            schedule: ScheduleManager::new(),
            settings: LeagueSettings {
                season_starting: (1, 12),
                season_ending: (1, 12),
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
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2023, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 2"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2025, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 3"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2020, 1, 1))
        ];

        Club::new(
            IntegerUtils::random(1, 10_000_000) as u32,
            StringUtils::random_string(15),
            ClubFinances::new(IntegerUtils::random(-10000, 10000000) as i32, sponsorship_contracts),
            training_schedule,
            PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
            StaffCollection::new((0..10).map(|_| Staff::generate()).collect()),
        )
    }
}

impl Player {
    fn generate() -> Player {
        let year = IntegerUtils::random(1980, 2010) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        return Player::new(
            IntegerUtils::random(1, 1_000_000) as u32,
            FullName {
                first_name: StringUtils::random_string(5),
                last_name: StringUtils::random_string(10),
                middle_name: StringUtils::random_string(15),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            generate_skills(),
            generate_attributes(),
            Some(PlayerClubContract::new(
                IntegerUtils::random(1980, 2010) as f64, NaiveDate::from_ymd(2020, 3, 14))),
            generate_positions(),
        );

        fn generate_skills() -> PlayerSkills {
            PlayerSkills {
                technical: Technical {
                    corners: IntegerUtils::random(1, 20) as u8,
                    crossing: IntegerUtils::random(1, 20) as u8,
                    dribbling: IntegerUtils::random(1, 20) as u8,
                    finishing: IntegerUtils::random(1, 20) as u8,
                    first_touch: IntegerUtils::random(1, 20) as u8,
                    free_kick_taking: IntegerUtils::random(1, 20) as u8,
                    heading: IntegerUtils::random(1, 20) as u8,
                    long_shots: IntegerUtils::random(1, 20) as u8,
                    long_throws: IntegerUtils::random(1, 20) as u8,
                    marking: IntegerUtils::random(1, 20) as u8,
                    passing: IntegerUtils::random(1, 20) as u8,
                    penalty_taking: IntegerUtils::random(1, 20) as u8,
                    tackling: IntegerUtils::random(1, 20) as u8,
                    technique: IntegerUtils::random(1, 20) as u8,
                },
                mental: Mental {
                    aggression: IntegerUtils::random(1, 20) as u8,
                    anticipation: IntegerUtils::random(1, 20) as u8,
                    bravery: IntegerUtils::random(1, 20) as u8,
                    composure: IntegerUtils::random(1, 20) as u8,
                    concentration: IntegerUtils::random(1, 20) as u8,
                    decisions: IntegerUtils::random(1, 20) as u8,
                    determination: IntegerUtils::random(1, 20) as u8,
                    flair: IntegerUtils::random(1, 20) as u8,
                    leadership: IntegerUtils::random(1, 20) as u8,
                    off_the_ball: IntegerUtils::random(1, 20) as u8,
                    positioning: IntegerUtils::random(1, 20) as u8,
                    teamwork: IntegerUtils::random(1, 20) as u8,
                    vision: IntegerUtils::random(1, 20) as u8,
                    work_rate: IntegerUtils::random(1, 20) as u8,
                },
                physical: Physical {
                    acceleration: IntegerUtils::random(1, 20) as u8,
                    agility: IntegerUtils::random(1, 20) as u8,
                    balance: IntegerUtils::random(1, 20) as u8,
                    jumping_reach: IntegerUtils::random(1, 20) as u8,
                    natural_fitness: IntegerUtils::random(1, 20) as u8,
                    pace: IntegerUtils::random(1, 20) as u8,
                    stamina: IntegerUtils::random(1, 20) as u8,
                    strength: IntegerUtils::random(1, 20) as u8,
                    match_readiness: IntegerUtils::random(1, 20) as u8,
                },
            }
        }

        fn generate_positions() -> Vec<PlayerPosition> {
            let positions_to_generate = IntegerUtils::random(1, 4) as u32;

            let mut positions = Vec::with_capacity(positions_to_generate as usize);

            for pos in 0..positions_to_generate {
                positions.push(PlayerPosition {
                    position: PlayerPositionGenerator::generate(),
                    level: IntegerUtils::random(0, 20) as u8,
                })
            }

            positions
        }

        fn generate_attributes() -> PlayerAttributes {
            PlayerAttributes::new(
                IntegerUtils::random(0, 20) as u8,
                IntegerUtils::random(-20, 20) as i8,
            )
        }
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
