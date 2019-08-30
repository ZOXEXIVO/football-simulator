use crate::models::player::FullName;
use crate::models::player::Player;
use crate::models::club::Club;
use crate::models::league::League;
use crate::models::country::Country;

use crate::utils::{StringUtils, IntegerUtils};

use chrono::NaiveDate;

pub struct CountryGenerator {}

impl CountryGenerator {
      pub fn generate(count: usize) -> Vec<Country> {
            let mut res = Vec::with_capacity(count);

            for _ in 0..count {
                  let country = Country {
                        name: StringUtils::random_string(10),
                        leagues: LeagueGenerator::generate(3),
                  };

                  res.push(country)
            }

            res
      }
}

pub struct LeagueGenerator {}

impl LeagueGenerator {
      pub fn generate(count: usize) -> Vec<League> {
            let mut res = Vec::with_capacity(count);

            for _ in 0..count {
                  let country = League::new(
                        StringUtils::random_string(10),
                        ClubGenerator::generate(50),
                  );

                  res.push(country)
            }

            res
      }
}

pub struct ClubGenerator {}

impl ClubGenerator {
      pub fn generate(count: usize) -> Vec<Club> {
            let mut res = Vec::with_capacity(count);

            for _i in 0..count {
                  let club = Club {
                        name: StringUtils::random_string(5),
                        players: PlayerGenerator::generate(60),
                  };

                  res.push(club)
            }

            res
      }
}

pub struct PlayerGenerator {}

impl PlayerGenerator {
      pub fn generate(count: usize) -> Vec<Player> {
            let mut res = Vec::with_capacity(count);

            for _i in 0..count {
                  let player = Player::new(
                        FullName {
                              first_name: StringUtils::random_string(5),
                              last_name: StringUtils::random_string(10),
                              middle_name: StringUtils::random_string(15),
                        },
                        PlayerGenerator::get_random_birthday() 
                  );

                  res.push(player);
            }

            res
      }

      fn get_random_birthday() -> NaiveDate {
          let year = IntegerUtils::random(1980, 2010);
          let month = IntegerUtils::random(1, 12);
          let day = IntegerUtils::random(1, 29);

          NaiveDate::from_ymd(year as i32, month, day)
      }
}
