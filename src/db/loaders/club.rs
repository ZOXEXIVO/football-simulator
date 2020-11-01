use serde::Deserialize;

const STATIC_CLUB_JSON: &'static str = include_str!("../data/clubs.json");

#[derive(Deserialize)]
pub struct ClubEntity{
}

pub struct ClubLoader;

impl ClubLoader {
    pub fn load() -> Vec<ClubEntity> {
        serde_json::from_str(STATIC_CLUB_JSON).unwrap()
    }
}