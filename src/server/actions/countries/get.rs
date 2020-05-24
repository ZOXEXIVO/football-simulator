use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};
use crate::country::Country;

#[derive(Deserialize)]
pub struct CountryGetRequest {
    game_id: String,
    country_id: u32
}

#[derive(Serialize)]
pub struct CountryGetResponse<'c> {
    pub country: CountryDto<'c>
}

#[derive(Serialize)]
pub struct CountryDto<'c> {
    pub id: u32,
    pub name: &'c str,
    pub leagues: Vec<LeagueDto<'c>>
}

#[derive(Serialize)]
pub struct LeagueDto<'l> {
    pub id: u32,
    pub name: &'l str,
}

pub async fn country_get_action(route_params: web::Path<CountryGetRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let country: &Country = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .find(|country| country.id == route_params.country_id)
        .unwrap();

    let result = CountryGetResponse {
        country: CountryDto {
            id: country.id,
            name: &country.name,
            leagues: country.leagues.iter().map(|l| LeagueDto {
                id: l.id,
                name: &l.name,
            }).collect()
        }
    };

    Ok(HttpResponse::Ok().json(result))
}
