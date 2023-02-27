use super::engine::FootballEngine;
use crate::r#match::engine::FootballMatchDetails;

use crate::league::LeagueMatch;
use crate::r#match::TeamSquad;
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

        let mut engine = FootballEngine::<840, 545>::new(self.home_squad, self.away_squad);

        let mut match_details = engine.play();

        debug!(
            "match played: {} {}:{} {}",
            home_team_name, match_details.score.home, away_team_name, match_details.score.away
        );

        MatchResult {
            id: String::from(self.id),
            league_id: self.league_id,
            home_team_id,
            home_goals: match_details.score.home,
            away_team_id,
            away_goals: match_details.score.away,
            details: Some(match_details),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub id: String,
    pub league_id: u32,
    pub details: Option<FootballMatchDetails>,
    pub home_team_id: u32,
    pub home_goals: u8,
    pub away_team_id: u32,
    pub away_goals: u8,
}

impl From<&LeagueMatch> for MatchResult {
    fn from(m: &LeagueMatch) -> Self {
        MatchResult {
            id: m.id.clone(),
            league_id: m.league_id,
            details: None,
            home_team_id: m.home_team_id,
            home_goals: m.result.as_ref().unwrap().home_goals,
            away_team_id: m.away_team_id,
            away_goals: m.result.as_ref().unwrap().away_goals,
        }
    }
}
