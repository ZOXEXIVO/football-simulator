use crate::r#match::position::MatchPositionData;

#[derive(Debug, Clone)]
pub struct FootballMatchResult {
    pub score: Score,
    pub position_data: MatchPositionData,

    pub home_players: FieldSquad,
    pub away_players: FieldSquad,

    pub match_time_ms: u64,
    pub additinal_time_ms: u64,
}

impl FootballMatchResult {
    pub fn with_match_time(match_time_ms: u64) -> Self {
        FootballMatchResult {
            score: Score::new(),
            position_data: MatchPositionData::new(),
            home_players: FieldSquad::new(),
            away_players: FieldSquad::new(),
            match_time_ms,
            additinal_time_ms: 0,
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
    pub assistant: Option<u32>,
    pub minute: u8,
}

impl Score {
    pub fn new() -> Self {
        Score {
            home: 0,
            away: 0,
            details: Vec::new(),
        }
    }
}
