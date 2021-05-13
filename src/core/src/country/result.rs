use crate::league::LeagueResult;
use crate::r#match::MatchResult;
use crate::simulator::SimulatorData;
use crate::{ClubResult, MatchHistory};

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
        home_team.match_history.push(MatchHistory::new(
            now,
            result.away_team_id,
            (result.home_goals, result.away_goals),
        ));

        let away_team = data.team_mut(result.away_team_id).unwrap();
        away_team.match_history.push(MatchHistory::new(
            now,
            result.home_team_id,
            (result.away_goals, result.home_goals),
        ));
    }
}
