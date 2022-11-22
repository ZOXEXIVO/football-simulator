use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::LeagueCollection;
use crate::utils::Logging;
use crate::{Club, ClubResult};

pub struct Country {
    pub id: u32,
    pub code: String,
    pub slug: String,
    pub name: String,
    pub continent_id: u32,
    pub leagues: LeagueCollection,
    pub clubs: Vec<Club>,
    pub reputation: u16,
    pub generator_data: CountryGeneratorData,
}

impl Country {
    pub fn new(
        id: u32,
        code: String,
        slug: String,
        name: String,
        continent_id: u32,
        leagues: LeagueCollection,
        clubs: Vec<Club>,
        reputation: u16,
        generator_data: CountryGeneratorData,
    ) -> Self {
        Country {
            id,
            code,
            slug,
            name,
            continent_id,
            leagues,
            clubs,
            reputation,
            generator_data,
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> CountryResult {
        let mut league_results = self.leagues.simulate(&self.clubs, &ctx);

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

        CountryResult::new(league_results, clubs_results)
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
