use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::{League, LeagueResult};
use crate::{Club, ClubResult, Team};
use crate::r#match::{Match, MatchResult};
use log::{debug};

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

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> CountryResult {
        debug!("start simulating country: {}", &self.name);
        
        let club_ids: Vec<u32> = self.clubs
            .iter()
            .map(|c| c.id).collect();
        
        let mut league_results: Vec<LeagueResult> = self
            .leagues
            .iter_mut()
            .map(|league| league.simulate(ctx.with_league(league.id, &club_ids)))
            .collect();
        
        let clubs_results: Vec<ClubResult> = self.clubs.iter_mut()
            .map(|club| club.simulate(ctx.with_club(club.id)))
            .collect();
  
        self.process_league_results(&mut league_results);

        debug!("end simulating country: {}", &self.name);
        
        CountryResult::new(league_results, clubs_results, Vec::new())
    }
    
    fn process_league_results(&mut self, results: &mut Vec<LeagueResult>) {
        let match_results: Vec<MatchResult> = 
            results.iter()
                .flat_map(|lr| &lr.matches)
                .map(|m| 
                    Match::make(&m.id, 
                                self.get_team(m.home_team_id), 
                                self.get_team(m.away_team_id))
                ).map(|m| m.play())
                .collect();

        // for match_result in &match_results {
        //     self.schedule_manager.update_match_result(&match_result.schedule_id, match_result.home_goals, match_result.away_goals);
        // 
        //     self.add_match_to_team_history(match_result.home_team_id,
        //                                    MatchHistory::new(
        //                                        current_date, match_result.away_team_id,
        //                                        (match_result.home_goals, match_result.away_goals)),
        //     );
        // 
        //     self.add_match_to_team_history(match_result.away_team_id,
        //                                    MatchHistory::new(
        //                                        current_date, match_result.home_team_id,
        //                                        (match_result.away_goals, match_result.home_goals)),
        //     );
        // }
    }
    
    fn get_team(&self, id: u32) -> &Team {
        self.clubs.iter().flat_map(|c| &c.teams).find(|team| team.id == id).unwrap()
    }
}
