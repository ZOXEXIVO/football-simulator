use crate::r#match::engine::PlayerChanges;
use super::engine::FootballEngine;
use crate::Team;

#[derive(Clone)]
pub struct Match<'m> {
    schedule_id: &'m str,
    home_team: &'m Team,
    away_team: &'m Team,
}

impl<'m> Match<'m> {
    pub fn make(schedule_id: &'m str, home_team: &'m Team, away_team: &'m Team) -> Self {
        Match {
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

        MatchResult {
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
    pub schedule_id: String,
    pub player_changes: Vec<PlayerChanges>,
    pub home_team_id: u32,
    pub home_goals: u8,
    pub away_team_id: u32,
    pub away_goals: u8,
}