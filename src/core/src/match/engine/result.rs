﻿use crate::league::LeagueMatch;
use crate::r#match::position::MatchPositionData;
use crate::r#match::{MatchPlayer, TeamSquad};
use crate::r#match::statistics::MatchStatisticType;

#[derive(Debug, Clone)]
pub struct MatchResultRaw {
    pub score: Score,

    pub position_data: MatchPositionData,

    pub home_players: FieldSquad,
    pub away_players: FieldSquad,

    pub match_time_ms: u64,
    pub additional_time_ms: u64,
}

impl MatchResultRaw {
    pub fn with_match_time(match_time_ms: u64) -> Self {
        MatchResultRaw {
            score: Score::new(),
            position_data: MatchPositionData::new(),
            home_players: FieldSquad::new(),
            away_players: FieldSquad::new(),
            match_time_ms,
            additional_time_ms: 0,
        }
    }

    pub fn write_team_players(
        &mut self,
        home_team_players: &FieldSquad,
        away_team_players: &FieldSquad,
    ) {
        self.home_players = FieldSquad::from(home_team_players);
        self.away_players = FieldSquad::from(away_team_players);
    }

    pub fn fill_details(&mut self, players: Vec<&MatchPlayer>){
        for player in players.iter().filter(|p| !p.statistics.is_empty()) {
            for stat in &player.statistics.items            {
                let detail = GoalDetail{
                    player_id: player.player_id,
                    match_second: stat.match_second,
                    stat_type: stat.stat_type
                };

                self.score.add_goal_detail(detail);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSquad {
    pub main: Vec<u32>,
    pub substitutes: Vec<u32>,
}

impl FieldSquad {
    pub fn new() -> Self {
        FieldSquad {
            main: Vec::new(),
            substitutes: Vec::new(),
        }
    }

    pub fn from(field_squad: &FieldSquad) -> Self {
        FieldSquad {
            main: field_squad.main.to_vec(),
            substitutes: field_squad.substitutes.to_vec(),
        }
    }

    pub fn from_team(squad: &TeamSquad) -> Self {
        FieldSquad {
            main: squad.main_squad.iter().map(|p| p.player_id).collect(),
            substitutes: squad.substitutes.iter().map(|p| p.player_id).collect(),
        }
    }

    pub fn count(&self) -> usize {
        self.main.len() + self.substitutes.len()
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub home: u8,
    pub away: u8,
    pub details: Vec<GoalDetail>,
}

#[derive(Debug, Clone)]
pub struct GoalDetail {
    pub player_id: u32,
    pub stat_type: MatchStatisticType,

    pub match_second: u64
}

impl Score {
    pub fn new() -> Self {
        Score {
            home: 0,
            away: 0,
            details: Vec::new(),
        }
    }

    pub fn add_goal_detail(&mut self, goal_detail: GoalDetail){
        self.details.push(goal_detail)
    }

    pub fn detail(&self) -> &[GoalDetail]{
        &self.details
    }
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub id: String,
    pub league_id: u32,
    pub result_details: Option<MatchResultRaw>,
    pub score: Score,
    pub home_team_id: u32,
    pub away_team_id: u32,
}

impl From<&LeagueMatch> for MatchResult {
    fn from(m: &LeagueMatch) -> Self {
        MatchResult {
            id: m.id.clone(),
            league_id: m.league_id,
            score: Score::new(),
            result_details: None,
            home_team_id: m.home_team_id,
            away_team_id: m.away_team_id,
        }
    }
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}