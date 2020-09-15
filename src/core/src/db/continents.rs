use serde::{Deserialize};

#[derive(Deserialize)]
pub struct DbContinent{
    pub id: u32,
    pub name: String
}

static CONTINENTS_SOURCE: &str = include_str!("data/continents.json");

pub struct Continents {
    continents: Vec<DbContinent>
}

impl Continents {
    pub fn get() -> Vec<DbContinent> {
        serde_json::from_str(CONTINENTS_SOURCE).unwrap()
    }
}