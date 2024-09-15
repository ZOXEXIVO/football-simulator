use super::engine::FootballEngine;
use crate::r#match::{MatchResult, TeamSquad};
use log::debug;

#[derive(Debug, Clone)]
pub struct Match {
    id: String,
    league_id: u32,
    pub left_squad: TeamSquad,
    pub right_squad: TeamSquad,
}

impl Match {
    pub fn make(id: String, league_id: u32, home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        Match {
            id,
            league_id,
            left_squad: home_squad,
            right_squad: away_squad,
        }
    }

    pub fn play(self) -> MatchResult {
        let home_team_id = self.left_squad.team_id;
        let home_team_name = String::from(&self.left_squad.team_name);

        let away_team_id = self.right_squad.team_id;
        let away_team_name = String::from(&self.right_squad.team_name);

        let match_raw_result = FootballEngine::<840, 545>::play(self.left_squad, self.right_squad);

        debug!(
            "match played: {} {}:{} {}",
            home_team_name,
            match_raw_result.score.home_team.score,
            away_team_name,
            match_raw_result.score.away_team.score,
        );

        MatchResult {
            id: String::from(self.id),
            league_id: self.league_id,
            home_team_id,
            away_team_id,
            score: match_raw_result.score.clone(),
            result_details: Some(match_raw_result),
        }
    }
}
