use crate::club::{Club, ClubResult};
use crate::league::{Schedule, LeagueResult};
use crate::r#match::{Match, MatchResult};
use crate::simulator::context::GlobalContext;
use crate::simulator::SimulationContext;
use chrono::Datelike;

#[derive(Debug)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub clubs: Vec<Club>,
    pub schedule: Option<Schedule>,
    pub settings: LeagueSettings,
    pub reputation: u16,
}

impl League {
    pub fn simulate(&mut self, ctx: GlobalContext) -> LeagueResult {
        if self.schedule.is_none() || self.settings.is_time_for_new_schedule(&ctx.simulation) {
            self.schedule =
                Some(Schedule::generate(&self.clubs, ctx.simulation.date.date()).unwrap());
        }

        let club_result: Vec<ClubResult> = self.clubs.iter_mut()
            .map(|club| club.simulate(ctx.with_club(club.id)))
            .collect();

        self.play_matches(&ctx);

        LeagueResult::new(club_result)
    }

    fn get_club(&self, club_id: u32) -> Option<&Club> {
        self.clubs.iter().find(|c| c.id == club_id)
    }

    fn play_matches(&mut self, context: &GlobalContext) {
        let matches: Vec<Match> = {
            let actual_schedule = self.schedule.as_ref().unwrap();

            let matches_to_play = actual_schedule.get_matches(context.simulation.date.date());

            matches_to_play
                .iter()
                .map(|m| {
                    Match::make(self.get_club(m.home_club_id).unwrap(),
                                self.get_club(m.guest_club_id).unwrap(),
                    )
                })
                .collect()
        };

        let match_results: Vec<MatchResult> = matches.into_iter().map(|game| game.play()).collect();

        for match_result in match_results {}
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
