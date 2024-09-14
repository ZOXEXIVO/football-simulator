use crate::league::LeagueMatch;
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
    pub fn with_match_time(match_time_ms: u64, team_a_id: u32, team_b_id: u32) -> Self {
        MatchResultRaw {
            score: Score::new(team_a_id, team_b_id),
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
    pub team_a: TeamScore,
    pub team_b: TeamScore,
    pub details: Vec<GoalDetail>,
}

#[derive(Debug, Clone)]
pub struct TeamScore {
    pub team_id: u32,
    pub score: u8
}

impl TeamScore {
    pub fn new(team_id: u32) -> Self {
        TeamScore {
            team_id,
            score: 0
        }
    }

    pub fn new_with_score(team_id: u32, score: u8) -> Self {
        TeamScore {
            team_id,
            score
        }
    }
}
impl From<&TeamScore> for TeamScore {
    fn from(team_score: &TeamScore) -> Self {
        TeamScore::new_with_score(team_score.team_id, team_score.score)
    }
}

impl PartialEq<Self> for TeamScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for TeamScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

#[derive(Debug, Clone)]
pub struct GoalDetail {
    pub player_id: u32,
    pub stat_type: MatchStatisticType,

    pub match_second: u64
}

impl Score {
    pub fn new(team_a_id: u32, team_b_id: u32) -> Self {
        Score {
            team_a: TeamScore::new(team_a_id),
            team_b: TeamScore::new(team_b_id),
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
    pub score: Score
}

impl From<&LeagueMatch> for MatchResult {
    fn from(m: &LeagueMatch) -> Self {
        MatchResult {
            id: m.id.clone(),
            league_id: m.league_id,
            score: Score::new(m.home_team_id, m.away_team_id),
            result_details: None
        }
    }
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}