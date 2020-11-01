use crate::db::loaders::{CountryEntity, CountryLoader, LeagueEntity, LeagueLoader};

mod loaders;
mod generators;

pub use generators::*;

pub struct DatabaseEntity{
    pub countries: Vec<CountryEntity>,
    pub leagues: Vec<LeagueEntity>
}

pub struct DatabaseLoader;

impl DatabaseLoader {
    pub fn load() -> DatabaseEntity{
        DatabaseEntity{
            countries: CountryLoader::load(),
            leagues: LeagueLoader::load()
        }
    }
}

