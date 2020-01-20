use crate::chrono::Datelike;
use crate::club::Club;
use crate::core::SimulationContext;
use crate::league::Schedule;
use crate::r#match::{Match, MatchResult};
use crate::utils::TimeEstimation;
use std::collections::HashMap;

use rayon::prelude::*;

#[derive(Debug)]
pub struct League {
    pub name: String,
    pub clubs: Vec<Club>,
    pub schedule: Option<Schedule>,
    pub settings: LeagueSettings,
    pub reputation: u16,
}

impl League {
    pub fn items_count(&self) -> usize {
        self.clubs.iter().map(|club| club.items_count()).sum()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if self.schedule.is_none() || self.settings.is_time_for_new_schedule(context) {
            self.schedule = Some(Schedule::generate(&self.clubs, context.date.date()).unwrap());
        }

        for club in &mut self.clubs {
            club.simulate(context);
        }

        self.play_matches(context);
    }

    fn play_matches(&mut self, context: &SimulationContext) {
        let matches: Vec<Match> = {
            let actual_schedule = self.schedule.as_ref().unwrap();

            let matches_to_play = actual_schedule.get_matches(context.date.date());

            matches_to_play
                .iter()
                .map(|m| {
                    let home_club = self
                        .clubs
                        .iter()
                        .find(|c| c.id == m.home_club_id)
                        .unwrap()
                        .clone();

                    let guest_club = self
                        .clubs
                        .iter()
                        .find(|c| c.id == m.guest_club_id)
                        .unwrap()
                        .clone();

                    Match::make(home_club, guest_club)
                })
                .collect()
        };

        let match_results: Vec<MatchResult> = matches.into_iter().map(|game| game.play()).collect();

        for match_result in match_results {
            //println!("{}", match_result);
        }
    }
}

#[derive(Debug)]
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
    use chrono::prelude::*;
    use chrono::prelude::{NaiveDateTime, NaiveTime};

    #[test]
    fn is_time_for_new_schedule_is_correct() {
        //        let mut settings = LeagueSettings {
        //            season_starting: (1, 3),
        //            season_ending: (4, 5),
        //        };
        //
        //        let mut context = SimulationContext::new(
        //            date: NaiveDate::from_ymd(2020, 3, 1)
        //        );
        //
        //        let result = settings.is_time_for_new_schedule(&mut context);
        //
        //        assert_eq!(true, result);
    }
}
