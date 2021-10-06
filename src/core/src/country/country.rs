use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::{League, LeagueResult, LeagueMatchResultResult};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::{Club, ClubResult, Team};

pub struct Country {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub continent_id: u32,
    pub leagues: Vec<League>,
    pub clubs: Vec<Club>,
    pub reputation: u16,
    pub generator_data: CountryGeneratorData,
}

impl Country {
    pub fn new(
        id: u32,
        code: String,
        name: String,
        continent_id: u32,
        leagues: Vec<League>,
        clubs: Vec<Club>,
        reputation: u16,
        generator_data: CountryGeneratorData,
    ) -> Self {
        Country {
            id,
            code,
            name,
            continent_id,
            leagues,
            clubs,
            reputation,
            generator_data,
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> CountryResult {
        let mut league_results = self.simulate_leagues(&ctx);

        let match_results = self.process_matches(&mut league_results);

        let clubs_results: Vec<ClubResult> = self
            .clubs
            .iter_mut()
            .map(|club| {
                let message = &format!("simulate club: {}", &club.name);
                Logging::estimate_result(
                    || club.simulate(ctx.with_club(club.id, &club.name.clone())),
                    message,
                )
            })
            .collect();

        CountryResult::new(league_results, clubs_results, match_results)
    }

    fn simulate_leagues(&mut self, ctx: &GlobalContext<'_>) -> Vec<LeagueResult> {
        let teams_ids: Vec<(u32, u32)> = self
            .clubs
            .iter()
            .flat_map(|c| &c.teams)
            .map(|c| (c.id, c.league_id))
            .collect();

        self.leagues
            .iter_mut()
            .map(|league| {
                let league_team_ids: Vec<u32> = teams_ids
                    .iter()
                    .filter(|(_, league_id)| *league_id == league.id)
                    .map(|(id, _)| *id)
                    .collect();
                {
                    let message = &format!("simulate league: {}", &league.name);
                    Logging::estimate_result(
                        || league.simulate(ctx.with_league(league.id, &league_team_ids)),
                        message,
                    )
                }
            })
            .collect()
    }

    fn process_matches(&mut self, results: &mut Vec<LeagueResult>) -> Vec<MatchResult> {
        let mut result = Vec::new(); //TODO capacity
        
        for league_result in results {
            for scheduled_match in &mut league_result.scheduled_matches {
                let match_to_play = Match::make(
                    scheduled_match.league_id,
                    &scheduled_match.id,
                    self.get_team(scheduled_match.home_team_id),
                    self.get_team(scheduled_match.away_team_id),
                );

                let message = &format!("play match: {} - {}", &match_to_play.home_team.name, &match_to_play.away_team.name);

                let match_result = Logging::estimate_result(|| match_to_play.play(), message);

                scheduled_match.result = Some(LeagueMatchResultResult {
                    home_goals: match_result.home_goals,
                    away_goals: match_result.away_goals
                });
                
                result.push(match_result);
            }
        }

        result 
    }

    fn get_team(&self, id: u32) -> &Team {
        self.clubs
            .iter()
            .flat_map(|c| &c.teams)
            .find(|team| team.id == id)
            .unwrap()
    }
}

pub struct CountryGeneratorData {
    pub people_names: PeopleNameGeneratorData,
}

impl CountryGeneratorData {
    pub fn new(first_names: Vec<String>, last_names: Vec<String>) -> Self {
        CountryGeneratorData {
            people_names: PeopleNameGeneratorData {
                first_names,
                last_names,
            },
        }
    }

    pub fn empty() -> Self {
        CountryGeneratorData {
            people_names: PeopleNameGeneratorData {
                first_names: Vec::new(),
                last_names: Vec::new(),
            },
        }
    }
}

pub struct PeopleNameGeneratorData {
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
}
