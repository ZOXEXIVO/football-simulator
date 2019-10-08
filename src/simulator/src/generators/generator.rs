use crate::models::club::Club;
use crate::models::country::Country;
use crate::models::league::League;
use crate::models::player::FullName;
use crate::models::player::Player;

use crate::utils::{IntegerUtils, StringUtils};

use chrono::NaiveDate;

pub trait Generator{
    fn generate() -> Self;
}

impl Generator for Country {
      fn generate() -> Country {
            Country {
                  name: StringUtils::random_string(10),
                  leagues: (0..20).map(|_| Generator::generate()).collect(),
            }
      }
}

impl Generator for League {
      fn generate() -> League {
            League {
                  name: StringUtils::random_string(10),
                  clubs: (0..30).map(|_| Generator::generate()).collect(),
                  schedule: None,
            }
      }
}

impl Generator for Club {
      fn generate() -> Club {
            Club {
                  name: StringUtils::random_string(5),
                  players: (0..100).map(|_| Generator::generate()).collect(),
            }
      }
}

impl Generator for Player {
      fn generate() -> Player {
            let year = IntegerUtils::random(1980, 2010);
            let month = IntegerUtils::random(1, 12);
            let day = IntegerUtils::random(1, 29);

            return Player::new(
                  FullName {
                        first_name: StringUtils::random_string(5),
                        last_name: StringUtils::random_string(10),
                        middle_name: StringUtils::random_string(15),
                  },
                  NaiveDate::from_ymd(year as i32, month, day),
            );
      }
}
