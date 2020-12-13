use crate::db::loaders::{CountryEntity, CountryLoader, LeagueEntity, LeagueLoader, ClubEntity, ClubLoader, ContinentEntity, ContinentLoader, NamesByCountryEntity, NamesByCountryLoader};

mod loaders;
mod generators;

pub use generators::*;

pub struct DatabaseEntity{
    pub continents: Vec<ContinentEntity>,
    pub countries: Vec<CountryEntity>,
    pub leagues: Vec<LeagueEntity>,
    pub clubs: Vec<ClubEntity>,
    
    pub names_by_country: Vec<NamesByCountryEntity>
}

pub struct DatabaseLoader;

impl DatabaseLoader {
    pub fn load() -> DatabaseEntity{
        DatabaseEntity{
            continents: ContinentLoader::load(),
            countries: CountryLoader::load(),
            leagues: LeagueLoader::load(),
            clubs: ClubLoader::load(),
            names_by_country: NamesByCountryLoader::load()
        }
    }
}

