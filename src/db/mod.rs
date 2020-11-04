use crate::db::loaders::{CountryEntity, CountryLoader, LeagueEntity, LeagueLoader, ClubEntity, ClubLoader};

mod loaders;
mod generators;

pub use generators::*;

pub struct DatabaseEntity{
    pub countries: Vec<CountryEntity>,
    pub leagues: Vec<LeagueEntity>,
    pub clubs: Vec<ClubEntity>
}

pub struct DatabaseLoader;

impl DatabaseLoader {
    pub fn load() -> DatabaseEntity{
        DatabaseEntity{
            countries: CountryLoader::load(),
            leagues: LeagueLoader::load(),
            clubs: ClubLoader::load()
        }
    }
}

