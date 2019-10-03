use crate::generators::Generator;

use crate::models::player::FullName;
use crate::models::player::Player;
use crate::models::club::Club;
use crate::models::league::League;
use crate::models::country::Country;

use crate::utils::{StringUtils, IntegerUtils};

use chrono::NaiveDate;

impl Generator for Country {
      fn generate() -> Country {
            let n = 10;

            let mut vec = Vec::with_capacity(n);

            for i in 0..n {
                  vec.push(Generator::generate());
            }

            Country {
               name: StringUtils::random_string(10),
               leagues: vec
            }
      }
}

impl Generator for League {
      fn generate() -> League {
            let n = 10;

            let mut vec = Vec::with_capacity(n);

            for i in 0..n {
                  vec.push(Generator::generate());
            }
            
            League{
                name: StringUtils::random_string(10),
                clubs: vec,
                schedule: None
            }
      }
}

impl Generator for Club {
      fn generate() -> Club {
            let n = 10;

            let mut vec = Vec::with_capacity(n);

            for i in 0..n {
                  vec.push(Generator::generate());
            }

            Club {
               name: StringUtils::random_string(5),
               players: vec
            }
      }
}

impl Generator for Player {
      fn generate() -> Player {
            let n = 10;

            let year = IntegerUtils::random(1980, 2010);
            let month = IntegerUtils::random(1, 12);
            let day = IntegerUtils::random(1, 29);
            
            let player = Player::new(
                        FullName {
                              first_name: StringUtils::random_string(5),
                              last_name: StringUtils::random_string(10),
                              middle_name: StringUtils::random_string(15),
                        },
                        NaiveDate::from_ymd(year as i32, month, day)
                  );

            player
      }
}
