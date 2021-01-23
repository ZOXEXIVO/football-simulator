use crate::simulator::SimulatorData;
use crate::league::{ScheduleItem, LeagueTable};
use crate::r#match::game::MatchResult;
use chrono::NaiveDateTime;

pub struct LeagueResult{
    pub league_id: u32,
    pub scheduled_matches: Vec<LeagueMatchResult>
}

impl LeagueResult {
    pub fn new(league_id: u32, scheduled_matches: Vec<LeagueMatchResult>) -> Self {
        LeagueResult {
            league_id,
            scheduled_matches
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        let league = data.league_mut(self.league_id).unwrap();
        
        let matches = self.scheduled_matches.iter().map(|m| MatchResult {
            league_id: m.league_id,
            schedule_id: m.id.clone(),
            player_changes: vec![],
            home_team_id: m.home_team_id,
            home_goals: m.result.as_ref().unwrap().home_goals,
            away_team_id: m.away_team_id,
            away_goals: m.result.as_ref().unwrap().away_goals
        }).collect();

        league.table.as_mut().unwrap().update(&matches)
    }
}

pub struct LeagueMatchResult {
    pub id: String,
    pub league_id: u32,
    
    pub date: NaiveDateTime,

    pub home_team_id: u32,
    pub away_team_id: u32,

    pub result: Option<LeagueMatchResultResult>
}

pub struct LeagueMatchResultResult {
    pub home_goals: u8,
    pub away_goals: u8
}

impl From<ScheduleItem> for LeagueMatchResult {
    fn from(item: ScheduleItem) -> Self {
        let mut result = LeagueMatchResult{
            id: item.id.clone(),
            league_id: item.league_id,
            date: item.date,
            home_team_id: item.home_team_id,
            away_team_id: item.away_team_id,
            result: None
        };
        
        if let Some(res) = item.result {
            result.result = Some(LeagueMatchResultResult{
                home_goals: res.home_goals,
                away_goals: res.away_goals
            });
        }
        
        result
    }
}