use crate::league::LeagueResult;
use crate::r#match::{MatchEvent, MatchResult};
use crate::simulator::SimulatorData;
use crate::{ClubResult, MatchHistoryItem};

pub struct CountryResult {
    pub leagues: Vec<LeagueResult>,
    pub clubs: Vec<ClubResult>,
    pub match_results: Vec<MatchResult>,
}

impl CountryResult {
    pub fn new(
        leagues: Vec<LeagueResult>,
        clubs: Vec<ClubResult>,
        match_results: Vec<MatchResult>,
    ) -> Self {
        CountryResult {
            leagues,
            clubs,
            match_results,
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        for match_result in &self.match_results {
            Self::process_match_results(match_result, data);
        }

        for league_result in &self.leagues {
            league_result.process(data);
        }

        for club_result in &self.clubs {
            club_result.process(data);
        }
    }

    fn process_match_results(result: &MatchResult, data: &mut SimulatorData) {
        let now = data.date;

        let league = data.league_mut(result.league_id).unwrap();

        league.schedule.as_mut().unwrap().update_match_result(
            &result.schedule_id,
            result.home_goals,
            result.away_goals,
        );

        let home_team = data.team_mut(result.home_team_id).unwrap();
        home_team.match_history.add(MatchHistoryItem::new(
            now,
            result.away_team_id,
            (result.home_goals, result.away_goals),
        ));

        let away_team = data.team_mut(result.away_team_id).unwrap();
        away_team.match_history.add(MatchHistoryItem::new(
            now,
            result.home_team_id,
            (result.away_goals, result.home_goals),
        ));

        process_match_events(result, data);

        fn process_match_events(result: &MatchResult, data: &mut SimulatorData) {
            for match_event in &result.details.as_ref().unwrap().events {
                match match_event {
                    MatchEvent::MatchPlayed(player_id, is_start_squad, minutes_played) => {
                        let mut player = data.player_mut(*player_id).unwrap();

                        if *is_start_squad {
                            player.statistics.played += 1;
                        } else {
                            player.statistics.played_subs += 1;
                        }
                    }
                    MatchEvent::Goal(player_id) => {
                        let mut player = data.player_mut(*player_id).unwrap();
                        player.statistics.goals += 1;
                    }
                    MatchEvent::Assist(player_id) => {
                        let mut player = data.player_mut(*player_id).unwrap();
                        player.statistics.assists += 1;
                    }
                    MatchEvent::Injury(player_id) => {}
                }
            }
        }
    }
}
