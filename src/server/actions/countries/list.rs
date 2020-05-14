use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CountryListRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct LeagueDto<'l> {
    pub id: u32,
    pub name: &'l str   
}

#[derive(Serialize)]
pub struct CountryDto<'c> {
    pub id: u32,
    pub name: &'c str,
    pub leagues: Vec<LeagueDto<'c>>
}

#[derive(Serialize)]
pub struct ContinentDto<'c> {
    pub name: &'c str,
    pub countries: Vec<CountryDto<'c>>
}

#[derive(Serialize)]
pub struct CountryListResponse<'c> {
    pub continents: Vec<ContinentDto<'c>>
}

pub async fn country_list_action(route_params: web::Path<CountryListRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let mut response = CountryListResponse {
        continents: Vec::with_capacity(simulator_data.continents.len())
    };
    
    for continent in &simulator_data.continents {
        let item = ContinentDto {
            name: &continent.name,
            countries: continent.countries.iter().map(|country|
                CountryDto {
                    id: country.id,
                    name: &country.name,
                    leagues: country.leagues.iter().map(|l| LeagueDto{
                        id: l.id,
                        name: &l.name
                    }).collect()
                }
            ).collect()
        };
        
        response.continents.push(item);
    }
    
    Ok(HttpResponse::Ok().json(response))
}
