
use crate::player::player::Player;
use crate::core::context::SimulationContext;

pub use chrono::prelude::{NaiveDate, DateTime, Utc, Datelike};

pub struct PlayerClubContract {
      player: Player,
      salary: f64,
      expired: NaiveDate,
}

impl PlayerClubContract {
      pub fn new(player: Player, expired: NaiveDate) -> Self {
            PlayerClubContract {
                  player: player,
                  salary: 100000.0,
                  expired: expired,
            }
      }

      pub fn is_expired(&self) -> bool {
            let now = Utc::now();

            let naive_now = NaiveDate::from_ymd(
                  now.year(), now.month(), now.day()
            );

            self.expired >= naive_now
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            self.player.simulate(context);
      }
}
