use crate::club::{Club, ClubBoard, ClubMood};
use crate::country::Country;
use crate::league::{League, LeagueSettings};
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::utils::{IntegerUtils, StringUtils};

use chrono::NaiveDate;

use crate::continent::Continent;
use crate::people::{
    Mental, Physical, Player, PlayerAttributes, PlayerClubContract, PlayerCollection,
    PlayerPosition, PlayerPositionType, PlayerSkills, Staff, StaffClubContract, StaffCollection,
    StaffPosition, StaffStatus, Technical,
};

use rayon::prelude::*;
use std::sync::Mutex;

pub trait Generator {
    fn generate() -> Self;
}

impl Generator for SimulatorData {
    fn generate() -> SimulatorData {
        let free_players = (0..1000)
            .into_par_iter()
            .map(|_| Generator::generate())
            .collect();

        let free_staffs = (0..1000)
            .into_par_iter()
            .map(|_| Generator::generate())
            .collect();

        SimulatorData {
            continents: (0..7).into_iter().map(|_| Generator::generate()).collect(),
            free_players_pool: Mutex::new(free_players),
            free_staffs_pool: Mutex::new(free_staffs),
        }
    }
}

impl Generator for Continent {
    fn generate() -> Continent {
        Continent {
            name: StringUtils::random_string(10),
            countries: (0..190).map(|_| Generator::generate()).collect(),
            tournaments: Vec::new(),
        }
    }
}

impl Generator for Country {
    fn generate() -> Country {
        Country {
            name: StringUtils::random_string(10),
            leagues: (0..10).map(|_| Generator::generate()).collect(),
            reputation: 5000,
        }
    }
}

impl Generator for League {
    fn generate() -> League {
        let clubs = (0..30).map(|_| Generator::generate()).collect();

        League {
            name: StringUtils::random_string(10),
            clubs,
            schedule: None,
            settings: LeagueSettings {
                season_starting: (1, 1),
                season_ending: (1, 12),
            },
            reputation: 5000,
        }
    }
}

impl Generator for Club {
    fn generate() -> Club {
        Club {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(5),
            mood: ClubMood::default(),
            board: ClubBoard::new(),
            players: PlayerCollection::new((0..10).map(|_| Generator::generate()).collect()),
            staffs: StaffCollection::new((0..10).map(|_| Generator::generate()).collect()),
            tactics: None,
            transfer_list: Vec::new(),
        }
    }
}

impl Generator for PlayerClubContract {
    fn generate() -> PlayerClubContract {
        PlayerClubContract::new(Generator::generate(), NaiveDate::from_ymd(2020, 3, 14))
    }
}

impl Generator for Player {
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

impl Generator for StaffClubContract {
    fn generate() -> StaffClubContract {
        StaffClubContract::new(
            Generator::generate(),
            NaiveDate::from_ymd(2020, 3, 14),
            StaffPosition::MainCoach,
            StaffStatus::Active,
        )
    }
}

impl Generator for Staff {
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
