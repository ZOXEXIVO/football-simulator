use crate::core::SimulatorData;
use crate::models::club::{Club, ClubBoard};
use crate::models::country::Country;
use crate::models::league::{League, LeagueSettings};
use crate::models::player::*;
use crate::models::shared::fullname::FullName;
use crate::models::staff::contract::StaffClubContract;
use crate::models::staff::staff::Staff;
use crate::utils::{IntegerUtils, StringUtils};

extern crate crossbeam;

use chrono::NaiveDate;

pub trait Generator {
      fn generate(index: i32) -> Self;
}

impl Generator for SimulatorData {
      fn generate(index: i32) -> SimulatorData {
            let generated_countries = (0..900).map(|i| Generator::generate(i)).collect();

            SimulatorData {
                  countries: generated_countries,
                  free_players: (0..1000).map(|i| Generator::generate(i)).collect(),
                  free_staff: (0..1000).map(|i| Generator::generate(i)).collect(),
            }
      }
}

impl Generator for Country {
      fn generate(index: i32) -> Country {
            Country {
                  name: index.to_string(),
                  leagues: (0..4).map(|i| Generator::generate(i)).collect(),
            }
      }
}

impl Generator for League {
      fn generate(index: i32) -> League {
            League {
                  name: StringUtils::random_string(10),
                  clubs: (0..60).map(|i| Generator::generate(i)).collect(),
                  schedule: None,
                  settings: LeagueSettings {
                        season_starting: (1, 1),
                        season_ending: (1, 12),
                  },
            }
      }
}

impl Generator for Club {
      fn generate(index: i32) -> Club {
            Club {
                  id: IntegerUtils::random(1, 10000000) as u32,
                  name: StringUtils::random_string(5),
                  board: ClubBoard::new(),
                  players: (0..60).map(|i| Generator::generate(i)).collect(),
                  staffs: (0..20).map(|i| Generator::generate(i)).collect(),
            }
      }
}

impl Generator for PlayerClubContract {
      fn generate(index: i32) -> PlayerClubContract {
            return PlayerClubContract::new(
                  Generator::generate(index),
                  NaiveDate::from_ymd(2020, 3, 14),
            );
      }
}

impl Generator for Player {
      fn generate(index: i32) -> Player {
            let year = IntegerUtils::random(1980, 2010) as u32;
            let month = IntegerUtils::random(1, 12) as u32;
            let day = IntegerUtils::random(1, 29) as u32;

            return Player::new(
                  IntegerUtils::random(1, 1000000) as u32,
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
      fn generate(index: i32) -> StaffClubContract {
            return StaffClubContract::new(
                  Generator::generate(index),
                  NaiveDate::from_ymd(2020, 3, 14),
            );
      }
}

impl Generator for Staff {
      fn generate(index: i32) -> Staff {
            let year = IntegerUtils::random(1980, 2010) as u32;
            let month = IntegerUtils::random(1, 12) as u32;
            let day = IntegerUtils::random(1, 29) as u32;

            Staff::new(
                  IntegerUtils::random(1, 10000000) as u32,
                  FullName {
                        first_name: StringUtils::random_string(5),
                        last_name: StringUtils::random_string(10),
                        middle_name: StringUtils::random_string(15),
                  },
                  NaiveDate::from_ymd(year as i32, month, day),
            )
      }
}
