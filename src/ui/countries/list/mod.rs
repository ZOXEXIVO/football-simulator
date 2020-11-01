use actix_web::{HttpResponse, Result};
use actix_web::web::Data;
use crate::GameAppData;
use askama::Template;

#[derive(Template)]
#[template(path = "countries/list/list.html")]
pub struct CountryListViewModel<'c> {
    pub continents: Vec<ContinentDto<'c>>
}

pub struct ContinentDto<'c> {
    pub name: &'c str,
    pub countries: Vec<CountryDto<'c>>
}

pub struct CountryDto<'c> {
    pub id: u32,
    pub code: &'c str,
    pub name: &'c str,
    pub leagues: Vec<LeagueDto<'c>>
}

pub struct LeagueDto<'l> {
    pub id: u32,
    pub name: &'l str
}

pub async fn country_list_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let guard = state.data.lock();
    
    let simulator_data = guard.as_ref().unwrap();

    let mut model = CountryListViewModel {
        continents: Vec::with_capacity(simulator_data.continents.len())
    };

    for continent in &simulator_data.continents {
        let item = ContinentDto {
            name: &continent.name,
            countries: continent.countries.iter().map(|country|
                CountryDto {
                    id: country.id,
                    code: &country.code,
                    name: &country.name,
                    leagues: country.leagues.iter().map(|l| LeagueDto{
                        id: l.id,
                        name: &l.name
                    }).collect()
                }
            ).collect()
        };

        model.continents.push(item);
    }

    let html = CountryListViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
