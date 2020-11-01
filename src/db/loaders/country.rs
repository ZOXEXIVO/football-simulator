use serde::Deserialize;

const STATIC_COUNTRIES_JSON: &'static str = include_str!("../data/countries.json");

#[derive(Deserialize)]
pub struct CountryEntity{
    pub id: u32,
    pub code: String,
    pub name: String,
    pub continent: String,
    pub reputation: u16
}

pub struct CountryLoader;

impl CountryLoader {
    pub fn load() -> Vec<CountryEntity> {
        serde_json::from_str(STATIC_COUNTRIES_JSON).unwrap()
    }
}