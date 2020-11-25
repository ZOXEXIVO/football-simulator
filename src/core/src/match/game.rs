use crate::r#match::engine::PlayerChanges;
use super::engine::FootballEngine;
use crate::Team;

use log::{debug};

#[derive(Clone)]
pub struct Match<'m> {
    league_id: u32,
    schedule_id: &'m str,
    pub home_team: &'m Team,
    pub away_team: &'m Team,
}

impl<'m> Match<'m> {
    pub fn make(league_id: u32, schedule_id: &'m str, home_team: &'m Team, away_team: &'m Team) -> Self {
        Match {
            league_id,
            schedule_id,
            home_team,
            away_team,
        }
    }

    pub fn play(self) -> MatchResult {
        let mut engine = FootballEngine::new(
            self.home_team.get_match_squad(),
            self.away_team.get_match_squad(),
        );

        let play_result = engine.play();

        debug!("match played: {} {}:{} {}", 
               &self.home_team.name, play_result.score.home,
               &self.away_team.name, play_result.score.away);
        
        MatchResult {
            league_id: self.league_id,
            schedule_id: String::from(self.schedule_id),
            player_changes: play_result.player_changes,
            home_team_id: self.home_team.id,
            home_goals: play_result.score.home,
            away_team_id: self.away_team.id,
            away_goals: play_result.score.away,
        }
    }
}

pub struct MatchResult  {
    pub league_id: u32,
    pub schedule_id: String,
    pub player_changes: Vec<PlayerChanges>,
    pub home_team_id: u32,
    pub home_goals: u8,
    pub away_team_id: u32,
    pub away_goals: u8,
}