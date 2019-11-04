use crate::chrono::Datelike;
use crate::club::Club;
use crate::core::SimulationContext;
use crate::play::{Match, MatchResult};
use crate::schedule::Schedule;
use std::fmt::{Display, Formatter, Result};

pub struct League {
      pub name: String,
      pub clubs: Vec<Club>,
      pub schedule: Option<Schedule>,
      pub settings: LeagueSettings,
}

impl League {
      pub fn items_count(&self) -> usize {
            return self.clubs.iter().map(|club| club.items_count()).sum();
      }

      pub fn get_club(&self, id: u32) -> &Club {
            self.clubs.iter().find(|c| c.id == id).unwrap()
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            if self.schedule.is_none() || self.settings.is_time_for_new_schedule(context) {
                  self.schedule = Some(Schedule::generate(&self.clubs, context.date).unwrap());
            }

            for club in &mut self.clubs {
                  club.simulate(context);
            }

            let matches_to_play = self.schedule.as_ref().unwrap().get_matches(context.date);

            for m in matches_to_play {
                  let home_club = self.get_club(m.home_club_id);
                  let away_club = self.get_club(m.guest_club_id);

                  let mut club_match = Match::make(home_club, away_club);

                  let match_result = club_match.play();

                  println!("{}", match_result);
            }
      }
}

pub struct LeagueSettings {
      pub season_starting: (u8, u8),
      pub season_ending: (u8, u8),
}

impl LeagueSettings {
      pub fn is_time_for_new_schedule(&self, context: &SimulationContext) -> bool {
            let current_day = context.date.day() as u8;
            let current_month = context.date.month() as u8;

            current_day == self.season_starting.0 && current_month == self.season_starting.1
      }
}

#[cfg(test)]
mod tests {
      use super::*;

      #[test]
      fn is_time_for_new_schedule_is_correct() {
            let mut settings = LeagueSettings {
                  season_starting: (1, 3),
                  season_ending: (4, 5),
            };

            let mut context = SimulationContext {
                  events: vec![],
                  date: NaiveDate::from_ymd(2020, 3, 1),
            };

            let result = settings.is_time_for_new_schedule(&mut context);

            assert_eq!(true, result);
      }
}
