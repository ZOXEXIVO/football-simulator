use crate::club::{Club, ClubBoard};
use crate::country::Country;
use crate::league::{League, LeagueSettings};
use crate::player::*;
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::staff::contract::{StaffClubContract, StaffCollection, StaffPosition};
use crate::staff::staff::Staff;
use crate::utils::{IntegerUtils, StringUtils};
use std::collections::HashMap;

use chrono::NaiveDate;

use rayon::prelude::*;
use crate::continent::Continent;

pub trait Generator {
    fn generate() -> Self;
}

impl Generator for SimulatorData {
    fn generate() -> SimulatorData {
        SimulatorData {
            continents: (0..7).into_par_iter().map(|_| Generator::generate()).collect(),
            free_players: (0..1000).into_par_iter().map(|_| Generator::generate()).collect(),
            free_staff: (0..1000).map(|_| Generator::generate()).collect(),
        }
    }
}


impl Generator for Continent {
    fn generate() -> Continent {
        Continent {
            name: StringUtils::random_string(10),
            countries: (0..40).map(|_| Generator::generate()).collect(),
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
        let clubs = (0..30)
            .map(|_| Generator::generate())        
            .into_iter()
            .collect();

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
            board: ClubBoard::new(),
            players: PlayerCollection::new((0..10).map(|_| Generator::generate()).collect()),
            staffs: StaffCollection::new((0..10).map(|_| Generator::generate()).collect()),
            tactics: None,
        }
    }
}

impl Generator for PlayerClubContract {
    fn generate() -> PlayerClubContract {
        PlayerClubContract::new(
            Generator::generate(),
            NaiveDate::from_ymd(2020, 3, 14),
        )
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
        );

        fn generate_skills() -> PlayerSkills {
            PlayerSkills {
                technical: Technical {
                    corners: 10,
                    crossing: 10,
                    dribbling: 10,
                    finishing: 10,
                    first_touch: 10,
                    free_kick_taking: 10,
                    heading: 10,
                    long_shots: 10,
                    long_throws: 10,
                    marking: 10,
                    passing: 10,
                    penalty_taking: 10,
                    tackling: 10,
                    technique: 10,
                },
                metal: Metal {
                    aggression: 10,
                    anticipation: 10,
                    brawery: 10,
                    composure: 10,
                    contentration: 10,
                    decisions: 10,
                    determination: 10,
                    flair: 10,
                    leadership: 10,
                    off_the_ball: 10,
                    positioning: 10,
                    teamwork: 10,
                    vision: 10,
                    work_rate: 10,
                },
                physical: Physical {
                    acceleration: 10,
                    agility: 10,
                    balance: 10,
                    jumping_reach: 10,
                    natural_fitness: 10,
                    pace: 10,
                    stamina: 10,
                    strength: 10,
                },
            }
        }
    }
}

impl Generator for StaffClubContract {
    fn generate() -> StaffClubContract {
        StaffClubContract::new(
              Generator::generate(),
              NaiveDate::from_ymd(2020, 3, 14),
              StaffPosition::MainCoach,
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
