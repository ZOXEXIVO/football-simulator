use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::Country;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CountryGetRequest {
    country_slug: String,
}

#[derive(Template)]
#[template(path = "countries/get/get.html")]
pub struct CountryGetViewModel<'c> {
    pub slug: &'c str,
    pub name: &'c str,
    pub code: &'c str,
    pub continent_name: &'c str,
    pub leagues: Vec<LeagueDto<'c>>,
}

pub struct LeagueDto<'l> {
    pub slug: &'l str,
    pub name: &'l str,
}

pub async fn country_get_action(
    state: Data<GameAppData>,
    route_params: web::Path<CountryGetRequest>,
) -> Result<HttpResponse> {
    let guard = state.data.lock().await;

    let simulator_data = guard.as_ref().unwrap();

    let country_id = simulator_data
        .indexes
        .as_ref()
        .unwrap()
        .slug_indexes
        .get_country_by_slug(&route_params.country_slug)
        .unwrap();

    let country: &Country = simulator_data
        .continents
        .iter()
        .flat_map(|c| &c.countries)
        .find(|country| country.id == country_id)
        .unwrap();

    let continent = simulator_data.continent(country.continent_id).unwrap();

    let model = CountryGetViewModel {
        slug: &country.slug,
        name: &country.name,
        code: &country.code,
        continent_name: &continent.name,
        leagues: country
            .leagues
            .leagues
            .iter()
            .map(|l| LeagueDto {
                slug: &l.slug,
                name: &l.name,
            })
            .collect(),
    };

    let html = CountryGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
