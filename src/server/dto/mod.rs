use serde::{Serialize};

#[derive(Serialize)]
pub struct ContinentDto<'c> {
    pub name: &'c str
}

#[derive(Serialize)]
pub struct CountryDto<'c> {
    pub name: &'c str
}

#[derive(Serialize)]
pub struct LeagueDto<'c> {
    pub name: &'c str
}

#[derive(Serialize)]
pub struct ClubDto<'c> {
    pub name: &'c str
}

#[derive(Serialize)]
pub struct PlayerDto<'p> {
    pub first_name: &'p str,
    pub last_name: &'p str,
    pub middle_name: &'p str,
}
