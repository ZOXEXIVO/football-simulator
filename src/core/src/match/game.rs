use super::engine::FootballEngine;
use crate::r#match::{MatchResult, TeamSquad};
use log::debug;

#[derive(Debug, Clone)]
pub struct Match {
    id: String,
    league_id: u32,
    pub home_squad: TeamSquad,
    pub away_squad: TeamSquad,
}

impl Match {
    pub fn make(id: String, league_id: u32, home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        Match {
            id,
            league_id,
            home_squad,
            away_squad,
        }
    }

    pub fn play(self) -> MatchResult {
        let home_team_id = self.home_squad.team_id;
        let home_team_name = String::from(&self.home_squad.team_name);

        let away_team_id = self.away_squad.team_id;
        let away_team_name = String::from(&self.away_squad.team_name);

        let match_raw_result = FootballEngine::<840, 545>::play(self.home_squad, self.away_squad);

        debug!(
            "match played: {} {}:{} {}",
            home_team_name,
            match_raw_result.score.home,
            away_team_name,
            match_raw_result.score.away
        );

        MatchResult {
            id: String::from(self.id),
            league_id: self.league_id,
            score: match_raw_result.score.clone(),
            home_team_id,
            away_team_id,
            result_details: Some(match_raw_result),
        }
    }
}
