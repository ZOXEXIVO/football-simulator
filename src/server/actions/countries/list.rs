use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA, CountryDto};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CountryListRequest {
    game_id: String
}

#[derive(Serialize)]
pub struct CountryListResponse<'c> {
    pub countries: Vec<CountryDto<'c>>
}

pub async fn country_list_action(route_params: web::Path<CountryListRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let simulator_data = data.lock().unwrap();

    let countries = simulator_data.continents.iter()
        .flat_map(|c| &c.countries);
    
    let json_result = serde_json::to_string(&CountryListResponse{
        countries: countries.map(|c| CountryDto {
            name: &c.name
        }).collect()
    }).unwrap();
    
    Ok(HttpResponse::Ok().body(json_result))
}
