use crate::simulator::SimulatorData;
use crate::league::ScheduleItem;
use chrono::NaiveDateTime;

pub struct LeagueResult{
    pub league_id: u32,
    pub matches: Vec<LeagueMatchResult>
}

impl LeagueResult {
    pub fn new(league_id: u32, matches: Vec<LeagueMatchResult>) -> Self {
        LeagueResult {
            league_id,
            matches
        }
    }

    pub fn process(self, data: &mut SimulatorData){
        let league = data.leagues_mut(self.league_id).unwrap();
        
        for match_item in self.matches {
            //league.league_table.update()
        }
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