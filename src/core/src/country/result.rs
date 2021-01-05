use crate::league::LeagueResult;
use crate::simulator::SimulatorData;
use crate::{ClubResult};
use crate::r#match::MatchResult;

pub struct CountryResult{
    pub leagues: Vec<LeagueResult>,
    pub clubs: Vec<ClubResult>,
    pub match_results: Vec<MatchResult>
}

impl CountryResult {
    pub fn new(leagues: Vec<LeagueResult>, clubs: Vec<ClubResult>, match_results: Vec<MatchResult>) -> Self {
        CountryResult {           
            leagues, 
            clubs,
            match_results
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        for league_result in &self.leagues {
            league_result.process(data);
        }

        for club_result in &self.clubs {
            club_result.process(data);
        }
        
        for match_result in &self.match_results {
            Self::process_match_results(match_result, data);
        }
    }
    
    fn process_match_results(result: &MatchResult, data: &mut SimulatorData){
        let league = data.league_mut(result.league_id).unwrap();

        league.schedule.update_match_result(&result.schedule_id,
                                                    result.home_goals, result.away_goals);

        // self.add_match_to_team_history(match_result.home_team_id,
        //                                MatchHistory::new(
        //                                    current_date, match_result.away_team_id,
        //                                    (match_result.home_goals, match_result.away_goals)),
        // );
        // 
        // self.add_match_to_team_history(match_result.away_team_id,
        //                                MatchHistory::new(
        //                                    current_date, match_result.home_team_id,
        //                                    (match_result.away_goals, match_result.home_goals)),
        // );
    }
}