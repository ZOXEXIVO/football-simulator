use serde::Deserialize;

const STATIC_LEAGUES_JSON: &'static str = include_str!("../data/leagues.json");

#[derive(Deserialize)]
pub struct LeagueEntity{
    pub id: u32,
    pub name: String,
    pub country_id: u32,
    pub settings: LeagueSettingsEntity,
    pub reputation: u16,
}

#[derive(Deserialize)]
pub struct LeagueSettingsEntity{
    pub season_starting_half: DayMonthPeriodEntity,
    pub season_ending_half: DayMonthPeriodEntity,
}

#[derive(Debug, Deserialize)]
pub struct DayMonthPeriodEntity {
    pub from_day: u8,
    pub from_month: u8,

    pub to_day: u8,
    pub to_month: u8
}

pub struct LeagueLoader;

impl LeagueLoader {
    pub fn load() -> Vec<LeagueEntity> {
        serde_json::from_str(STATIC_LEAGUES_JSON).unwrap()
    }
}