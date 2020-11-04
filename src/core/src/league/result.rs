use crate::simulator::SimulatorData;
use crate::r#match::MatchResult;
use crate::league::ScheduleItem;

pub struct LeagueResult{
    id: u32,
    matches: Vec<ScheduleItem>
}

impl LeagueResult {
    pub fn new(id: u32, matches: Vec<ScheduleItem>) -> Self {
        LeagueResult {
            id,
            matches
        }
    }

    pub fn process(self, data: &mut SimulatorData){
        let league = data.leagues_mut(self.id).unwrap();
        
        for match_item in self.matches {
            //league.league_table.update()
        }
    }
}