use crate::league::ScheduleItem;
use crate::r#match::game::MatchResult;
use crate::r#match::FootballMatchDetails;
use crate::simulator::SimulatorData;
use chrono::NaiveDateTime;

pub struct LeagueResult {
    pub league_id: u32,
    pub scheduled_matches: Vec<LeagueMatch>,
}

impl LeagueResult {
    pub fn new(league_id: u32, scheduled_matches: Vec<LeagueMatch>) -> Self {
        LeagueResult {
            league_id,
            scheduled_matches,
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        let league = data.league_mut(self.league_id).unwrap();

        let matches = self
            .scheduled_matches
            .iter()           
            .map(|lm| MatchResult::from(lm))
            .collect();

        league.table.as_mut().unwrap().update(&matches)
    }
}

pub struct LeagueMatch {
    pub id: String,
    pub league_id: u32,

    pub date: NaiveDateTime,

    pub home_team_id: u32,
    pub away_team_id: u32,

    pub result: Option<LeagueMatchResultResult>,
}

pub struct LeagueMatchResultResult {
    pub home_goals: i32,
    pub away_goals: i32
}

impl From<ScheduleItem> for LeagueMatch {
    fn from(item: ScheduleItem) -> Self {
        let mut result = LeagueMatch {
            id: item.id.clone(),
            league_id: item.league_id,
            date: item.date,
            home_team_id: item.home_team_id,
            away_team_id: item.away_team_id,
            result: None,
        };

        if let Some(res) = item.result {
            result.result = Some(LeagueMatchResultResult {
                home_goals: res.home_goals,
                away_goals: res.away_goals
            });
        }

        result
    }
}
