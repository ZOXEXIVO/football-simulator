use crate::context::GlobalContext;
use crate::country::CountryResult;
use crate::league::{League, LeagueMatchResultResult, LeagueResult};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::{
    Club, ClubResult, CountryClubProcessor, CountryLeagueProcessor, CountryMatchProcessor, Team,
};

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
        let mut league_results = CountryLeagueProcessor::process(self, &ctx);
        let match_results = CountryMatchProcessor::process(self, &mut league_results);

        CountryResult::new(
            league_results,
            CountryClubProcessor::process(self, &ctx),
            match_results,
        )
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
