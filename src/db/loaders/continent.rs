use serde::Deserialize;

const STATIC_CONTINENTS_JSON: &'static str = include_str!("../data/continents.json");

#[derive(Deserialize)]
pub struct ContinentEntity{
    pub id: u32,
    pub name: String
}

pub struct ContinentLoader;

impl ContinentLoader {
    pub fn load() -> Vec<ContinentEntity> {
        serde_json::from_str(STATIC_CONTINENTS_JSON).unwrap()
    }
}