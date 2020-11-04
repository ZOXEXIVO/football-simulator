use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::{League, LeagueResult};
use crate::{Club, ClubResult, MatchHistory};

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
        let league_results: Vec<LeagueResult> = self
            .leagues
            .iter_mut()
            .map(|league| league.simulate(ctx.with_league(league.id)))
            .collect();
        
        let clubs_results: Vec<ClubResult> = self.clubs.iter_mut()
            .map(|club| club.simulate(ctx.with_club(club.id)))
            .collect();
  
        for league_result in &league_results {
            
        }
        
       // let match_results = self.play_matches(&ctx);

        CountryResult::new(league_results, clubs_results, Vec::new())
    }

    // fn play_matches(&mut self, context: &GlobalContext) -> Vec<MatchResult> {
    //     let current_date = context.simulation.date;
    // 
    //     let matches: Vec<Match> = {
    //         self.schedule_manager.get_matches(current_date)
    //             .iter()
    //             .map(|m| {
    //                 Match::make(&m.id,
    //                             self.get_team(&m.home_team_id),
    //                             self.get_team(&m.away_team_id),
    //                 )
    //             }).collect()
    //     };
    // 
    //     let match_results: Vec<MatchResult> = matches.into_iter().map(|game| game.play()).collect();
    // 
    //     for match_result in &match_results {
    //         self.schedule_manager.update_match_result(&match_result.schedule_id, match_result.home_goals, match_result.away_goals);
    // 
    //         self.add_match_to_team_history(match_result.home_team_id,
    //                                        MatchHistory::new(
    //                                            current_date, match_result.away_team_id,
    //                                            (match_result.home_goals, match_result.away_goals)),
    //         );
    // 
    //         self.add_match_to_team_history(match_result.away_team_id,
    //                                        MatchHistory::new(
    //                                            current_date, match_result.home_team_id,
    //                                            (match_result.away_goals, match_result.home_goals)),
    //         );
    //     }
    // 
    //     match_results
    // }
}
