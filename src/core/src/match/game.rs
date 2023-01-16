use super::engine::FootballEngine;
use crate::r#match::engine::FootballMatchDetails;

use crate::league::LeagueMatch;
use crate::Team;
use log::debug;

#[derive(Clone)]
pub struct Match<'m> {
    id: &'m str,
    league_id: u32,
    pub home_team: &'m Team,
    pub away_team: &'m Team,
}

impl<'m> Match<'m> {
    pub fn make(id: &'m str, league_id: u32, home_team: &'m Team, away_team: &'m Team) -> Self {
        Match {
            id,
            league_id,
            home_team,
            away_team,
        }
    }

    pub fn play(self) -> MatchResult {
        let mut engine = FootballEngine::<150, 100>::new(
            self.home_team.get_match_squad(),
            self.away_team.get_match_squad(),
        );

        let match_details = engine.play();

        debug!(
            "match played: {} {}:{} {}",
            &self.home_team.name,
            match_details.score.home,
            &self.away_team.name,
            match_details.score.away
        );

        MatchResult {
            id: String::from(self.id),
            league_id: self.league_id,
            home_team_id: self.home_team.id,
            home_goals: match_details.score.home,
            away_team_id: self.away_team.id,
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
    pub home_goals: i32,
    pub away_team_id: u32,
    pub away_goals: i32,
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
