use crate::club::{Club, ClubBoard};
use crate::country::Country;
use crate::league::{League, LeagueSettings};
use crate::player::*;
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::staff::contract::{StaffClubContract, StaffCollection, StaffPosition};
use crate::staff::staff::{Staff};
use crate::utils::{IntegerUtils, StringUtils};
use std::collections::HashMap;

use chrono::NaiveDate;

use rayon::prelude::*;

pub trait Generator {
      fn generate() -> Self;
}

impl Generator for SimulatorData {
      fn generate() -> SimulatorData {
            SimulatorData {
                  //countries: (0..1000).into_par_iter().map(|_| Generator::generate()).collect(),
                  countries: vec![Country {
                        name: "Russia".to_string(),
                        leagues: vec![League {
                              name: "Russian Football Premier League".to_string(),
                              clubs: [(
                                    0,
                                    Club {
                                          id: 0,
                                          name: "Zenith".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..30).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    1,
                                    Club {
                                          id: 1,
                                          name: "Spartak Moscow".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    2,
                                    Club {
                                          id: 2,
                                          name: "Lokomotiv Moscow".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    3,
                                    Club {
                                          id: 3,
                                          name: "Krasnodar".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    4,
                                    Club {
                                          id: 4,
                                          name: "Rostov".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    5,
                                    Club {
                                          id: 5,
                                          name: "CSKA Moscow".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              )]
                              .iter()
                              .cloned()
                              .collect(),
                              schedule: None,
                              settings: LeagueSettings {
                                    season_starting: (5, 1),
                                    season_ending: (1, 12),
                              },
                        }, League {
                              name: "Football National League".to_string(),
                              clubs: [(
                                    0,
                                    Club {
                                          id: 0,
                                          name: "Tom".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..30).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    1,
                                    Club {
                                          id: 1,
                                          name: "Spartak-2".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    2,
                                    Club {
                                          id: 2,
                                          name: "Chertanovo".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    3,
                                    Club {
                                          id: 3,
                                          name: "Khimki".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    4,
                                    Club {
                                          id: 4,
                                          name: "Spartak Moscow - 2".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              ),
                              (
                                    5,
                                    Club {
                                          id: 5,
                                          name: "Baltika".to_string(),
                                          board: ClubBoard::new(),
                                          players: (0..60).map(|_| Generator::generate()).collect(),
                                          staffs: StaffCollection::new((0..20).map(|_| Generator::generate()).collect()),
                                          tactics: None
                                    },
                              )]
                              .iter()
                              .cloned()
                              .collect(),
                              schedule: None,
                              settings: LeagueSettings {
                                    season_starting: (5, 1),
                                    season_ending: (1, 12),
                              },
                        }],
                  }],
                  free_players: (0..1000).into_par_iter().map(|_| Generator::generate()).collect(),
                  free_staff: (0..1000).map(|_| Generator::generate()).collect(),
            }
      }
}

impl  Generator for Country  {
      fn generate() -> Country  {
            Country {
                  name: StringUtils::random_string(10),
                  leagues: (0..10).map(|_| Generator::generate()).collect(),
            }
      }
}

impl Generator for League {
      fn generate() -> League {
            
            let clubs: HashMap<u32, Club> = (0..30)
            .map(|_| Generator::generate())
            .map(|club: Club| (club.id, club))
            .into_iter().collect();

            League {
                  name: StringUtils::random_string(10),
                  clubs,
                  schedule: None,
                  settings: LeagueSettings {
                        season_starting: (1, 1),
                        season_ending: (1, 12),
                  }
            }
      }
}

impl Generator for Club {
      fn generate() -> Club {
            Club {
                  id: IntegerUtils::random(1, 10_000_000) as u32,
                  name: StringUtils::random_string(5),
                  board: ClubBoard::new(),
                  players: (0..10).map(|_| Generator::generate()).collect(),
                  staffs: StaffCollection::new((0..10).map(|_| Generator::generate()).collect()),
                  tactics: None
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
                  StaffPosition::MainCoach
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
