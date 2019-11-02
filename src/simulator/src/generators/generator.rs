use crate::models::staff::contract::StaffClubContract;
use crate::models::staff::staff::Staff;
use crate::models::club::Club;
use crate::models::country::Country;
use crate::models::league::League;
use crate::models::player::*;
use crate::models::Schedule;

use crate::utils::{IntegerUtils, StringUtils};

use chrono::NaiveDate;


pub trait Generator {
      fn generate(index: i32) -> Self;
}

impl  Generator for Country  {
      fn generate(index: i32) -> Country  {
            Country {
                  name: index.to_string(),
                  leagues: (0..3).map(|i| Generator::generate(i)).collect(),
            }
      }
}

impl Generator for League {
      fn generate(index: i32) -> League {
            League {
                  name: StringUtils::random_string(10),
                  clubs: (0..30).map(|i| Generator::generate(i)).collect(),
                  schedule: None,
            }
      }
}

impl Generator for Club {
      fn generate(index: i32) -> Club {
            Club {
                  id: IntegerUtils::random(1, 10000000),
                  name: StringUtils::random_string(5),
                  players: (0..10).map(|i| Generator::generate(i)).collect(),
                  staffs: (0..10).map(|i| Generator::generate(i)).collect(),
            }
      }
}

impl Generator for PlayerClubContract {
      fn generate(index: i32) -> PlayerClubContract {
            return PlayerClubContract::new(
                  generate_player(),
                  NaiveDate::from_ymd(2020, 3, 14)
            );

            fn generate_player() -> Player {
                  let year = IntegerUtils::random(1980, 2010);
                  let month = IntegerUtils::random(1, 12);
                  let day = IntegerUtils::random(1, 29);

                  Player::new(
                        IntegerUtils::random(1, 1000000),
                        FullName {                              
                              first_name: StringUtils::random_string(5),
                              last_name: StringUtils::random_string(10),
                              middle_name: StringUtils::random_string(15),
                        },
                        NaiveDate::from_ymd(year as i32, month, day),
                  )
            }
      }
}

impl Generator for StaffClubContract {
      fn generate(index: i32) -> StaffClubContract {
            return StaffClubContract::new(
                  generate_staff(),
                  NaiveDate::from_ymd(2020, 3, 14)
            );

            fn generate_staff() -> Staff {
                  let year = IntegerUtils::random(1980, 2010);
                  let month = IntegerUtils::random(1, 12);
                  let day = IntegerUtils::random(1, 29);

                  Staff::new(
                        IntegerUtils::random(1, 10000000),
                        FullName {                              
                              first_name: StringUtils::random_string(5),
                              last_name: StringUtils::random_string(10),
                              middle_name: StringUtils::random_string(15),
                        },
                        NaiveDate::from_ymd(year as i32, month, day),
                  )
            }
      }
}
