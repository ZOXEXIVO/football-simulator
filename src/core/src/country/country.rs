use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::{League, LeagueResult};
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
        let league_results = self.simulate_leagues(&ctx);

        let match_results = self.process_matches(&league_results);

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

    fn process_matches(&mut self, results: &Vec<LeagueResult>) -> Vec<MatchResult> {
        results
            .iter()
            .flat_map(|lr| &lr.scheduled_matches)
            .map(|m| {
                Match::make(
                    m.league_id,
                    &m.id,
                    self.get_team(m.home_team_id),
                    self.get_team(m.away_team_id),
                )
            })
            .map(|m| {
                let message = &format!("play match: {} - {}", &m.home_team.name, &m.away_team.name);
                Logging::estimate_result(|| m.play(), message)
            })
            .collect()
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
