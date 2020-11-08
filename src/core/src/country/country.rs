use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::{League, LeagueResult, LeagueMatchResult};
use crate::{Club, ClubResult, MatchHistory, Team};
use crate::r#match::Match;

pub struct Country {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub leagues: Vec<League>,
    pub clubs: Vec<Club>,
    pub reputation: u16,
}

impl Country {
    pub fn new(
        id: u32,
        code: String,
        name: String,
        leagues: Vec<League>,
        clubs: Vec<Club>,
        reputation: u16,
    ) -> Self {
        Country {
            id,
            code,
            name,
            leagues,
            clubs,
            reputation,
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> CountryResult {
        let mut league_results: Vec<LeagueResult> = self
            .leagues
            .iter_mut()
            .map(|league| league.simulate(ctx.with_league(league.id)))
            .collect();
        
        let clubs_results: Vec<ClubResult> = self.clubs.iter_mut()
            .map(|club| club.simulate(ctx.with_club(club.id)))
            .collect();
  
        self.process_league_results(&mut league_results);

        CountryResult::new(league_results, clubs_results, Vec::new())
    }
    
    fn process_league_results(&mut self, results: &mut Vec<LeagueResult>) {
        let matches_to_play: Vec<LeagueMatchResult> = 
            results.iter()
                .flat_map(|lr| lr.matches)
                .map(|m| 
                    Match::make(&m.id, 
                                self.get_team(m.home_team_id), 
                                self.get_team(m.away_team_id))
                ).map(|m| m.play())
                .collect();

        for match_result in &match_results {
                    self.schedule_manager.update_match_result(&match_result.schedule_id, match_result.home_goals, match_result.away_goals);
            
                    self.add_match_to_team_history(match_result.home_team_id,
                                                   MatchHistory::new(
                                                       current_date, match_result.away_team_id,
                                                       (match_result.home_goals, match_result.away_goals)),
                    );
            
                    self.add_match_to_team_history(match_result.away_team_id,
                                                   MatchHistory::new(
                                                       current_date, match_result.home_team_id,
                                                       (match_result.away_goals, match_result.home_goals)),
                    );
                }
            
    }
    
    fn get_team(&self, id: u32) -> &Team {
        self.clubs.iter().flat_map(|c| &c.teams).find(|team| team.id == id).unwrap()
    }
}
