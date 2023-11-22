use super::engine::FootballEngine;

use crate::league::LeagueMatch;
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

        let match_results = FootballEngine::<840, 545>::play(self.home_squad, self.away_squad);

        debug!(
            "match played: {} {}:{} {}",
            home_team_name, match_results.score.home, away_team_name, match_results.score.away
        );

        MatchResult {
            id: String::from(self.id),
            league_id: self.league_id,
            home_team_id,
            home_goals: match_results.score.home,
            away_team_id,
            away_goals: match_results.score.away,
            result_details: Some(match_results),
        }
    }
}